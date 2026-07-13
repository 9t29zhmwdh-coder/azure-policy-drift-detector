use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::auth::AzureClient;
use apdd_core::models::{AzureResource, Scope};

const RESOURCE_GRAPH_URL: &str =
    "https://management.azure.com/providers/Microsoft.ResourceGraph/resources?api-version=2021-03-01";

const DEFAULT_KQL: &str =
    "Resources | project id, name, type, location, subscriptionId, tags | limit 1000";

#[derive(Debug, Serialize)]
struct ResourceGraphRequest<'a> {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    subscriptions: Vec<&'a str>,
    /// Resource Graph scans every subscription under these management
    /// groups in one call; no need to enumerate subscriptions ourselves.
    #[serde(rename = "managementGroups", skip_serializing_if = "Vec::is_empty")]
    management_groups: Vec<&'a str>,
    query: &'a str,
    options: ResourceGraphOptions,
}

#[derive(Debug, Serialize)]
struct ResourceGraphOptions {
    #[serde(rename = "$top")]
    top: u32,
    #[serde(rename = "$skip")]
    skip: u32,
}

#[derive(Debug, Deserialize)]
struct ResourceGraphResponse {
    data: ResourceGraphData,
    #[serde(rename = "$skipToken")]
    skip_token: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ResourceGraphData {
    rows: Vec<Vec<serde_json::Value>>,
    columns: Vec<ResourceGraphColumn>,
}

#[derive(Debug, Deserialize)]
struct ResourceGraphColumn {
    name: String,
}

pub async fn query_resources(
    client: &AzureClient,
    kql: Option<&str>,
    scope: &Scope,
) -> Result<Vec<AzureResource>> {
    let query = kql.unwrap_or(DEFAULT_KQL);
    let (subscriptions, management_groups): (Vec<&str>, Vec<&str>) = match scope {
        Scope::Subscription(id) => (vec![id.as_str()], vec![]),
        Scope::ManagementGroup(id) => (vec![], vec![id.as_str()]),
    };
    let mut all_resources = vec![];
    let mut skip = 0u32;

    loop {
        let request = ResourceGraphRequest {
            subscriptions: subscriptions.clone(),
            management_groups: management_groups.clone(),
            query,
            options: ResourceGraphOptions { top: 1000, skip },
        };

        let response: ResourceGraphResponse = client
            .post_json(RESOURCE_GRAPH_URL, &request)
            .await?;

        let columns: Vec<&str> = response.data.columns.iter().map(|c| c.name.as_str()).collect();

        for row in &response.data.rows {
            let mut map: HashMap<&str, &serde_json::Value> = HashMap::new();
            for (i, col) in columns.iter().enumerate() {
                if let Some(val) = row.get(i) {
                    map.insert(col, val);
                }
            }

            let tags = map
                .get("tags")
                .and_then(|v| v.as_object())
                .map(|obj| {
                    obj.iter()
                        .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                        .collect()
                })
                .unwrap_or_default();

            all_resources.push(AzureResource {
                id: map.get("id").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
                name: map.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
                resource_type: map.get("type").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
                location: map.get("location").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
                subscription_id: map.get("subscriptionId").and_then(|v| v.as_str()).unwrap_or_default().to_string(),
                tags,
            });
        }

        if response.skip_token.is_none() || response.data.rows.is_empty() {
            break;
        }
        skip += 1000;
    }

    Ok(all_resources)
}
