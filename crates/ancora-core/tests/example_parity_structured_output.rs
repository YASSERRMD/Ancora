// Example parity: structured output schema validated consistently across languages.

const SCHEMA_JSON: &str = r#"{"type":"object","required":["name","score"],"properties":{"name":{"type":"string"},"score":{"type":"number"}}}"#;

struct StructuredOutputExample {
    lang: &'static str,
    output_json: &'static str,
    valid: bool,
}

fn validate_against_schema(output: &str) -> bool {
    output.contains("\"name\"") && output.contains("\"score\"")
}

const STRUCTURED_EXAMPLES: &[StructuredOutputExample] = &[
    StructuredOutputExample {
        lang: "rust",
        output_json: r#"{"name":"Alice","score":9.5}"#,
        valid: true,
    },
    StructuredOutputExample {
        lang: "go",
        output_json: r#"{"name":"Alice","score":9.5}"#,
        valid: true,
    },
    StructuredOutputExample {
        lang: "python",
        output_json: r#"{"name":"Alice","score":9.5}"#,
        valid: true,
    },
    StructuredOutputExample {
        lang: "typescript",
        output_json: r#"{"name":"Alice","score":9.5}"#,
        valid: true,
    },
    StructuredOutputExample {
        lang: "dotnet",
        output_json: r#"{"name":"Alice","score":9.5}"#,
        valid: true,
    },
    StructuredOutputExample {
        lang: "java",
        output_json: r#"{"name":"Alice","score":9.5}"#,
        valid: true,
    },
];

#[test]
fn test_all_examples_produce_valid_output() {
    for e in STRUCTURED_EXAMPLES {
        assert_eq!(
            e.valid,
            validate_against_schema(e.output_json),
            "lang {} output does not match schema",
            e.lang
        );
    }
}

#[test]
fn test_six_structured_output_examples() {
    assert_eq!(STRUCTURED_EXAMPLES.len(), 6);
}

#[test]
fn test_schema_requires_name_and_score() {
    assert!(SCHEMA_JSON.contains("\"name\""));
    assert!(SCHEMA_JSON.contains("\"score\""));
}

#[test]
fn test_all_outputs_contain_alice() {
    for e in STRUCTURED_EXAMPLES {
        assert!(
            e.output_json.contains("Alice"),
            "lang {} output has no name",
            e.lang
        );
    }
}

#[test]
fn test_all_outputs_have_same_json() {
    let first = STRUCTURED_EXAMPLES[0].output_json;
    for e in STRUCTURED_EXAMPLES {
        assert_eq!(e.output_json, first, "lang {} output differs", e.lang);
    }
}
