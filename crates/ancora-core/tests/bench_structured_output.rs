// Benchmark: structured output validation -- 500k validations under 300ms.

use std::time::Instant;

const SO_BENCH_N: usize = 500_000;
const SO_BENCH_MS: u128 = 300;

struct OutputSchema {
    required_fields: &'static [&'static str],
}

fn validate_json_fields(json: &str, schema: &OutputSchema) -> bool {
    for field in schema.required_fields {
        if !json.contains(&format!("\"{}\":", field)) {
            return false;
        }
    }
    true
}

const AGENT_OUTPUT_SCHEMA: OutputSchema = OutputSchema {
    required_fields: &["name", "score"],
};

fn make_output_json(idx: usize) -> String {
    format!(r#"{{"name":"result-{idx}","score":{:.2}}}"#, (idx % 100) as f64 / 100.0)
}

#[test]
fn test_bench_500k_structured_output_validations_under_300ms() {
    let t0 = Instant::now();
    let mut valid = 0u64;
    for i in 0..SO_BENCH_N {
        let json = make_output_json(i);
        if validate_json_fields(&json, &AGENT_OUTPUT_SCHEMA) { valid += 1; }
    }
    let elapsed = t0.elapsed().as_millis();
    assert!(elapsed < SO_BENCH_MS, "took {}ms budget {}ms", elapsed, SO_BENCH_MS);
    assert_eq!(valid, SO_BENCH_N as u64);
}

#[test]
fn test_valid_json_passes() {
    let json = r#"{"name":"test","score":0.9}"#;
    assert!(validate_json_fields(json, &AGENT_OUTPUT_SCHEMA));
}

#[test]
fn test_missing_field_fails() {
    let json = r#"{"name":"test"}"#;
    assert!(!validate_json_fields(json, &AGENT_OUTPUT_SCHEMA));
}

#[test]
fn test_output_json_contains_name_and_score() {
    let j = make_output_json(42);
    assert!(j.contains("\"name\":"));
    assert!(j.contains("\"score\":"));
}
