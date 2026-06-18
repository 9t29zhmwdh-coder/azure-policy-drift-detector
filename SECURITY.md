# Security Policy

## Supported Versions

| Version | Supported |
|---|---|
| 0.1.x | Yes |

## Reporting a Vulnerability

Open a GitHub issue with the label `security`. Do not include subscription IDs, credentials, or resource data in the report.

I will acknowledge receipt within 72 hours and aim to provide a fix within 14 days for confirmed vulnerabilities.

## Security Design Principles

- **Read-only by design.** The tool uses only read-only Azure RBAC roles. No write operations are performed at any time.
- **Credentials via environment variables only.** No credentials are stored in code, tracked configuration files, or log output.
- **No data exfiltration.** All API responses are processed locally. No data is forwarded to external services.
- **Minimal permission scope.** Only `Reader` and `Policy Insights Data Reader` are required.
- **No persistent storage.** Results are written only to files explicitly specified by the user via `--output`.
