/// Rotation record: tracks when a secret was last rotated and by whom.
#[derive(Clone, Debug)]
pub struct RotationRecord {
    pub provider: String,
    pub key: String,
    pub rotated_at_secs: u64,
}

/// Audit log of all rotation events.
#[derive(Default)]
pub struct RotationLog {
    records: Vec<RotationRecord>,
}

impl RotationLog {
    pub fn record(&mut self, provider: impl Into<String>, key: impl Into<String>, at: u64) {
        self.records.push(RotationRecord {
            provider: provider.into(),
            key: key.into(),
            rotated_at_secs: at,
        });
    }

    pub fn history(&self) -> &[RotationRecord] {
        &self.records
    }

    pub fn last_rotation_for(&self, key: &str) -> Option<&RotationRecord> {
        self.records.iter().rev().find(|r| r.key == key)
    }
}
