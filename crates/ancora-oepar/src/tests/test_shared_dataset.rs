/// Tests verifying the cross-language shared eval dataset is stable and complete.
use crate::eval_parity::shared_eval_dataset;
use crate::polyglot::{shared_eval_case_ids, SHARED_EVAL_DATASET_ID};

#[test]
fn test_shared_dataset_id_is_stable() {
    assert_eq!(SHARED_EVAL_DATASET_ID, "ancora-oepar-v1");
}

#[test]
fn test_shared_dataset_case_ids_match_polyglot_registry() {
    let dataset = shared_eval_dataset();
    let registry_ids = shared_eval_case_ids();
    assert_eq!(
        dataset.len(),
        registry_ids.len(),
        "dataset length must match registry"
    );
    for (case, &registry_id) in dataset.iter().zip(registry_ids.iter()) {
        assert_eq!(case.id, registry_id, "case ids must be in the same order");
    }
}

#[test]
fn test_all_cases_have_metadata() {
    let dataset = shared_eval_dataset();
    for case in &dataset {
        assert!(
            !case.metadata.is_empty(),
            "case {:?} should have at least one metadata entry",
            case.id
        );
    }
}

#[test]
fn test_all_cases_have_non_empty_fields() {
    let dataset = shared_eval_dataset();
    for case in &dataset {
        assert!(!case.id.is_empty(), "case id must not be empty");
        assert!(!case.input.is_empty(), "case input must not be empty");
        assert!(
            !case.expected_output.is_empty(),
            "expected_output must not be empty"
        );
    }
}
