use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    pub subscription_id: String,
    pub scanned_at: DateTime<Utc>,
    pub total_resources: usize,
    pub compliant_count: usize,
    pub non_compliant_count: usize,
    pub exempt_count: usize,
    pub drifts: Vec<DriftResult>,
    pub summary: ComplianceSummary,
}
