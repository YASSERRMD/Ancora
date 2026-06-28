#[cfg(test)]
mod tests {
    use crate::signature::{ComponentSignature, SignatureAlgorithm, SignatureStore};

    fn make_sig(component_id: &str, signer: &str, tick: u64) -> ComponentSignature {
        ComponentSignature::new(
            component_id,
            SignatureAlgorithm::Ed25519,
            signer,
            "sig-value",
            tick,
        )
    }

    #[test]
    fn test_by_signer_returns_only_matching_sigs() {
        let mut store = SignatureStore::new();
        store.register(make_sig("comp-1", "signer-alice", 100));
        store.register(make_sig("comp-2", "signer-alice", 101));
        store.register(make_sig("comp-3", "signer-bob", 102));

        let alice_sigs = store.by_signer("signer-alice");
        assert_eq!(alice_sigs.len(), 2);
    }

    #[test]
    fn test_by_signer_returns_empty_for_unknown_signer() {
        let mut store = SignatureStore::new();
        store.register(make_sig("comp-1", "signer-alice", 100));
        assert!(store.by_signer("signer-unknown").is_empty());
    }

    #[test]
    fn test_by_signer_returns_correct_component_ids() {
        let mut store = SignatureStore::new();
        store.register(make_sig("comp-x", "signer-bob", 200));
        store.register(make_sig("comp-y", "signer-alice", 201));

        let bob_sigs = store.by_signer("signer-bob");
        assert_eq!(bob_sigs.len(), 1);
        assert_eq!(bob_sigs[0].component_id, "comp-x");
    }

    #[test]
    fn test_by_signer_empty_store_returns_empty() {
        let store = SignatureStore::new();
        assert!(store.by_signer("anyone").is_empty());
    }
}
