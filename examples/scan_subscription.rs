//! Example: scan a subscription and print all drift findings.
//!
//! Set AZURE_TENANT_ID, AZURE_CLIENT_ID, AZURE_CLIENT_SECRET, AZURE_SUBSCRIPTION_ID.
//! cargo run --example scan_subscription

use anyhow::{anyhow, Result};
use apdd_azure::{auth::AzureClient, policy_insights, resource_graph};
use apdd_core::{analyzer, models::Scope};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let client = AzureClient::from_env()?;
    let scope = Scope::Subscription(
        client
            .subscription_id
            .clone()
            .ok_or_else(|| anyhow!("AZURE_SUBSCRIPTION_ID not set"))?,
    );

    println!("Scanning {}...", scope.label());

    let resources = resource_graph::query_resources(&client, None, &scope).await?;
    let policy_states = policy_insights::query_policy_states(&client, &scope).await?;

    println!(
        "Resources: {}  Policy states: {}",
        resources.len(),
        policy_states.len()
    );

    let report = analyzer::build_report(&scope, &resources, &policy_states);

    println!(
        "\nFindings: {} Critical, {} High, {} Medium, {} Low",
        report.summary.critical_count,
        report.summary.high_count,
        report.summary.medium_count,
        report.summary.low_count,
    );

    for drift in &report.drifts {
        println!("[{}] {} | {}", drift.severity, drift.resource_name, drift.policy_name);
    }

    Ok(())
}
