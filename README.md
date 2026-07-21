<div align="center">
  <img src="RayStudio.png" alt="RayStudio Logo" width="120"/>

  <h1>Azure Policy Drift Detector</h1>
</div>

[🇩🇪 Deutsche Version](README.de.md)

**Read-only Rust CLI to detect Azure Policy drift across subscriptions, prioritize non-compliant resources and generate remediation reports.**

Azure Policy Drift Detector connects to Azure Resource Graph and Policy Insights using application credentials and compares resource configurations against active policy assignments. Entirely read-only, no data leaves your machine.

Aligned with the [Microsoft Cloud Security Benchmark (MCSB)](https://learn.microsoft.com/en-us/security/benchmark/azure/overview) and designed for Azure Governance and compliance teams.

[![CI](https://github.com/9t29zhmwdh-coder/azure-policy-drift-detector/actions/workflows/ci.yml/badge.svg)](https://github.com/9t29zhmwdh-coder/azure-policy-drift-detector/actions) [![CodeQL](https://github.com/9t29zhmwdh-coder/azure-policy-drift-detector/actions/workflows/github-code-scanning/codeql/badge.svg)](https://github.com/9t29zhmwdh-coder/azure-policy-drift-detector/security/code-scanning) [![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/9t29zhmwdh-coder/azure-policy-drift-detector/badge)](https://securityscorecards.dev/viewer/?uri=github.com/9t29zhmwdh-coder/azure-policy-drift-detector) [![OpenSSF Best Practices](https://www.bestpractices.dev/projects/13717/badge)](https://www.bestpractices.dev/projects/13717)

![Azure Ready](https://img.shields.io/badge/Azure-Ready-0078d4?logo=microsoftazure&logoColor=white) ![Platform](https://img.shields.io/badge/Platform-Windows_%7C_Ubuntu-lightgrey) ![Rust](https://img.shields.io/badge/Rust-CE422B?logo=rust&logoColor=white) ![AI | Claude Code](https://img.shields.io/badge/AI-Claude_Code-black?logo=anthropic&logoColor=white) ![AI | Copilot](https://img.shields.io/badge/AI-Copilot-black?logo=github&logoColor=white) [![Release](https://img.shields.io/github/v/release/9t29zhmwdh-coder/azure-policy-drift-detector?color=3F8E7E)](https://github.com/9t29zhmwdh-coder/azure-policy-drift-detector/releases) [![License](https://img.shields.io/github/license/9t29zhmwdh-coder/azure-policy-drift-detector?color=lightgrey)](LICENSE)

> **How it runs:** This is a command-line tool, not a desktop app and not a server. `apdd` runs once per command and exits; there is no installer and no background process. Run `apdd demo` to see it work against a built-in synthetic subscription, no Azure credentials needed.

![azure-policy-drift-detector](docs/screenshot.png)

---

> 🌱 New here? → [Step-by-step guide for beginners](GETTING_STARTED.md)

---

**In practice:** you get a CLI that connects to your subscription read-only and prints a prioritized list of policy drift findings straight to your terminal, or exports them as JSON/Markdown for tickets and audits.

## Features

| Capability | Description |
|---|---|
| Resource discovery | Queries all resources in a subscription via Azure Resource Graph (KQL) |
| Policy state retrieval | Fetches latest compliance states from Azure Policy Insights |
| Drift detection | Identifies non-compliant configurations, tag mismatches and policy exemptions |
| Risk prioritization | Classifies findings by severity based on policy category (security, compliance, operational) |
| JSON export | Machine-readable compliance report for integration with ticketing systems |
| Markdown export | Human-readable report with findings, descriptions and remediation steps |
| SARIF stub | Prepared for GitHub Advanced Security integration (v0.2) |
| GitHub Action template | Ready-to-use workflow template for scheduled compliance checks |

---

## Required Azure RBAC Roles

Register an application in Entra ID and assign the following roles at subscription scope:

| Role | Purpose |
|---|---|
| `Reader` | Azure Resource Graph queries |
| `Policy Insights Data Reader` | Read policy compliance states |

Both roles are read-only. No write permissions are required or used.

For `--management-group` scans, assign both roles at the **Management Group** scope instead of on individual subscriptions, so they apply to every subscription underneath it.

---

## App Registration Setup

1. Open the [Azure Portal](https://portal.azure.com) and navigate to **Entra ID > App registrations > New registration**
2. Name the application (e.g. `apdd-scanner`) and register
3. Navigate to your **Subscription > Access control (IAM) > Add role assignment**
4. Assign `Reader` and `Policy Insights Data Reader` to the application
5. Go to **Entra ID > App registrations > your app > Certificates and secrets > New client secret**
6. Copy the secret value immediately. It will not be shown again.
7. Note your **Tenant ID**, **Client ID**, **Client Secret** and **Subscription ID**

---

## Requirements

- Rust 1.78+
- Azure subscription with an app registration
- Network access to `login.microsoftonline.com` and `management.azure.com`

---

## Quick Start

```bash
git clone https://github.com/9t29zhmwdh-coder/azure-policy-drift-detector
cd azure-policy-drift-detector
cargo build --release

# Try it without any credentials, against a built-in synthetic subscription
./target/release/apdd demo

cp .env.example .env
# Fill in your credentials for a real subscription

# Scan for drift findings
./target/release/apdd scan

# Show only critical and high severity findings
./target/release/apdd scan --min-severity high

# Export as Markdown report
./target/release/apdd export --format md --output report.md

# Export as JSON
./target/release/apdd export --format json --output report.json

# Scan every subscription under a Management Group in one run
./target/release/apdd scan --management-group mg-contoso-prod

# Azure Lighthouse: scan specific subscriptions delegated from other tenants
./target/release/apdd scan --subscriptions sub-customer-a,sub-customer-b
```

## Management Group Scope

Pass `--management-group <id>` (or set `AZURE_MANAGEMENT_GROUP_ID`) to scan every subscription under that Management Group in a single run, instead of one subscription at a time. `AZURE_SUBSCRIPTION_ID` is not needed in this mode. The report includes a per-subscription breakdown (resource count, non-compliant count, exempt count) alongside the aggregate findings, so a scan across many subscriptions stays readable instead of turning into one undifferentiated list. The service principal needs Reader access at the Management Group scope, not just on individual subscriptions, see [Required Azure RBAC Roles](#required-azure-rbac-roles).

## Azure Lighthouse Multi-Tenant Scanning

Pass `--subscriptions <id1,id2,...>` (or set `AZURE_SUBSCRIPTION_IDS`, comma-separated) to scan an explicit list of subscriptions in one run, instead of a whole Management Group. This is the mode for Azure Lighthouse: once a customer tenant delegates RBAC on a subscription to your managing tenant, the same client-credentials token you already use is authorized against it, no separate login per tenant. Just list the delegated subscription IDs, whichever tenant they actually live in. Reports include the same per-subscription breakdown as a Management Group scan. `--management-group` and `--subscriptions` are mutually exclusive.

---

## Uninstall / Cleanup

Delete the `target/` build directory, your local `.env` file (contains your client secret), and any exported report files (`report.md`, `report.json`). The tool is read-only against Azure and never writes anything back to your subscription.

---

## Configuration

```env
AZURE_TENANT_ID=your-tenant-id
AZURE_CLIENT_ID=your-client-id
AZURE_CLIENT_SECRET=your-client-secret
AZURE_SUBSCRIPTION_ID=your-subscription-id
```

The `.env` file is listed in `.gitignore`. Credentials are never committed.

---

## Drift Severity

| Level | Trigger | Examples |
|---|---|---|
| Critical | Security-relevant policy violation | Network security, encryption, identity |
| High | Compliance framework violation or non-compliant configuration | ISO, NIST, CIS benchmark policies |
| Medium | Tag mismatch or missing required tag | Cost center tag, environment tag |
| Low | Minor configuration deviation | Naming convention |
| Informational | Policy exemption active | Resources with valid exemptions |

---

## GitHub Action Integration

Copy `.github/workflows/policy-check-template.yml` to your repository and configure the required secrets (`AZURE_TENANT_ID`, `AZURE_CLIENT_ID`, `AZURE_CLIENT_SECRET`, `AZURE_SUBSCRIPTION_ID`). The workflow runs weekly and uploads the drift report as a build artifact.

---

## Sample Output

```
=== Azure Policy Drift Detector ===

Resources scanned: 214  Non-compliant: 8  Exempt: 2  Drift findings: 10

+----------+-------------------------------+-------------------+------------------------------------------+
| Severity | Type                          | Resource          | Policy                                   |
+----------+-------------------------------+-------------------+------------------------------------------+
| CRITICAL | Non-Compliant Configuration   | vm-prod-001       | Enable network security group on subnet  |
| HIGH     | Non-Compliant Configuration   | storage-backup    | Require encryption in transit            |
| MEDIUM   | Tag Mismatch                  | aks-cluster-eu    | Require cost center tag                  |
| INFO     | Policy Exempt                 | vm-legacy-002     | Audit unmanaged disks                    |
+----------+-------------------------------+-------------------+------------------------------------------+

Summary: 1 Critical, 3 High, 4 Medium, 0 Low
```

---

**Author:** [Rafael Yilmaz](https://github.com/9t29zhmwdh-coder) · **Status:** Active · ![version](https://img.shields.io/github/v/release/9t29zhmwdh-coder/azure-policy-drift-detector?color=6b7280&style=flat-square) · **License:** MIT
