# Enterprise Features

This document lists features planned for the Enterprise Edition of this
project, licensed separately under
[LICENSE.COMMERCIAL](LICENSE.COMMERCIAL). See [COMMERCIAL.md](COMMERCIAL.md)
for the licensing model.

## Status

No Enterprise features have shipped yet. This list is a forward-looking plan,
not a changelog of existing functionality: everything currently in this
repository is part of the Community Edition and remains MIT-licensed. See the
repository's own [ROADMAP.md](ROADMAP.md), "Dual-Licensing Readiness"
section, for the prerequisites that need to land first.

## Planned

- Multi-subscription and Management Group scope: consolidated, cross-
  subscription governance instead of one subscription at a time.
- Multi-tenant support (Azure Lighthouse): scanning multiple customer
  tenants from a single install, for MSPs and consultancies.
- SARIF export and Defender for Cloud integration: turnkey findings feed
  into an organization's existing security and compliance stack.
- Azure Monitor push integration and full MCSB (Microsoft Cloud Security
  Benchmark) benchmarking.

## Not planned

The core drift detection engine (Resource Graph query, Policy Insights
retrieval, risk prioritization, CLI) stays in the Community Edition
permanently. Dual-licensing governs only new, enterprise-shaped capabilities
such as the ones listed above, not the tool's standalone usefulness for a
single subscription.
