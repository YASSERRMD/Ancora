use crate::semver::SemVer;
use crate::version_negotiation::{negotiate, CoreApiVersion, ExtensionManifest, NegotiationResult};

/// An entry in the compatibility matrix.
#[derive(Debug, Clone)]
pub struct CompatEntry {
    pub extension_id: String,
    pub extension_version: SemVer,
    pub core_version: SemVer,
    pub result: NegotiationResult,
}

/// A compatibility matrix records the compatibility status for a set of
/// extension-core version combinations.
#[derive(Debug, Default)]
pub struct CompatMatrix {
    entries: Vec<CompatEntry>,
}

impl CompatMatrix {
    pub fn new() -> Self {
        CompatMatrix {
            entries: Vec::new(),
        }
    }

    /// Add an entry by testing the given extension manifest against the core version.
    pub fn record(
        &mut self,
        extension_id: impl Into<String>,
        manifest: &ExtensionManifest,
        core: &CoreApiVersion,
    ) {
        let result = negotiate(manifest, core);
        self.entries.push(CompatEntry {
            extension_id: extension_id.into(),
            extension_version: manifest.min_api_version.clone(),
            core_version: core.version.clone(),
            result,
        });
    }

    /// Returns all entries in the matrix.
    pub fn entries(&self) -> &[CompatEntry] {
        &self.entries
    }

    /// Returns the count of compatible entries.
    pub fn compatible_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| e.result == NegotiationResult::Compatible)
            .count()
    }

    /// Returns the count of incompatible entries.
    pub fn incompatible_count(&self) -> usize {
        self.entries.len() - self.compatible_count()
    }

    /// Generate a simple text report of the matrix.
    pub fn generate_report(&self) -> String {
        let mut lines = vec![
            "Extension Compatibility Matrix".to_string(),
            format!("Total: {}", self.entries.len()),
            format!("Compatible: {}", self.compatible_count()),
            format!("Incompatible: {}", self.incompatible_count()),
            String::new(),
        ];
        for e in &self.entries {
            lines.push(format!(
                "{} (min api {}) vs core {}: {:?}",
                e.extension_id, e.extension_version, e.core_version, e.result
            ));
        }
        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_records_compatible_entry() {
        let mut matrix = CompatMatrix::new();
        let manifest = ExtensionManifest {
            min_api_version: SemVer::new(1, 0, 0),
            max_api_version: SemVer::new(1, 9, 0),
        };
        let core = CoreApiVersion {
            version: SemVer::new(1, 2, 0),
        };
        matrix.record("my-ext", &manifest, &core);
        assert_eq!(matrix.compatible_count(), 1);
        assert_eq!(matrix.incompatible_count(), 0);
    }

    #[test]
    fn report_contains_extension_id() {
        let mut matrix = CompatMatrix::new();
        let manifest = ExtensionManifest {
            min_api_version: SemVer::new(1, 0, 0),
            max_api_version: SemVer::new(1, 9, 0),
        };
        let core = CoreApiVersion {
            version: SemVer::new(1, 1, 0),
        };
        matrix.record("some-ext", &manifest, &core);
        let report = matrix.generate_report();
        assert!(report.contains("some-ext"));
    }
}
