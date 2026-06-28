# Security for Extensions

All Ancora plugins must comply with the following security requirements.

## Mandatory requirements

| ID      | Requirement                                             |
|---------|---------------------------------------------------------|
| sec-001 | Never log or expose secret values in plain text         |
| sec-002 | Validate and sanitize all external inputs               |
| sec-003 | Pin and audit transitive dependencies (`cargo audit`)   |
| sec-004 | Report vulnerabilities to security@ancora.dev first     |

## Recommended

| ID      | Requirement                                              |
|---------|----------------------------------------------------------|
| sec-005 | Request only the capabilities the plugin genuinely needs |

## Vulnerability disclosure

Ancora follows a coordinated disclosure model. Please email security@ancora.dev with details.
A fix will be prepared and a CVE will be issued before public disclosure.
