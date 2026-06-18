# Roadmap

## v0.1.0 (current)

- Azure Resource Graph integration (KQL-based resource discovery)
- Policy Insights compliance state retrieval
- Drift detection for non-compliant configurations, tag mismatches and exemptions
- Risk prioritization by policy category (security, compliance, operational)
- JSON and Markdown export
- SARIF stub for GitHub Advanced Security
- GitHub Action workflow template
- CI on Ubuntu and Windows

## v0.2.0

- SARIF export for GitHub Advanced Security
- Management Group scope support (scan multiple subscriptions)
- Custom KQL query support via `--query` flag
- HTML report template

## v0.3.0

- Drift comparison mode (diff between two report snapshots)
- Azure Policy initiative (regulatory compliance) breakdown
- Policy assignment details and definition parameter inspection
- Threshold-based exit codes for CI/CD pipeline integration

## v1.0.0

- Stable CLI interface
- Benchmark against Microsoft Cloud Security Benchmark (MCSB)
- Azure Lighthouse multi-tenant support
