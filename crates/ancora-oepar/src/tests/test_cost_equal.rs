use crate::cost_parity::{check_cost_parity, reference_cost_record, PricingRates, TokenUsage};

const ALL_LANGS: &[&str] = &["rust", "python", "typescript", "go", "java", "csharp"];

#[test]
fn test_cost_equal_across_all_six_languages() {
    let records: Vec<_> = ALL_LANGS
        .iter()
        .map(|l| reference_cost_record(*l))
        .collect();
    let issues = check_cost_parity(&records);
    assert!(
        issues.is_empty(),
        "cost not equal across languages: {:?}",
        issues
    );
}

#[test]
fn test_pricing_rates_deterministic() {
    let rates = PricingRates::new(0.03, 0.06);
    let usage = TokenUsage::new(500, 200);
    let (in_cost, out_cost) = rates.compute_cost(usage);
    assert!((in_cost - 0.015).abs() < 1e-9, "input cost: {}", in_cost);
    assert!((out_cost - 0.012).abs() < 1e-9, "output cost: {}", out_cost);
}

#[test]
fn test_cost_values_are_positive() {
    for &lang in ALL_LANGS {
        let record = reference_cost_record(lang);
        for (key, val) in &record.attributes {
            assert!(
                *val >= 0.0,
                "language {:?} attribute {:?} is negative: {}",
                lang,
                key,
                val
            );
        }
    }
}
