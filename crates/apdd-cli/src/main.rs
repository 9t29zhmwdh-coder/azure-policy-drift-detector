use anyhow::Result;
use clap::{Parser, Subcommand};
use apdd_azure::{auth::AzureClient, policy_insights, resource_graph};
use apdd_core::{analyzer, report};
use tabled::{Table, Tabled};
use tracing::info;

#[derive(Parser)]
#[command(
    name = "apdd",
    version = "0.1.0",
    author = "RayStudio",
    about = "Azure Policy Drift Detector: read-only Azure Policy compliance analysis"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Scan a subscription for policy drift
    Scan {
        /// Show only findings at or above this severity (critical|high|medium|low)
        #[arg(long, default_value = "low")]
        min_severity: String,
    },
    /// Export full compliance report
    Export {
        /// Output format: json, md, or sarif
        #[arg(long, short, default_value = "json")]
        format: String,
        /// Output file path (defaults to stdout)
        #[arg(long, short)]
        output: Option<String>,
    },
    /// Run drift detection against a built-in synthetic subscription, no Azure credentials needed
    Demo,
}

#[derive(Tabled)]
struct DriftRow {
    #[tabled(rename = "Severity")]
    severity: String,
    #[tabled(rename = "Type")]
    drift_type: String,
    #[tabled(rename = "Resource")]
    resource: String,
    #[tabled(rename = "Policy")]
    policy: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("apdd=info".parse()?),
        )
        .init();

    let cli = Cli::parse();

    if matches!(cli.command, Command::Demo) {
        run_demo();
        return Ok(());
    }

    let client = AzureClient::from_env()?;
    let subscription_id = client.subscription_id.clone();

    match cli.command {
        Command::Scan { min_severity } => {
            info!("Scanning subscription {}...", subscription_id);
            let (resources, policy_states) = fetch_all(&client).await?;
            let report = analyzer::build_report(subscription_id, &resources, &policy_states);

            println!("\n=== Azure Policy Drift Detector ===\n");
            println!(
                "Resources scanned: {}  Non-compliant: {}  Exempt: {}  Drift findings: {}\n",
                report.total_resources,
                report.non_compliant_count,
                report.exempt_count,
                report.drifts.len()
            );

            let min_level = severity_level(&min_severity);
            let filtered: Vec<&apdd_core::models::DriftResult> = report
                .drifts
                .iter()
                .filter(|d| severity_level(&format!("{}", d.severity).to_lowercase()) <= min_level)
                .collect();

            if filtered.is_empty() {
                println!("No drift findings at or above '{}' severity.", min_severity);
            } else {
                let rows: Vec<DriftRow> = filtered
                    .iter()
                    .map(|d| DriftRow {
                        severity: d.severity.to_string(),
                        drift_type: d.drift_type.to_string(),
                        resource: truncate(&d.resource_name, 30),
                        policy: truncate(&d.policy_name, 40),
                    })
                    .collect();
                println!("{}", Table::new(rows));
            }

            println!(
                "\nSummary: {} Critical, {} High, {} Medium, {} Low",
                report.summary.critical_count,
                report.summary.high_count,
                report.summary.medium_count,
                report.summary.low_count,
            );
        }

        Command::Export { format, output } => {
            info!("Exporting compliance report for subscription {}...", subscription_id);
            let (resources, policy_states) = fetch_all(&client).await?;
            let compliance_report =
                analyzer::build_report(subscription_id, &resources, &policy_states);

            let content = match format.as_str() {
                "md" => report::to_markdown(&compliance_report),
                "sarif" => report::to_sarif_stub(&compliance_report),
                _ => report::to_json(&compliance_report)?,
            };

            match output {
                Some(path) => std::fs::write(&path, &content)?,
                None => print!("{}", content),
            }
        }

        Command::Demo => unreachable!("handled before AzureClient is created"),
    }

    Ok(())
}

fn run_demo() {
    use apdd_core::models::{AzureResource, ComplianceState, PolicyState};
    use std::collections::HashMap;

    let mut tags_missing_owner = HashMap::new();
    tags_missing_owner.insert("environment".to_string(), "production".to_string());

    let mut tags_ok = HashMap::new();
    tags_ok.insert("environment".to_string(), "production".to_string());
    tags_ok.insert("owner".to_string(), "platform-team".to_string());

    let resources = vec![
        AzureResource {
            id: "/subscriptions/demo/resourceGroups/rg-prod/providers/Microsoft.Storage/storageAccounts/proddata".into(),
            name: "proddata".into(),
            resource_type: "Microsoft.Storage/storageAccounts".into(),
            location: "westeurope".into(),
            subscription_id: "demo-subscription".into(),
            tags: tags_missing_owner,
        },
        AzureResource {
            id: "/subscriptions/demo/resourceGroups/rg-prod/providers/Microsoft.Compute/virtualMachines/vm-web01".into(),
            name: "vm-web01".into(),
            resource_type: "Microsoft.Compute/virtualMachines".into(),
            location: "westeurope".into(),
            subscription_id: "demo-subscription".into(),
            tags: tags_ok,
        },
    ];

    let policy_states = vec![
        PolicyState {
            resource_id: resources[0].id.clone(),
            policy_assignment_id: "assign-require-owner-tag".into(),
            policy_assignment_name: "Require owner tag".into(),
            policy_definition_id: "def-require-owner-tag".into(),
            policy_definition_name: "Require owner tag on resources".into(),
            compliance_state: ComplianceState::NonCompliant,
            timestamp: chrono::Utc::now(),
        },
        PolicyState {
            resource_id: resources[1].id.clone(),
            policy_assignment_id: "assign-allowed-locations".into(),
            policy_assignment_name: "Allowed locations".into(),
            policy_definition_id: "def-allowed-locations".into(),
            policy_definition_name: "Allowed locations for resources".into(),
            compliance_state: ComplianceState::Compliant,
            timestamp: chrono::Utc::now(),
        },
    ];

    let report = analyzer::build_report("demo-subscription".to_string(), &resources, &policy_states);

    println!("\n=== Azure Policy Drift Detector (demo, synthetic data) ===\n");
    println!(
        "Resources scanned: {}  Non-compliant: {}  Exempt: {}  Drift findings: {}\n",
        report.total_resources,
        report.non_compliant_count,
        report.exempt_count,
        report.drifts.len()
    );

    if report.drifts.is_empty() {
        println!("No drift findings.");
    } else {
        let rows: Vec<DriftRow> = report
            .drifts
            .iter()
            .map(|d| DriftRow {
                severity: d.severity.to_string(),
                drift_type: d.drift_type.to_string(),
                resource: truncate(&d.resource_name, 30),
                policy: truncate(&d.policy_name, 40),
            })
            .collect();
        println!("{}", Table::new(rows));
    }

    println!(
        "\nSummary: {} Critical, {} High, {} Medium, {} Low",
        report.summary.critical_count,
        report.summary.high_count,
        report.summary.medium_count,
        report.summary.low_count,
    );
}

async fn fetch_all(
    client: &AzureClient,
) -> Result<(
    Vec<apdd_core::models::AzureResource>,
    Vec<apdd_core::models::PolicyState>,
)> {
    let resources = resource_graph::query_resources(client, None).await?;
    let policy_states = policy_insights::query_policy_states(client).await?;
    Ok((resources, policy_states))
}

fn severity_level(s: &str) -> u8 {
    match s {
        "critical" => 0,
        "high" => 1,
        "medium" => 2,
        "low" => 3,
        _ => 4,
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max.saturating_sub(3)])
    }
}
