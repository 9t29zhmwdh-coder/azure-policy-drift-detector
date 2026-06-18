# Privacy Statement

## Data Handling

Azure Policy Drift Detector is a **fully local, read-only tool**:

- **No data leaves your machine.** API responses from Azure are processed in memory and never forwarded to any external service.
- **No telemetry.** The tool does not collect, transmit, or store usage data of any kind.
- **No persistent storage by default.** Reports are only written to disk when the user explicitly uses `--output`.
- **Credentials are not logged.** Client secrets and tokens are never written to stdout, stderr, or log files.

## Data Accessed via Azure APIs

| Data type | Purpose |
|---|---|
| Resource IDs, names, types, locations | Drift correlation |
| Resource tags | Tag compliance analysis |
| Policy assignment IDs and names | Identifying violated policies |
| Policy compliance states | Core drift detection input |

This data is processed in memory for the duration of the scan and discarded afterwards unless written to an output file by the user.

## Compliance Considerations

When exporting reports containing resource IDs or subscription data, treat the output as confidential internal data in accordance with your organization's data classification policy.
