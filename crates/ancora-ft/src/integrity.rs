//! Adapter integrity verification using simulated checksums.

use crate::model::{AdapterDescriptor, AdapterIntegrity};
use crate::runtime::{FtError, FtResult};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Compute a deterministic pseudo-checksum for a descriptor path.
/// In production this would be a real SHA-256 of the file on disk.
pub fn compute_pseudo_sha256(descriptor: &AdapterDescriptor) -> String {
    let mut hasher = DefaultHasher::new();
    descriptor.path.hash(&mut hasher);
    descriptor.id.as_str().hash(&mut hasher);
    format!(
        "{:016x}{:016x}{:016x}{:016x}",
        hasher.finish(),
        hasher.finish(),
        hasher.finish(),
        hasher.finish()
    )
}

/// Attach integrity metadata to a descriptor (modifies in place).
pub fn attach_integrity(descriptor: &mut AdapterDescriptor, size_bytes: u64) {
    let sha256 = compute_pseudo_sha256(descriptor);
    descriptor.integrity = Some(AdapterIntegrity { sha256, size_bytes });
}

/// Verify a descriptor's integrity against its stored checksum.
///
/// Returns Ok(()) if the checksum matches, Err if it differs.
pub fn verify_integrity(descriptor: &AdapterDescriptor) -> FtResult<()> {
    match &descriptor.integrity {
        None => Err(FtError::IntegrityFailure(
            "no integrity metadata present".into(),
        )),
        Some(stored) => {
            let computed = compute_pseudo_sha256(descriptor);
            if computed == stored.sha256 {
                Ok(())
            } else {
                Err(FtError::IntegrityFailure(format!(
                    "checksum mismatch: stored={}, computed={}",
                    stored.sha256, computed
                )))
            }
        }
    }
}

/// Verify all adapters in a slice; returns the first error if any fail.
pub fn verify_all(descriptors: &[AdapterDescriptor]) -> FtResult<()> {
    for desc in descriptors {
        verify_integrity(desc)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::AdapterDescriptor;
    use std::path::PathBuf;

    fn make_desc(id: &str) -> AdapterDescriptor {
        AdapterDescriptor::new(
            id,
            "Integrity Test",
            "llama-3.1-8b",
            PathBuf::from(format!("/tmp/{}.safetensors", id)),
        )
    }

    #[test]
    fn integrity_attach_and_verify() {
        let mut desc = make_desc("a1");
        attach_integrity(&mut desc, 2048);
        assert!(verify_integrity(&desc).is_ok());
    }

    #[test]
    fn integrity_verify_without_metadata_fails() {
        let desc = make_desc("a1");
        let err = verify_integrity(&desc).unwrap_err();
        assert!(matches!(err, FtError::IntegrityFailure(_)));
    }

    #[test]
    fn integrity_tampered_checksum_fails() {
        let mut desc = make_desc("a1");
        attach_integrity(&mut desc, 2048);
        // Tamper the checksum.
        if let Some(ref mut int) = desc.integrity {
            int.sha256 = "deadbeefdeadbeef".into();
        }
        let err = verify_integrity(&desc).unwrap_err();
        assert!(matches!(err, FtError::IntegrityFailure(_)));
    }

    #[test]
    fn integrity_verify_all_passes() {
        let mut descs: Vec<AdapterDescriptor> =
            (0..3).map(|i| make_desc(&format!("a{}", i))).collect();
        for d in &mut descs {
            attach_integrity(d, 512);
        }
        assert!(verify_all(&descs).is_ok());
    }

    #[test]
    fn integrity_verify_all_fails_on_bad_entry() {
        let mut descs: Vec<AdapterDescriptor> =
            (0..3).map(|i| make_desc(&format!("a{}", i))).collect();
        for d in &mut descs {
            attach_integrity(d, 512);
        }
        // Tamper the second entry.
        if let Some(ref mut int) = descs[1].integrity {
            int.sha256 = "0000".into();
        }
        assert!(verify_all(&descs).is_err());
    }
}
