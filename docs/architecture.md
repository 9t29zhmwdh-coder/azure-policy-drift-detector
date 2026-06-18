# Architecture Reference

See [ARCHITECTURE.md](../ARCHITECTURE.md) for the full overview including data flow diagram and crate responsibilities.

## Azure API Endpoints Used

| Endpoint | Method | Purpose |
|---|---|---|
| `https://management.azure.com/providers/Microsoft.ResourceGraph/resources` | POST | KQL resource queries |
| `https://management.azure.com/subscriptions/{id}/providers/Microsoft.PolicyInsights/policyStates/latest/queryResults` | POST | Non-compliant policy states |
| `https://login.microsoftonline.com/{tenant}/oauth2/v2.0/token` | POST | OAuth2 token acquisition |

All data calls are read-only POST requests with query bodies. No PUT, PATCH, or DELETE requests are made.
