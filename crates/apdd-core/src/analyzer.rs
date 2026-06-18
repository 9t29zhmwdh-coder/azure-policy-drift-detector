use std::collections::HashMap;

use chrono::Utc;

use crate::models::{
    AzureResource, ComplianceReport, ComplianceSummary, ComplianceState, DriftResult, DriftType,
    PolicyState, Severity,
};

const SECURITY_POLICY_KEYWORDS: &[&str] = &[
    "security", "encryption", "tls", "https", "network", "firewall",
    "defender", "sentinel", "key vault", "mfa", "identity",
];

const COMPLIANCE_POLICY_KEYWORDS: &[&str] = &[
    "iso", "nist", "cis", "gdpr", "hipaa", "pci", "sox", "audit",
];

fn classify_severity(policy_name: &str, drift_type: &DriftType) -> Severity {
    let name_lower = policy_name.to_lowercase();

    if SECURITY_POLICY_KEYWORDS.iter().any(|k| name_lower.contains(k)) {
        return Severity::Critical;
    }

    if COMPLIANCE_POLICY_KEYWORDS.iter().any(|k| name_lower.contains(k)) {
        return Severity::High;
    }

    match drift_type {
        DriftType::NonCompliantConfiguration => Severity::High,
        DriftType::TagMismatch => Severity::Medium,
        DriftType::MissingRequiredTag => Severity::Medium,
        DriftType::PolicyExempt => Severity::Informational,
    }
}

pub fn detect_drift(
    resources: &[AzureResource],
    policy_states: &[PolicyState],
) -> Vec<DriftResult> {
    let resource_map: HashMap<&str, &AzureResource> =
        resources.iter().map(|r| (r.id.as_str(), r)).collect();

    let mut drifts = vec![];

    for state in policy_states {
        if state.compliance_state == ComplianceState::Compliant {
            continue;
        }

        let resource = match resource_map.get(state.resource_id.as_str()) {
            Some(r) => r,
            None => continue,
        };

        let (drift_type, description, remediation) = match state.compliance_state {
            ComplianceState::NonCompliant => {
                let policy_lower = state.policy_definition_name.to_lowercase();
                if policy_lower.contains("tag") {
                    (
                        DriftType::TagMismatch,
                        format!(
                            "Resource '{}' violates tagging policy '{}'.",
                            resource.name, state.policy_definition_name
                        ),
                        "Apply the required tags as defined in the policy assignment. Use 'az tag update' or the Azure Portal.".to_string(),
                    )
                } else {
                    (
                        DriftType::NonCompliantConfiguration,
                        format!(
                            "Resource '{}' is non-compliant with policy '{}'.",
                            resource.name, state.policy_definition_name
                        ),
                        format!(
                            "Review the policy definition '{}' and bring the resource configuration into compliance.",
                            state.policy_definition_name
                        ),
                    )
                }
            }
            ComplianceState::Exempt => (
                DriftType::PolicyExempt,
                format!(
                    "Resource '{}' is exempt from policy '{}'.",
                    resource.name, state.policy_definition_name
                ),
                "Review the exemption justification and expiry date. Ensure the exemption is still valid.".to_string(),
            ),
            _ => continue,
        };

        let severity = classify_severity(&state.policy_definition_name, &drift_type);

        drifts.push(DriftResult {
            resource_id: resource.id.clone(),
            resource_name: resource.name.clone(),
            resource_type: resource.resource_type.clone(),
            location: resource.location.clone(),
            drift_type,
            severity,
            policy_name: state.policy_definition_name.clone(),
            description,
            remediation,
        });
    }

    drifts
}

pub fn prioritize_by_risk(mut drifts: Vec<DriftResult>) -> Vec<DriftResult> {
    drifts.sort_by_key(|d| match d.severity {
        Severity::Critical => 0u8,
        Severity::High => 1,
        Severity::Medium => 2,
        Severity::Low => 3,
        Severity::Informational => 4,
    });
    drifts
}

