// Coverage gate: cost model formula exercised by tests.

const COST_RATE_IN_PER_M: f64 = 3.0;
const COST_RATE_OUT_PER_M: f64 = 15.0;

fn compute_cost(input_tokens: u64, output_tokens: u64) -> f64 {
    (input_tokens as f64 / 1_000_000.0) * COST_RATE_IN_PER_M
        + (output_tokens as f64 / 1_000_000.0) * COST_RATE_OUT_PER_M
}

const COST_TEST_CASES: &[(u64, u64, f64)] = &[
    (1_000_000, 0,         3.0),
    (0,         1_000_000, 15.0),
    (1_000_000, 1_000_000, 18.0),
    (500_000,   200_000,   1.5 + 3.0),
    (0,         0,         0.0),
];

#[test]
fn test_cost_formula_for_all_cases() {
    for (inp, out, expected) in COST_TEST_CASES {
        let actual = compute_cost(*inp, *out);
        assert!((actual - expected).abs() < 0.0001, "inp={inp} out={out}: expected {expected} got {actual}");
    }
}

#[test]
fn test_five_cost_test_cases() {
    assert_eq!(COST_TEST_CASES.len(), 5);
}

#[test]
fn test_input_rate_is_3_per_million() {
    assert!((COST_RATE_IN_PER_M - 3.0).abs() < 0.0001);
}

#[test]
fn test_output_rate_is_15_per_million() {
    assert!((COST_RATE_OUT_PER_M - 15.0).abs() < 0.0001);
}

#[test]
fn test_zero_tokens_zero_cost() {
    assert_eq!(compute_cost(0, 0), 0.0);
}

#[test]
fn test_output_tokens_cost_more_than_input() {
    assert!(compute_cost(0, 1_000_000) > compute_cost(1_000_000, 0));
}
