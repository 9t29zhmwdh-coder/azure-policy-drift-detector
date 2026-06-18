//! Example: export the compliance report to a Markdown file.
//!
//! cargo run --example export_report -- --output report.md

use anyhow::Result;
use apdd_azure::{auth::AzureClient, policy_insights, resource_graph};
use apdd_core::{analyzer, report};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let output_path = std::env::args().nth(2).unwrap_or_else(|| "report.md".to_string());
    let client = AzureClient::from_env()?;
    let subscription_id = client.subscription_id.clone();

    let resources = resource_graph::query_resources(&client, None).await?;
    let policy_states = policy_insights::query_policy_states(&client).await?;
    let compliance_report = analyzer::build_report(subscription_id, &resources, &policy_states);

    let md = report::to_markdown(&compliance_report);
    std::fs::write(&output_path, md)?;
    println!("Report written to {}", output_path);

    Ok(())
}
