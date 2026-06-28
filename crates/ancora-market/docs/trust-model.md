# Trust Model

The ancora-market trust model assigns a score from 0 to 100 to every extension
published on the marketplace. The score is computed from a set of weighted signals
before the extension is listed, and re-evaluated on each install against the
operator's install policy.

## Score Components

| Signal               | Max Points | Notes                                      |
|----------------------|------------|--------------------------------------------|
| Identity             | 20         | 20 for verified author, 5 for unverified   |
| Security Scan        | 30         | 30 for clean, 0 for critical findings      |
| License Declaration  | 15         | 15 for open-source, 5 for declared-only    |
| Residency Declaration| 15         | 15 for complete declaration                |
| Badges               | 10         | 2 points each, capped at 10               |
| Version History      | 10         | 2 points per version, capped at 10         |

## Trust Tiers

- **High Trust**: score >= 80. Eligible for the HighTrust badge.
- **Acceptable**: score 50-79. Allowed under Warn and Strict policies.
- **Low Trust**: score < 50. Blocked under Strict policy; warned under Warn policy.

## Trust Policy Modes

- **Permissive**: all extensions are installable; trust signals are informational.
- **Warn**: extensions below the configured threshold trigger a warning at install time.
- **Strict**: extensions below the threshold or missing required badges are blocked.

## Enforcement

Trust is enforced at two points:

1. **Publish time**: the registry validates the security scan result and rejects
   extensions with Critical or High severity findings.
2. **Install time**: the `evaluate_policy` function compares the trust score and
   badge set against the operator's `InstallPolicy` and returns a verdict of
   `Allow`, `Warn`, or `Block`.
