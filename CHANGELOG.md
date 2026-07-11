# Changelog

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
