//! Integration tests for Azure Policy drift detection.

use apdd_core::{
    analyzer::detect_drift,
    models::{AzureResource, ComplianceSummary, ComplianceState, DriftType, PolicyState, Severity},
};
use chrono::Utc;
use std::collections::HashMap;

fn make_resource(id: &str, name: &str) -> AzureResource {
    AzureResource {
        id: id.into(),
        name: name.into(),
        resource_type: "Microsoft.Compute/virtualMachines".into(),
        location: "westeurope".into(),
        subscription_id: "sub-1".into(),
        tags: HashMap::new(),
    }
}

fn make_policy_state(resource_id: &str, policy_name: &str, state: ComplianceState) -> PolicyState {
    PolicyState {
        resource_id: resource_id.into(),
        policy_assignment_id: "/sub/1/assign/test".into(),
        policy_assignment_name: "Test Assignment".into(),
        policy_definition_id: "/sub/1/def/test".into(),
        policy_definition_name: policy_name.into(),
        compliance_state: state,
        timestamp: Utc::now(),
    }
}

#[test]
fn compliant_resources_produce_no_drift() {
    let resources = vec![make_resource("/sub/1/res/vm1", "vm1")];
    let states = vec![make_policy_state("/sub/1/res/vm1", "Test Policy", ComplianceState::Compliant)];
    let drifts = detect_drift(&resources, &states);
    assert!(drifts.is_empty(), "Compliant resource should produce no drift");
}

#[test]
fn non_compliant_resource_produces_drift() {
    let resources = vec![make_resource("/sub/1/res/vm1", "vm1")];
    let states = vec![make_policy_state("/sub/1/res/vm1", "Test Policy", ComplianceState::NonCompliant)];
    let drifts = detect_drift(&resources, &states);
    assert_eq!(drifts.len(), 1);
    assert_eq!(drifts[0].drift_type, DriftType::NonCompliantConfiguration);
}

#[test]
fn security_policy_gets_critical_severity() {
    let resources = vec![make_resource("/sub/1/res/vm1", "vm1")];
    let states = vec![make_policy_state(
        "/sub/1/res/vm1",
        "require-encryption-at-rest",
        ComplianceState::NonCompliant,
    )];
    let drifts = detect_drift(&resources, &states);
    assert_eq!(drifts.len(), 1);
    assert_eq!(drifts[0].severity, Severity::Critical, "Encryption policy should be Critical");
}

#[test]
fn tag_policy_violation_is_tag_mismatch() {
    let resources = vec![make_resource("/sub/1/res/vm1", "vm1")];
    let states = vec![make_policy_state(
        "/sub/1/res/vm1",
        "require-environment-tag",
        ComplianceState::NonCompliant,
    )];
    let drifts = detect_drift(&resources, &states);
    assert_eq!(drifts.len(), 1);
    assert_eq!(drifts[0].drift_type, DriftType::TagMismatch);
}

#[test]
fn exempt_resource_produces_informational_drift() {
    let resources = vec![make_resource("/sub/1/res/vm1", "vm1")];
    let states = vec![make_policy_state("/sub/1/res/vm1", "Test Policy", ComplianceState::Exempt)];
    let drifts = detect_drift(&resources, &states);
    assert_eq!(drifts.len(), 1);
    assert_eq!(drifts[0].drift_type, DriftType::PolicyExempt);
    assert_eq!(drifts[0].severity, Severity::Informational);
}

#[test]
fn resource_without_matching_policy_state_is_ignored() {
    let resources = vec![make_resource("/sub/1/res/vm1", "vm1")];
    let states = vec![make_policy_state("/sub/1/res/vm-other", "Test Policy", ComplianceState::NonCompliant)];
    let drifts = detect_drift(&resources, &states);
    assert!(drifts.is_empty(), "Unknown resource_id should be ignored");
}

#[test]
fn multiple_non_compliant_resources_all_appear_in_drifts() {
    let resources = vec![
        make_resource("/sub/1/res/vm1", "vm1"),
        make_resource("/sub/1/res/vm2", "vm2"),
        make_resource("/sub/1/res/vm3", "vm3"),
    ];
    let states = vec![
        make_policy_state("/sub/1/res/vm1", "security-policy", ComplianceState::NonCompliant),
        make_policy_state("/sub/1/res/vm2", "tag-policy", ComplianceState::NonCompliant),
        make_policy_state("/sub/1/res/vm3", "Test Policy", ComplianceState::Compliant),
    ];
    let drifts = detect_drift(&resources, &states);
    assert_eq!(drifts.len(), 2);
    let summary = ComplianceSummary::from_drifts(&drifts);
    assert_eq!(summary.non_compliant_resources, 2);
}
