use crate::cost_parity::{reference_cost_record, check_cost_parity, REQUIRED_COST_ATTRS};

#[test]
fn test_reference_cost_record_is_complete() {
    let record = reference_cost_record("rust");
    assert!(record.is_complete(), "cost record missing: {:?}", record.missing_attributes());
}

#[test]
fn test_all_required_attrs_present() {
    let record = reference_cost_record("python");
    for attr in REQUIRED_COST_ATTRS {
        assert!(
            record.attributes.contains_key(*attr),
            "missing required cost attribute: {:?}",
            attr
        );
    }
}

#[test]
fn test_cost_parity_across_languages() {
    let langs = &["rust", "python", "typescript", "go", "java", "csharp"];
    let records: Vec<_> = langs.iter().map(|l| reference_cost_record(*l)).collect();
    let issues = check_cost_parity(&records);
    assert!(issues.is_empty(), "cost parity issues: {:?}", issues);
}

#[test]
fn test_total_cost_equals_sum_of_parts() {
    let record = reference_cost_record("rust");
    let input = record.attributes["gen_ai.cost.input_usd"];
    let output = record.attributes["gen_ai.cost.output_usd"];
    let total = record.attributes["gen_ai.cost.total_usd"];
    assert!(
        (input + output - total).abs() < 1e-9,
        "total cost should equal input+output: {} + {} != {}",
        input, output, total
    );
}
