use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// What a scan covers.
///
/// `Subscriptions` is what makes Azure Lighthouse multi-tenant scanning
/// work: Lighthouse delegates RBAC on customer-tenant subscriptions to a
/// service principal in the managing tenant, so a single client-credentials
/// token from that one tenant is already authorized against every
/// delegated subscription, regardless of which tenant it actually lives
/// in. There is no separate "switch tenant" step; scanning several tenants
/// at once is just passing several subscription IDs (Resource Graph
/// accepts a batch of subscriptions natively; Policy Insights doesn't, so
/// apdd-azure loops over them there).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Scope {
    Subscription(String),
    Subscriptions(Vec<String>),
    ManagementGroup(String),
}

impl Scope {
    /// Human-readable label stored on the report, e.g. "subscription:abc",
    /// "subscriptions:abc,def" or "management-group:contoso-prod".
    pub fn label(&self) -> String {
        match self {
            Scope::Subscription(id) => format!("subscription:{id}"),
            Scope::Subscriptions(ids) => format!("subscriptions:{}", ids.join(",")),
            Scope::ManagementGroup(id) => format!("management-group:{id}"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureResource {
    pub id: String,
    pub name: String,
    pub resource_type: String,
    pub location: String,
    pub subscription_id: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ComplianceState {
    Compliant,
    NonCompliant,
    Exempt,
    Unknown,
}

impl std::fmt::Display for ComplianceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComplianceState::Compliant => write!(f, "Compliant"),
            ComplianceState::NonCompliant => write!(f, "NonCompliant"),
            ComplianceState::Exempt => write!(f, "Exempt"),
            ComplianceState::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyState {
    pub resource_id: String,
    pub policy_assignment_id: String,
    pub policy_assignment_name: String,
    pub policy_definition_id: String,
    pub policy_definition_name: String,
    pub compliance_state: ComplianceState,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DriftType {
    NonCompliantConfiguration,
    TagMismatch,
    MissingRequiredTag,
    PolicyExempt,
}

impl std::fmt::Display for DriftType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DriftType::NonCompliantConfiguration => write!(f, "Non-Compliant Configuration"),
            DriftType::TagMismatch => write!(f, "Tag Mismatch"),
            DriftType::MissingRequiredTag => write!(f, "Missing Required Tag"),
            DriftType::PolicyExempt => write!(f, "Policy Exempt"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Informational,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Critical => write!(f, "CRITICAL"),
            Severity::High => write!(f, "HIGH"),
            Severity::Medium => write!(f, "MEDIUM"),
            Severity::Low => write!(f, "LOW"),
            Severity::Informational => write!(f, "INFO"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftResult {
    pub resource_id: String,
    pub resource_name: String,
    pub resource_type: String,
    pub location: String,
    pub drift_type: DriftType,
    pub severity: Severity,
    pub policy_name: String,
    pub description: String,
    pub remediation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComplianceSummary {
    pub critical_count: usize,
    pub high_count: usize,
    pub medium_count: usize,
    pub low_count: usize,
    pub informational_count: usize,
    pub non_compliant_resources: usize,
    pub exempt_resources: usize,
}

impl ComplianceSummary {
    pub fn from_drifts(drifts: &[DriftResult]) -> Self {
        Self {
            critical_count: drifts.iter().filter(|d| d.severity == Severity::Critical).count(),
            high_count: drifts.iter().filter(|d| d.severity == Severity::High).count(),
            medium_count: drifts.iter().filter(|d| d.severity == Severity::Medium).count(),
            low_count: drifts.iter().filter(|d| d.severity == Severity::Low).count(),
            informational_count: drifts
                .iter()
                .filter(|d| d.severity == Severity::Informational)
                .count(),
            non_compliant_resources: drifts
                .iter()
                .filter(|d| d.drift_type != DriftType::PolicyExempt)
                .count(),
            exempt_resources: drifts
                .iter()
                .filter(|d| d.drift_type == DriftType::PolicyExempt)
                .count(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionBreakdown {
    pub subscription_id: String,
    pub total_resources: usize,
    pub non_compliant_count: usize,
    pub exempt_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    /// Was this a single-subscription scan or a Management Group scan;
    /// see `Scope::label`. Kept as a plain string on the report itself so
    /// old JSON exports and this field's shape stay simple to consume.
    pub scope: String,
    pub scanned_at: DateTime<Utc>,
    pub total_resources: usize,
    pub compliant_count: usize,
    pub non_compliant_count: usize,
    pub exempt_count: usize,
    pub drifts: Vec<DriftResult>,
    pub summary: ComplianceSummary,
    /// Per-subscription rollup. Has one entry for a single-subscription
    /// scan too, so callers don't need to special-case scope kinds.
    pub by_subscription: Vec<SubscriptionBreakdown>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_drift(drift_type: DriftType, severity: Severity) -> DriftResult {
        DriftResult {
            resource_id: "/sub/1/res/test".into(),
            resource_name: "test-resource".into(),
            resource_type: "Microsoft.Compute/virtualMachines".into(),
            location: "westeurope".into(),
            drift_type,
            severity,
            policy_name: "Test Policy".into(),
            description: "Test description".into(),
            remediation: "Test remediation".into(),
        }
    }

    #[test]
    fn compliance_summary_counts_correctly() {
        let drifts = vec![
            make_drift(DriftType::NonCompliantConfiguration, Severity::Critical),
            make_drift(DriftType::TagMismatch, Severity::Medium),
            make_drift(DriftType::MissingRequiredTag, Severity::High),
            make_drift(DriftType::PolicyExempt, Severity::Informational),
        ];
        let summary = ComplianceSummary::from_drifts(&drifts);
        assert_eq!(summary.critical_count, 1);
        assert_eq!(summary.high_count, 1);
        assert_eq!(summary.medium_count, 1);
        assert_eq!(summary.informational_count, 1);
        assert_eq!(summary.non_compliant_resources, 3);
        assert_eq!(summary.exempt_resources, 1);
    }

    #[test]
    fn compliance_summary_empty_drifts_returns_zeros() {
        let summary = ComplianceSummary::from_drifts(&[]);
        assert_eq!(summary.critical_count, 0);
        assert_eq!(summary.non_compliant_resources, 0);
    }

    #[test]
    fn compliance_state_display() {
        assert_eq!(ComplianceState::Compliant.to_string(), "Compliant");
        assert_eq!(ComplianceState::NonCompliant.to_string(), "NonCompliant");
        assert_eq!(ComplianceState::Exempt.to_string(), "Exempt");
        assert_eq!(ComplianceState::Unknown.to_string(), "Unknown");
    }

    #[test]
    fn severity_display_uppercase() {
        assert_eq!(Severity::Critical.to_string(), "CRITICAL");
        assert_eq!(Severity::High.to_string(), "HIGH");
        assert_eq!(Severity::Medium.to_string(), "MEDIUM");
        assert_eq!(Severity::Low.to_string(), "LOW");
        assert_eq!(Severity::Informational.to_string(), "INFO");
    }

    #[test]
    fn drift_type_display() {
        assert_eq!(DriftType::NonCompliantConfiguration.to_string(), "Non-Compliant Configuration");
        assert_eq!(DriftType::TagMismatch.to_string(), "Tag Mismatch");
        assert_eq!(DriftType::MissingRequiredTag.to_string(), "Missing Required Tag");
        assert_eq!(DriftType::PolicyExempt.to_string(), "Policy Exempt");
    }

    #[test]
    fn azure_resource_can_be_serialized() {
        let resource = AzureResource {
            id: "/sub/1/res/test".into(),
            name: "test".into(),
            resource_type: "Microsoft.Compute/virtualMachines".into(),
            location: "westeurope".into(),
            subscription_id: "sub-1".into(),
            tags: HashMap::from([("env".into(), "prod".into())]),
        };
        let json = serde_json::to_string(&resource).unwrap();
        let deserialized: AzureResource = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, "test");
        assert_eq!(deserialized.tags["env"], "prod");
    }
}
