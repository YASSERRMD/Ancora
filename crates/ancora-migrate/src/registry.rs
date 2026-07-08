use crate::error::MigrateError;
use crate::migration::Migration;
use std::collections::BTreeMap;

pub struct MigrationRegistry {
    migrations: BTreeMap<u32, Migration>,
}

impl MigrationRegistry {
    pub fn new() -> Self {
        Self {
            migrations: BTreeMap::new(),
        }
    }

    pub fn register(&mut self, m: Migration) {
        self.migrations.insert(m.version, m);
    }

    pub fn get(&self, version: u32) -> Option<&Migration> {
        self.migrations.get(&version)
    }

    pub fn versions_asc(&self) -> Vec<u32> {
        self.migrations.keys().copied().collect()
    }

    pub fn count(&self) -> usize {
        self.migrations.len()
    }

    pub fn validate_sequence(&self) -> Result<(), MigrateError> {
        let versions = self.versions_asc();
        for (i, &v) in versions.iter().enumerate() {
            let expected = (i as u32) + 1;
            if v != expected {
                return Err(MigrateError::VersionGap { expected, got: v });
            }
        }
        Ok(())
    }
}

impl Default for MigrationRegistry {
    fn default() -> Self {
        Self::new()
    }
}
