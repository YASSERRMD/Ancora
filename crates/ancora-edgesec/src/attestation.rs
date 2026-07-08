use std::collections::HashMap;
use std::fmt;

/// The kind of object being attested.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttestationKind {
    Model,
    Config,
}

impl fmt::Display for AttestationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttestationKind::Model => f.write_str("MODEL"),
            AttestationKind::Config => f.write_str("CONFIG"),
        }
    }
}

/// Result of an attestation check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttestationResult {
    Valid,
    Invalid,
    Missing,
}

impl fmt::Display for AttestationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttestationResult::Valid => f.write_str("VALID"),
            AttestationResult::Invalid => f.write_str("INVALID"),
            AttestationResult::Missing => f.write_str("MISSING"),
        }
    }
}

/// A record attesting to the integrity of a named artifact (model or config).
#[derive(Debug, Clone)]
pub struct AttestationRecord {
    pub artifact_id: String,
    pub kind: AttestationKind,
    pub expected_digest: Vec<u8>,
    pub measured_digest: Vec<u8>,
    pub result: AttestationResult,
    pub tick: u64,
}

impl AttestationRecord {
    /// Create a new attestation record and determine its result.
    pub fn new(
        artifact_id: impl Into<String>,
        kind: AttestationKind,
        expected_digest: Vec<u8>,
        measured_digest: Vec<u8>,
        tick: u64,
    ) -> Self {
        let result = if expected_digest == measured_digest {
            AttestationResult::Valid
        } else {
            AttestationResult::Invalid
        };
        Self {
            artifact_id: artifact_id.into(),
            kind,
            expected_digest,
            measured_digest,
            result,
            tick,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.result == AttestationResult::Valid
    }
}

/// Registry of attested artifacts for an edge device.
pub struct AttestationRegistry {
    records: HashMap<String, AttestationRecord>,
}

impl Default for AttestationRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl AttestationRegistry {
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
        }
    }

    /// Record an attestation for an artifact.
    pub fn attest(
        &mut self,
        artifact_id: impl Into<String>,
        kind: AttestationKind,
        expected_digest: Vec<u8>,
        measured_digest: Vec<u8>,
        tick: u64,
    ) -> &AttestationRecord {
        let id: String = artifact_id.into();
        let record =
            AttestationRecord::new(id.clone(), kind, expected_digest, measured_digest, tick);
        self.records.insert(id.clone(), record);
        self.records.get(&id).unwrap()
    }

    /// Get attestation for an artifact.
    pub fn get(&self, artifact_id: &str) -> Option<&AttestationRecord> {
        self.records.get(artifact_id)
    }

    /// Number of registered attestations.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns true if no attestations are registered.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Returns true if all registered attestations are valid.
    pub fn all_valid(&self) -> bool {
        self.records.values().all(|r| r.is_valid())
    }

    /// Returns a list of invalid artifact ids.
    pub fn invalid_artifacts(&self) -> Vec<&str> {
        self.records
            .values()
            .filter(|r| !r.is_valid())
            .map(|r| r.artifact_id.as_str())
            .collect()
    }
}

impl Default for AttestationRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Attest model integrity for a given device.
pub fn attest_model(
    registry: &mut AttestationRegistry,
    model_id: impl Into<String>,
    expected_digest: Vec<u8>,
    measured_digest: Vec<u8>,
    tick: u64,
) -> &AttestationRecord {
    registry.attest(
        model_id,
        AttestationKind::Model,
        expected_digest,
        measured_digest,
        tick,
    )
}

/// Attest config integrity for a given device.
pub fn attest_config(
    registry: &mut AttestationRegistry,
    config_id: impl Into<String>,
    expected_digest: Vec<u8>,
    measured_digest: Vec<u8>,
    tick: u64,
) -> &AttestationRecord {
    registry.attest(
        config_id,
        AttestationKind::Config,
        expected_digest,
        measured_digest,
        tick,
    )
}
