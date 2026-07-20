# Changelog

## [1.0.1] - 2026-07-20

### Changed

- OpenSSF Scorecard workflow and badge.
- `copilot-instructions.md` for consistent AI-assisted contributions.
- Unified the EN/DE language-switch link format and restored missing sections in the German README.
- Split the README's security/CI badges onto their own line, separate from the platform/tech/AI badges (they were rendering as a single merged line).

## [1.0.0] - 2026-07-17

First stable release: a real release pipeline now builds and attaches
`apdd` binaries for Linux, macOS, and Windows to every GitHub Release,
the prerequisite for a 1.0 release per this portfolio's own SemVer
discipline.

### Added
- Release workflow (`release.yml`) that cross-compiles `apdd` for Linux/macOS/Windows on every `v*` tag push and attaches the binaries to a GitHub Release. Previously there was no prebuilt binary; users had to build from source.

## [0.3.1] - 2026-07-17

### Changed
- CI: added an explicit `permissions: contents: read` block to the workflow(s) that were missing one (CodeQL `actions/missing-workflow-permissions`), narrowing the default GITHUB_TOKEN scope.

## [0.3.0] (2026-07-13)

### Added

- Azure Lighthouse multi-tenant support: `apdd scan --subscriptions <id1,id2,...>` (or `AZURE_SUBSCRIPTION_IDS`) scans an explicit list of subscriptions, including ones delegated from other tenants via Lighthouse. A single client-credentials token from the managing tenant already covers every delegated subscription; no per-tenant login step is needed. `--management-group` and `--subscriptions` are mutually exclusive.
- This completes both explicit blockers in the Dual-Licensing Readiness assessment (Management Group scope in 0.2.0, Lighthouse multi-tenant here).

### Fixed

- `--management-group` (and the new `--subscriptions`) now work when passed after the subcommand (`apdd scan --management-group ...`), matching the README's documented usage. Previously, since neither flag was marked `global`, clap only accepted them before the subcommand (`apdd --management-group ... scan`), which was never documented and not what anyone would expect.

## [0.2.0] (2026-07-13)

### Added

- Management Group scope: `apdd scan --management-group <id>` (or `AZURE_MANAGEMENT_GROUP_ID`) scans every subscription under a Management Group in one run, instead of one subscription at a time.
- Per-subscription breakdown on every report (resource count, non-compliant count, exempt count), shown as its own table in Markdown output and included in JSON, so a Management Group scan across many subscriptions stays readable.
- `AzureClient.subscription_id` is now optional: it's only required for a single-subscription scan, not for a Management Group scan.

### Changed

- `ComplianceReport.subscription_id` renamed to `ComplianceReport.scope`, now `"subscription:<id>"` or `"management-group:<id>"`, since a report can now cover more than one subscription.

## [0.1.6] (2026-07-12)

### Added

- Dual-Licensing skeleton: LICENSE.COMMERCIAL, COMMERCIAL.md, and ENTERPRISE_FEATURES.md, documenting the licensing model for a future Enterprise Edition ahead of any actual feature split. The existing MIT LICENSE and all currently released code are unchanged; nothing in this repository is restricted by this addition.

## [0.1.5] (2026-07-11)

### Added

- Documented Dual-Licensing readiness assessment in ROADMAP.md.

## [0.1.4] (2026-07-11)

### Fixed

- Updated actions/checkout, actions/upload-artifact and codecov/codecov-action to their latest major versions in CI, since GitHub is deprecating the Node.js 20 runtime and older action versions were being forced onto Node 24 and crashing during post-run cleanup.

## [0.1.3] (2026-07-10)

### Fixed

- Changed the language-switch link from a blockquote to plain text

## [0.1.2] (2026-07-10)

### Changed

- Moved the "New here? -> beginners guide" callout in README.md to the top of the file (previously only appeared near Requirements)

### Added

- Added the "New here?" beginner guide callout to README.de.md (was missing)

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
