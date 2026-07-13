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

- [x] Management Group scope support (`--management-group`, scans every subscription under it in one run, with a per-subscription breakdown in the report)
- [ ] SARIF export for GitHub Advanced Security (the `results` array is still an empty stub, see `report::to_sarif_stub`)
- [ ] Custom KQL query support via `--query` flag
- [ ] HTML report template

## v0.3.0

- [x] Azure Lighthouse multi-tenant support (`--subscriptions`, scans an explicit list of subscriptions delegated from any number of customer tenants with a single client-credentials token, moved forward from v1.0.0 below)
- [ ] Drift comparison mode (diff between two report snapshots)
- [ ] Azure Monitor integration: push drift alerts to Log Analytics or Action Groups
- [ ] Azure Policy initiative (regulatory compliance) breakdown
- [ ] Policy assignment details and definition parameter inspection
- [ ] Threshold-based exit codes for CI/CD pipeline integration

## v1.0.0

- Stable CLI interface
- Full benchmark against Microsoft Cloud Security Benchmark (MCSB) policy set
- Microsoft Defender for Cloud integration (export findings as security alerts)

## Dual-Licensing Readiness

Assessed 2026-07-11 as a Dual-Licensing candidate (Community MIT + Commercial/Enterprise tier): cloud governance and compliance tooling is one of the most established commercial categories (Wiz, Prisma Cloud and native Azure Policy add-ons all charge for this exact problem space), and APDD's own roadmap already lists several classic enterprise differentiators.

**Update 2026-07-13:** both explicit gating blockers have landed (Management Group scope in v0.2.0, Azure Lighthouse multi-tenant support in v0.3.0, both staying Community/MIT since they're core scanning capability, not an Enterprise differentiator by themselves). This repo is ready for its Enterprise companion: candidate Enterprise-only features are SARIF/Defender for Cloud export, Azure Monitor push integration, and full MCSB benchmarking, with the core drift detection engine (Resource Graph query, Policy Insights retrieval, Management Group/Lighthouse scope, risk prioritization, CLI) staying Community/MIT. A server/API/persistence layer to gate a Commercial tier against doesn't exist yet either; that's expected to land alongside whichever Enterprise feature needs it first, not as a separate prerequisite.
