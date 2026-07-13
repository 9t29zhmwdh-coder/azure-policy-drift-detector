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

- Drift comparison mode (diff between two report snapshots)
- Azure Monitor integration: push drift alerts to Log Analytics or Action Groups
- Azure Policy initiative (regulatory compliance) breakdown
- Policy assignment details and definition parameter inspection
- Threshold-based exit codes for CI/CD pipeline integration

## v1.0.0

- Stable CLI interface
- Full benchmark against Microsoft Cloud Security Benchmark (MCSB) policy set
- Microsoft Defender for Cloud integration (export findings as security alerts)
- Azure Lighthouse multi-tenant support

## Dual-Licensing Readiness

Assessed 2026-07-11 as a Dual-Licensing candidate (Community MIT + Commercial/Enterprise tier): cloud governance and compliance tooling is one of the most established commercial categories (Wiz, Prisma Cloud and native Azure Policy add-ons all charge for this exact problem space), and APDD's own roadmap already lists several classic enterprise differentiators. Not ready yet; blocked on:

- [x] Management Group / multi-subscription scope shipped in v0.2.0 (`--management-group`), staying Community/MIT: this is core scanning capability, not an Enterprise differentiator by itself
- [ ] No multi-tenant support yet (Azure Lighthouse, v1.0.0 item above): MSPs and consultancies managing multiple customer tenants are a natural Commercial-tier audience
- [ ] No server or API component to gate a Commercial tier against: today APDD is a pure local CLI with no persistence layer
- [ ] Enterprise-shaped features (SARIF export, Defender for Cloud integration, Azure Monitor push, full MCSB benchmarking) are still only roadmap entries, not implemented

Once Azure Lighthouse multi-tenant support (v1.0.0) lands, revisit: candidate Enterprise-only features would be multi-tenant scanning across customer tenants, SARIF/Defender for Cloud export, Azure Monitor push integration, and full MCSB benchmarking, with the core drift detection engine (Resource Graph query, Policy Insights retrieval, Management Group scope, risk prioritization, CLI) staying Community/MIT.
