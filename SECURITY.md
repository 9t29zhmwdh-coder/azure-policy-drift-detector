# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| Latest  | ✅ Yes    |
| Older   | ❌ No     |

Security fixes are only applied to the latest release.

## Reporting a Vulnerability

**Do NOT open a public GitHub issue for security vulnerabilities.**

Instead, report it privately via [GitHub Security Advisory](https://github.com/9t29zhmwdh-coder/azure-policy-drift-detector/security/advisories/new) or contact the maintainer via the GitHub profile.

Include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

A response within **48 hours** is the target, and the issue will be worked on promptly.

## Security Design Principles

- **Read-only by design.** The tool uses only read-only Azure RBAC roles. No write operations are performed at any time.
- **Credentials via environment variables only.** No credentials are stored in code, tracked configuration files, or log output.
- **No data exfiltration.** All API responses are processed locally. No data is forwarded to external services.
- **Minimal permission scope.** Only `Reader` and `Policy Insights Data Reader` are required.
- **No persistent storage.** Results are written only to files explicitly specified by the user via `--output`.
