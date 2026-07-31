# Changelog

## [1.0.8] - 2026-07-31

### Fixed

- CI checked Linux and Windows but not macOS, while the release workflow builds and publishes a macOS binary. That artefact went out without ever having been compile-checked, so a fault appearing only on macOS would have surfaced in somebody's download rather than in a pull request. The `check` matrix covers all three platforms the release targets.

---

## [1.0.7] - 2026-07-31

### Fixed

- The supported-versions table in `SECURITY.md` still listed `0.1.x`, a release line that no longer exists. Somebody reporting a vulnerability reads that table first, and it told them the current release was out of scope. It lists `1.0.x`.

---

## [1.0.6] - 2026-07-31

### Changed

- Both READMEs now open with why an unranked compliance list is useless and what this does about it, rather than with the tool's own category. `apdd demo` is shown first so the tool can be judged without Azure credentials, and a short paragraph points people who want drift fixed automatically at Azure Policy remediation tasks instead.

---

## [1.0.5] - 2026-07-29

### Security

- The release workflow no longer grants `contents: write` for its whole run. The permission moves to the one job that publishes the release, and everything else runs with `contents: read`. OpenSSF Scorecard scores the Token-Permissions check 0 out of 10 whenever any workflow holds a top-level write permission, regardless of how little of the run needs it, so this single line was what held the check at zero.

---

## [1.0.4] - 2026-07-29

### Changed

Dependency and workflow updates merged since 1.0.3:

- chore(ci): bump the actions group across 1 directory with 2 updates
- chore(deps): bump the cargo group across 1 directory with 7 updates

---

## [1.0.3] - 2026-07-29

### Changed

- CodeQL moved from GitHub's default setup to an advanced setup with a committed `.github/workflows/codeql.yml`. The default setup decides on its own when to run and skips pull requests that touch no code of a given language, so a dependency pull request changing only `Cargo.lock` reported `skipping` on the required `Analyze` checks and could never be merged. The workflow runs on every pull request regardless of what changed and uses the `security-extended` query suite, which the default setup does not allow choosing. Required checks are unchanged.
- The Cargo group in `.github/dependabot.yml` is limited to `minor` and `patch` updates. Without that limit a major bump lands inside a grouped pull request that reads as routine, which is how a breaking change slips in unreviewed.

---

## [1.0.2] - 2026-07-28

### Added

- `.github/dependabot.yml`, covering GitHub Actions and Cargo with grouped weekly updates. The file was missing, and without it there are no version updates at all: repository security alerts only fire for disclosed vulnerabilities. Follows `engineering-standards` v0.10.0.

### Fixed

- 8 action references were pinned to a mutable tag or branch rather than a commit SHA, `dtolnay/rust-toolchain@stable` among them. A branch HEAD can be moved to point at different code at any time without the workflow file changing, which is exactly what `standards/ci-cd.md` section 2 exists to prevent. All are now pinned to SHAs with the version in the comment. Pinned at their current versions, not upgraded: a major bump belongs in its own reviewed PR, and Dependabot will now propose one.
- `actions/checkout` pins were inconsistent across workflows. All now use v7.0.1 with the full version in the comment, per `standards/ci-cd.md` section 2.

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
