# Security Disclosure for Extensions

## Reporting a Vulnerability

If you discover a security vulnerability in an Ancora extension:

1. Do **not** open a public GitHub issue.
2. Email the core security team at security@ancora-project.example with:
   - Extension ID and version affected.
   - Description of the vulnerability and its impact.
   - Steps to reproduce.
   - Any suggested mitigations.
3. The team will acknowledge your report within 48 hours.

## Disclosure Process

Security reports follow the `SecurityDisclosure` lifecycle:

1. **Received**: Report logged and assigned an ID (e.g., `CVE-2026-NNN`).
2. **Triaging**: Team validates the report and assesses severity.
3. **Confirmed**: Vulnerability confirmed; maintainer notified.
4. **Patched**: Fix released; coordinated embargo in effect.
5. **Disclosed**: Public disclosure after embargo (typically 30 days post-patch).

## Severity Levels

- **Critical**: Remote code execution, privilege escalation.
- **High**: Data exposure, denial of service.
- **Medium**: Limited impact, difficult to exploit.
- **Low**: Minimal risk, defense in depth improvement.

## Policy

Extensions with unpatched Critical or High severity vulnerabilities will be
automatically unlisted from the registry after 7 days without a patch.
