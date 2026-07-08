#[cfg(test)]
mod tests {
    use crate::signature::{
        ComponentSignature, SignatureAlgorithm, SignatureStore, VerificationResult,
    };

    fn make_sig(component_id: &str, signature: &str) -> ComponentSignature {
        ComponentSignature::new(
            component_id,
            SignatureAlgorithm::Ed25519,
            "trusted-signer",
            signature,
            500,
        )
    }

    #[test]
    fn test_register_and_verify_valid_signature() {
        let mut store = SignatureStore::new();
        store.register(make_sig("comp-1", "sig-abc"));
        let result = store.verify("comp-1", "sig-abc");
        assert!(matches!(result, VerificationResult::Valid));
    }

    #[test]
    fn test_has_signature_after_register() {
        let mut store = SignatureStore::new();
        store.register(make_sig("comp-2", "sig-xyz"));
        assert!(store.has_signature("comp-2"));
    }

    #[test]
    fn test_count_after_register() {
        let mut store = SignatureStore::new();
        store.register(make_sig("comp-3", "sig-one"));
        store.register(make_sig("comp-4", "sig-two"));
        assert_eq!(store.count(), 2);
    }

    #[test]
    fn test_verify_returns_valid_for_correct_sig() {
        let mut store = SignatureStore::new();
        store.register(make_sig("comp-5", "correct-sig"));
        assert!(matches!(
            store.verify("comp-5", "correct-sig"),
            VerificationResult::Valid
        ));
    }
}
