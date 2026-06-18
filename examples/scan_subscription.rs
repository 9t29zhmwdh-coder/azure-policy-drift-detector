//! Example: scan a subscription and print all drift findings.
//!
//! Set AZURE_TENANT_ID, AZURE_CLIENT_ID, AZURE_CLIENT_SECRET, AZURE_SUBSCRIPTION_ID.
//! cargo run --example scan_subscription

use anyhow::Result;
use apdd_azure::{auth::AzureClient, policy_insights, resource_graph};
use apdd_core::analyzer;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let client = AzureClient::from_env()?;
    let subscription_id = client.subscription_id.clone();

    println!("Scanning subscription {}...", subscription_id);

    let resources = resource_graph::query_resources(&client, None).await?;
    let policy_states = policy_insights::query_policy_states(&client).await?;

    println!(
        "Resources: {}  Policy states: {}",
        resources.len(),
        policy_states.len()
    );

    let report = analyzer::build_report(subscription_id, &resources, &policy_states);

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
