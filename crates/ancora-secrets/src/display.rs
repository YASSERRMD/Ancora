use crate::secret::{Secret, SecretKind, SecretStatus};
use std::fmt;

impl fmt::Display for SecretStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SecretStatus::Active => write!(f, "active"),
            SecretStatus::Rotated => write!(f, "rotated"),
            SecretStatus::Deleted => write!(f, "deleted"),
            SecretStatus::Expired => write!(f, "expired"),
        }
    }
}

impl fmt::Display for SecretKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SecretKind::Opaque => write!(f, "opaque"),
            SecretKind::DatabaseCredential => write!(f, "database-credential"),
            SecretKind::ApiKey => write!(f, "api-key"),
            SecretKind::TlsCertificate => write!(f, "tls-certificate"),
            SecretKind::SshKey => write!(f, "ssh-key"),
        }
    }
}

impl fmt::Display for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Secret[tenant={} path={} kind={} version={} versions={}]",
            self.tenant_id,
            self.path,
            self.kind,
            self.active_version,
            self.version_count()
        )
    }
}
