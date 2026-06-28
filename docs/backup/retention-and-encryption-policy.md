# Backup Retention and Encryption Policy

## Encryption

All backups produced by `BackupEngine::new(key)` are encrypted with the
provided key before being stored. The payload is AES-256-GCM in production
(XOR stream cipher in unit tests for offline coverage).

The key must:
- Be at least 32 bytes.
- Be stored in a secret manager (e.g. Kubernetes Secret, HashiCorp Vault).
- Never be logged, journaled, or committed to source control.

The `manifest.encrypted` field records whether the archive is encrypted so
the restore path knows whether to decrypt.

## Retention schedule

| Backup type | Retention |
|-------------|-----------|
| Full snapshot | 30 days |
| Incremental | 7 days |
| Point-in-time | Keep at least 2 per day for 3 days |

## Key rotation

When rotating the encryption key:
1. Take a new full snapshot encrypted with the new key.
2. Verify the snapshot restores correctly.
3. Delete snapshots encrypted with the old key.
4. Revoke the old key from the secret manager.

## Storage

Archives should be stored in object storage (e.g. S3, GCS) in a bucket with
versioning enabled and server-side encryption active. Cross-region replication
is recommended for DR compliance.
