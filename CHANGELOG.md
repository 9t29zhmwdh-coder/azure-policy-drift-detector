# Changelog

## [0.1.0] (2026-06-18)

### Added

- Azure Resource Graph integration for resource discovery (KQL)
- Azure Policy Insights integration for compliance state retrieval
- Drift detection for non-compliant configurations, tag mismatches and policy exemptions
- Risk prioritization by policy category
- JSON export via `apdd export --format json`
- Markdown export via `apdd export --format md`
- SARIF stub for future GitHub Advanced Security integration
- GitHub Action workflow template at `.github/workflows/policy-check-template.yml`
- CI pipeline on Ubuntu and Windows