pub fn build_report(
    subscription_id: String,
    resources: &[AzureResource],
    policy_states: &[PolicyState],
) -> ComplianceReport {
    let drifts = prioritize_by_risk(detect_drift(resources, policy_states));

    let compliant_count = policy_states
        .iter()
        .filter(|s| s.compliance_state == ComplianceState::Compliant)
        .count();

    let non_compliant_count = policy_states
        .iter()
        .filter(|s| s.compliance_state == ComplianceState::NonCompliant)
        .count();

    let exempt_count = policy_states
        .iter()
        .filter(|s| s.compliance_state == ComplianceState::Exempt)
        .count();

    let summary = ComplianceSummary::from_drifts(&drifts);

    ComplianceReport {
        subscription_id,
        scanned_at: Utc::now(),
        total_resources: resources.len(),
        compliant_count,
        non_compliant_count,
        exempt_count,
        drifts,
        summary,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_resource(id: &str, name: &str) -> AzureResource {
        AzureResource {
            id: id.to_string(),
            name: name.to_string(),
            resource_type: "microsoft.compute/virtualmachines".to_string(),
            location: "westeurope".to_string(),
            subscription_id: "sub-001".to_string(),
            tags: Default::default(),
        }
    }

    fn make_policy_state(resource_id: &str, policy: &str, state: ComplianceState) -> PolicyState {
        PolicyState {
            resource_id: resource_id.to_string(),
            policy_assignment_id: format!("/assignments/{}", policy),
            policy_assignment_name: policy.to_string(),
            policy_definition_id: format!("/definitions/{}", policy),
            policy_definition_name: policy.to_string(),
            compliance_state: state,
            timestamp: Utc::now(),
        }
    }

    #[test]
    fn test_non_compliant_creates_drift() {
        let resources = vec![make_resource("/subscriptions/sub/res/vm1", "vm1")];
        let states = vec![make_policy_state(
            "/subscriptions/sub/res/vm1",
            "Require encryption",
            ComplianceState::NonCompliant,
        )];
        let drifts = detect_drift(&resources, &states);
        assert_eq!(drifts.len(), 1);
        assert_eq!(drifts[0].resource_name, "vm1");
    }

    #[test]
    fn test_compliant_no_drift() {
        let resources = vec![make_resource("/subscriptions/sub/res/vm2", "vm2")];
        let states = vec![make_policy_state(
            "/subscriptions/sub/res/vm2",
            "Require TLS",
            ComplianceState::Compliant,
        )];
        let drifts = detect_drift(&resources, &states);
        assert!(drifts.is_empty());
    }

    #[test]
    fn test_security_policy_critical_severity() {
        let resources = vec![make_resource("/res/vm3", "vm3")];
        let states = vec![make_policy_state(
            "/res/vm3",
            "Enable network security",
            ComplianceState::NonCompliant,
        )];
        let drifts = detect_drift(&resources, &states);
        assert_eq!(drifts[0].severity, Severity::Critical);
    }

    #[test]
    fn test_prioritize_critical_first() {
        let drifts = vec![
            DriftResult {
                resource_id: "r1".into(), resource_name: "a".into(),
                resource_type: "t".into(), location: "l".into(),
                drift_type: DriftType::TagMismatch, severity: Severity::Medium,
                policy_name: "p".into(), description: "d".into(), remediation: "r".into(),
            },
            DriftResult {
                resource_id: "r2".into(), resource_name: "b".into(),
                resource_type: "t".into(), location: "l".into(),
                drift_type: DriftType::NonCompliantConfiguration, severity: Severity::Critical,
                policy_name: "p".into(), description: "d".into(), remediation: "r".into(),
            },
        ];
        let sorted = prioritize_by_risk(drifts);
        assert_eq!(sorted[0].severity, Severity::Critical);
    }
}
