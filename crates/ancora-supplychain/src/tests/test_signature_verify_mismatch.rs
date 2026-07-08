#[cfg(test)]
mod tests {
    use crate::signature::{
        ComponentSignature, SignatureAlgorithm, SignatureStore, VerificationResult,
    };

    fn make_sig(component_id: &str, signature: &str) -> ComponentSignature {
        ComponentSignature::new(
            component_id,
            SignatureAlgorithm::Ed25519,
            "signer",
            signature,
            600,
        )
    }

    #[test]
    fn test_wrong_signature_returns_invalid() {
        let mut store = SignatureStore::new();
        store.register(make_sig("comp-a", "correct-sig"));
        let result = store.verify("comp-a", "wrong-sig");
        assert!(matches!(result, VerificationResult::Invalid(_)));
    }

    #[test]
    fn test_missing_component_returns_missing() {
        let store = SignatureStore::new();
        let result = store.verify("never-registered", "any-sig");
        assert!(matches!(result, VerificationResult::Missing));
    }

    #[test]
    fn test_has_signature_returns_false_for_unregistered() {
        let store = SignatureStore::new();
        assert!(!store.has_signature("unregistered-comp"));
    }

    #[test]
    fn test_invalid_result_contains_reason() {
        let mut store = SignatureStore::new();
        store.register(make_sig("comp-b", "real-sig"));
        let result = store.verify("comp-b", "fake-sig");
        if let VerificationResult::Invalid(reason) = result {
            assert!(!reason.is_empty());
        } else {
            panic!("Expected Invalid variant");
        }
    }
}
