#[cfg(test)]
mod tests {
    use crate::provenance::{ProvenanceKind, ProvenanceRecord};

    #[test]
    fn test_new_stores_component_id() {
        let r = ProvenanceRecord::new("comp-1", ProvenanceKind::BuildSystem, "ci.example.com", "build-42", 700);
        assert_eq!(r.component_id, "comp-1");
    }

    #[test]
    fn test_new_stores_kind() {
        let r = ProvenanceRecord::new("comp-2", ProvenanceKind::Vcs, "github.com/org/repo", "abc123", 701);
        assert!(matches!(r.kind, ProvenanceKind::Vcs));
    }

    #[test]
    fn test_new_stores_source() {
        let r = ProvenanceRecord::new("comp-3", ProvenanceKind::Registry, "registry.npmjs.org", "build-99", 702);
        assert_eq!(r.source, "registry.npmjs.org");
    }

    #[test]
    fn test_new_stores_build_id() {
        let r = ProvenanceRecord::new("comp-4", ProvenanceKind::ArtifactStore, "artifactory.corp", "artifact-777", 703);
        assert_eq!(r.build_id, "artifact-777");
    }

    #[test]
    fn test_new_stores_tick() {
        let r = ProvenanceRecord::new("comp-5", ProvenanceKind::BuildSystem, "jenkins.corp", "build-1", 999);
        assert_eq!(r.tick, 999);
    }

    #[test]
    fn test_with_metadata_stores_key_value() {
        let r = ProvenanceRecord::new("comp-6", ProvenanceKind::Vcs, "src", "b-id", 800)
            .with_metadata("commit", "deadbeef");
        assert_eq!(r.metadata.get("commit"), Some(&"deadbeef".to_string()));
    }
}
