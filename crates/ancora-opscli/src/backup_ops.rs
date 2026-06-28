use serde::{Deserialize, Serialize};

/// Simulated backup produced by the CLI backup command.
#[derive(Debug, Serialize, Deserialize)]
pub struct CliBackup {
    pub id: String,
    pub tenant_id: String,
    pub kind: String,
    pub size_bytes: u64,
    pub created_at_secs: u64,
    pub checksum: String,
}

pub struct BackupOps {
    backups: Vec<CliBackup>,
}

impl BackupOps {
    pub fn new() -> Self {
        Self { backups: Vec::new() }
    }

    pub fn create_backup(&mut self, tenant_id: &str, now: u64) -> &CliBackup {
        let id = format!("bkp-{}-{}", tenant_id, now);
        let checksum = format!("{:016x}", now ^ 0xdeadbeef);
        self.backups.push(CliBackup {
            id: id.clone(),
            tenant_id: tenant_id.into(),
            kind: "snapshot".into(),
            size_bytes: 1024,
            created_at_secs: now,
            checksum,
        });
        self.backups.last().unwrap()
    }

    pub fn list(&self) -> &[CliBackup] {
        &self.backups
    }
}

impl Default for BackupOps {
    fn default() -> Self {
        Self::new()
    }
}
