# Security Advisory Process

## Reporting a vulnerability

Email `security@ancora.dev` with:

- A clear description of the vulnerability.
- Steps to reproduce.
- The affected version(s).
- Whether you have a proposed fix.

Do NOT open a public GitHub issue for security vulnerabilities.

## Response timeline

| Step | SLA |
|------|-----|
| Acknowledge receipt | 24 hours |
| Initial assessment | 72 hours |
| Fix or workaround | 14 days for critical, 30 days for medium |
| Public disclosure | After fix is released and embargo lifted |

## Severity levels

| Level | CVSS range | Examples |
|-------|-----------|---------|
| Critical | 9.0-10.0 | RCE, credential exfiltration |
| High | 7.0-8.9 | Privilege escalation, prompt injection with exfil |
| Medium | 4.0-6.9 | Policy bypass, local DoS |
| Low | 0.1-3.9 | Information disclosure, low-impact bypass |

## CVE assignment

We request CVEs through GitHub Security Advisories (GHSA) for vulnerabilities
that meet the threshold. We will credit reporters by name or handle as they prefer.

## No live keys policy

No API keys, secrets, or credentials are ever committed to this repository.
The `sec_no_live_keys.rs` test enforces this in CI by scanning for known prefixes.
