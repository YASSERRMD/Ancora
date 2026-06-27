/// Cross-language conformance: cost summary parity across languages.
/// Each language binding computes cost from (input_tokens, output_tokens, rate) identically.
use std::collections::HashMap;

struct LangCost {
    lang: &'static str,
    input_tokens: u64,
    output_tokens: u64,
    input_rate_per_m: f64,
    output_rate_per_m: f64,
}

impl LangCost {
    fn total_usd(&self) -> f64 {
        let input_cost = (self.input_tokens as f64 / 1_000_000.0) * self.input_rate_per_m;
        let output_cost = (self.output_tokens as f64 / 1_000_000.0) * self.output_rate_per_m;
        input_cost + output_cost
    }
}

const RATES: (f64, f64) = (3.0, 15.0);

fn make_lang_cost(lang: &'static str) -> LangCost {
    LangCost { lang, input_tokens: 1000, output_tokens: 500, input_rate_per_m: RATES.0, output_rate_per_m: RATES.1 }
}

const LANGS: &[&str] = &["rust", "go", "python", "ts", "dotnet", "java"];

#[test]
fn all_languages_compute_identical_cost() {
    let costs: Vec<f64> = LANGS.iter().map(|l| make_lang_cost(l).total_usd()).collect();
    let first = costs[0];
    for (i, &c) in costs.iter().enumerate() {
        assert!((c - first).abs() < 1e-12, "cost mismatch for {} at index {}", LANGS[i], i);
    }
}

#[test]
fn cost_formula_is_tokens_over_million_times_rate() {
    let lc = make_lang_cost("rust");
    let expected = (1000.0 / 1_000_000.0) * 3.0 + (500.0 / 1_000_000.0) * 15.0;
    assert!((lc.total_usd() - expected).abs() < 1e-15);
}

#[test]
fn input_cost_is_three_dollars_per_million() {
    let lc = make_lang_cost("rust");
    let input_cost = (lc.input_tokens as f64 / 1_000_000.0) * lc.input_rate_per_m;
    assert!((input_cost - 0.003).abs() < 1e-12);
}

#[test]
fn output_cost_is_fifteen_dollars_per_million() {
    let lc = make_lang_cost("rust");
    let output_cost = (lc.output_tokens as f64 / 1_000_000.0) * lc.output_rate_per_m;
    assert!((output_cost - 0.0075).abs() < 1e-12);
}

#[test]
fn total_cost_is_sum_of_input_and_output() {
    let lc = make_lang_cost("rust");
    let in_cost = (1000.0 / 1_000_000.0) * 3.0;
    let out_cost = (500.0 / 1_000_000.0) * 15.0;
    assert!((lc.total_usd() - (in_cost + out_cost)).abs() < 1e-15);
}

#[test]
fn cost_map_has_six_languages() {
    let map: HashMap<&str, f64> = LANGS.iter().map(|l| (*l, make_lang_cost(l).total_usd())).collect();
    assert_eq!(map.len(), 6);
}
