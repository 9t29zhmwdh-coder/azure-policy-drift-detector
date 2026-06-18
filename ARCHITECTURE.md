# Architecture

## Overview

```
┌──────────────────────────────────────────────────────────────────┐
│                    apdd-cli (binary: apdd)                        │
│    scan | export                                                  │
└────────────────────┬─────────────────────────────────────────────┘
                     │
          ┌──────────┴──────────┐
          │                     │
┌─────────▼──────────┐  ┌───────▼──────────────────────────────┐
│    apdd-azure       │  │           apdd-core                   │
│                     │  │                                       │
│  AzureClient        │  │  analyzer::detect_drift               │
│    acquire_token()  │  │  analyzer::prioritize_by_risk         │
│    post_json()      │  │  analyzer::build_report               │
│                     │  │                                       │
│  resource_graph::   │  │  report::to_json                      │
│    query_resources  │  │  report::to_markdown                  │
│                     │  │  report::to_sarif_stub                │
│  policy_insights::  │  └───────────────────────────────────────┘
│    query_policy_    │
│    states           │
└─────────┬───────────┘
          │
          │ HTTPS (read-only)
          │
┌─────────▼────────────────────────────────────────────────────┐
│               Azure Management API                             │
│                                                               │
│  POST /providers/Microsoft.ResourceGraph/resources            │
│  POST /subscriptions/{id}/providers/                          │
│       Microsoft.PolicyInsights/policyStates/latest/...        │
└───────────────────────────────────────────────────────────────┘
```

## Data Flow

1. CLI parses command and reads credentials from environment variables
2. `AzureClient::from_env()` constructs the client
3. `acquire_token()` performs OAuth2 client credentials flow
4. `resource_graph::query_resources()` fetches all subscription resources via KQL
5. `policy_insights::query_policy_states()` fetches non-compliant policy states
6. `analyzer::build_report()` correlates resources with policy states and produces drift results
7. Output is rendered as table (stdout), JSON, Markdown, or SARIF

## Crate Responsibilities

| Crate | Responsibility |
|---|---|
| `apdd-core` | Domain types, drift analysis logic, report generation. No I/O, no network. |
| `apdd-azure` | Azure Management API client, OAuth2 token acquisition, Resource Graph and Policy Insights calls. |
| `apdd-cli` | CLI parsing, orchestration, terminal rendering. |

## Security Boundary

All network traffic is outbound HTTPS to Microsoft Azure endpoints only (`login.microsoftonline.com`, `management.azure.com`). The `apdd-core` crate has no network dependencies and can be tested without Azure credentials.
