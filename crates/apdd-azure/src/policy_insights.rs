use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::auth::AzureClient;
use apdd_core::models::{ComplianceState, PolicyState};

#[derive(Debug, Serialize)]
struct PolicyQueryBody {
    #[serde(rename = "$filter")]
    filter: String,
    #[serde(rename = "$top")]
    top: u32,
}

#[derive(Debug, Deserialize)]
struct PolicyInsightsResponse {
    value: Vec<RawPolicyState>,
    #[serde(rename = "@odata.nextLink")]
    next_link: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawPolicyState {
    resource_id: Option<String>,
    policy_assignment_id: Option<String>,
    policy_assignment_name: Option<String>,
    policy_definition_id: Option<String>,
    policy_definition_name: Option<String>,
    compliance_state: Option<String>,
}

pub async fn query_policy_states(client: &AzureClient) -> Result<Vec<PolicyState>> {
    let url = client.management_url(&format!(
        "/subscriptions/{}/providers/Microsoft.PolicyInsights/policyStates/latest/queryResults?api-version=2019-10-01",
        client.subscription_id
    ));

    let body = PolicyQueryBody {
        filter: "complianceState ne 'Compliant'".to_string(),
        top: 1000,
    };

    let mut all_states = vec![];
    let mut current_url = url;

    loop {
        let response: PolicyInsightsResponse = client.post_json(&current_url, &body).await?;

        for raw in response.value {
            let compliance_state = match raw.compliance_state.as_deref() {
                Some("NonCompliant") => ComplianceState::NonCompliant,
                Some("Exempt") => ComplianceState::Exempt,
                Some("Compliant") => ComplianceState::Compliant,
                _ => ComplianceState::Unknown,
            };

            all_states.push(PolicyState {
                resource_id: raw.resource_id.unwrap_or_default(),
                policy_assignment_id: raw.policy_assignment_id.unwrap_or_default(),
                policy_assignment_name: raw.policy_assignment_name.unwrap_or_default(),
                policy_definition_id: raw.policy_definition_id.unwrap_or_default(),
                policy_definition_name: raw.policy_definition_name.unwrap_or_default(),
                compliance_state,
                timestamp: Utc::now(),
            });
        }

        match response.next_link {
            Some(link) => current_url = link,
            None => break,
        }
    }

    Ok(all_states)
}
