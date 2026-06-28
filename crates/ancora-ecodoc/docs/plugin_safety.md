# Plugin Safety Model

Ancora enforces capability-based access control on all plugins.

## Trust levels

| Level      | Capabilities                                   |
|------------|------------------------------------------------|
| Untrusted  | None                                           |
| Community  | FileSystemRead                                 |
| Verified   | FileSystemRead, FileSystemWrite, NetworkAccess |
| Core       | All capabilities including SecretAccess        |

## Requesting a trust level

Include a `trust-level = "community"` field in your `ancora-catalog.toml`.
The Ancora catalog team will review and may upgrade or downgrade the level.
