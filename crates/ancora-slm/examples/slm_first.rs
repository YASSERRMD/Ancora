//! slm_first: demonstrates SLM-first orchestration patterns offline.
//!
//! This example is fully self-contained: it uses in-memory replay functions
//! and requires no network calls or running model processes.

use ancora_slm::{
    constrained::{run_constrained, ConstrainedConfig},
    decompose::{execute_plan, DecompositionPlan, OutputFormat, Step},
    escalate::{run_with_escalation, EscalationPolicy},
    fewshot::{inject_few_shots, FewShotExample, FewShotLibrary},
    model::Message,
    prompt::{format_prompt, slm_system_prompt, PromptOptions, PromptStyle},
    repair::repair_tool_call,
    replay::make_replay_fn,
    schema::{augment_prompt_with_schema, validate, Schema},
    verifier::{run_verifiers, ValidJsonVerifier, Verifier},
};

fn main() {
    // ── 1. SLM-friendly prompt formatting ────────────────────────────────────
    println!("=== 1. Prompt Formatting ===");
    let sys = slm_system_prompt("Extract named entities", &[
        "Read the input sentence",
        "Identify all named entities",
        "Return a JSON object with an 'entities' array",
    ]);
    let messages = vec![
        Message::system(&sys),
        Message::user("Alice works at Anthropic in San Francisco."),
    ];
    let opts = PromptOptions {
        style: PromptStyle::Alpaca,
        request_json: true,
        max_chars: None,
    };
    let formatted = format_prompt(&messages, &opts);
    println!("Formatted prompt (first 200 chars):\n{}\n", &formatted[..formatted.len().min(200)]);

    // ── 2. Tool-call repair ───────────────────────────────────────────────────
    println!("=== 2. Tool-call Repair ===");
    let malformed = r#"Sure thing! Here: {"function_name": "get_weather", "args": {"city": "Paris",}}"#;
    match repair_tool_call(malformed) {
        Ok(tc) => println!("Repaired tool call: name={}, args={}", tc.name, tc.arguments),
        Err(e) => println!("Repair failed: {}", e),
    }
    println!();

    // ── 3. Constrained decoding ───────────────────────────────────────────────
    println!("=== 3. Constrained Decoding ===");
    let config = ConstrainedConfig { max_retries: 2, add_json_fence_instruction: true };
    // Simulated model that initially returns prose, then valid JSON.
    let attempt = std::cell::Cell::new(0usize);
    let result = run_constrained("Extract entities from: 'Bob is in London.'", &config, |_| {
        let n = attempt.get() + 1;
        attempt.set(n);
        if n == 1 {
            "I cannot provide JSON right now.".to_string()
        } else {
            r#"{"entities": ["Bob", "London"]}"#.to_string()
        }
    });
    println!("Constrained result (attempt {}): {:?}\n", attempt.get(), result);

    // ── 4. Schema-guided generation ───────────────────────────────────────────
    println!("=== 4. Schema-guided Generation ===");
    let schema = Schema::Object {
        required: vec!["entities".into()],
        properties: vec![
            ("entities".into(), Schema::Array { item_schema: Box::new(Schema::String) }),
        ],
    };
    let augmented = augment_prompt_with_schema("Extract entities.", &schema);
    println!("Schema-augmented prompt:\n{}\n", augmented);
    let sample_output = serde_json::json!({"entities": ["Alice", "ACME"]});
    let errors = validate(&sample_output, &schema);
    println!("Validation errors: {}\n", errors.len());

    // ── 5. Step decomposition ─────────────────────────────────────────────────
    println!("=== 5. Step Decomposition ===");
    let plan = DecompositionPlan::new(
        "Translate and summarise a paragraph",
        vec![
            Step {
                id: "translate".into(),
                description: "Translate the paragraph to English.".into(),
                output_format: OutputFormat::Text,
                optional: false,
            },
            Step {
                id: "summarise".into(),
                description: "Summarise the translated text in one sentence.".into(),
                output_format: OutputFormat::Text,
                optional: false,
            },
        ],
    );
    let model_fn = |_: &str| "Sample translated and summarised text.".to_string();
    let results = execute_plan(&plan, model_fn);
    for r in &results {
        println!("Step '{}': valid={}", r.step_id, r.valid);
    }
    println!();

    // ── 6. Verifier-heavy pattern ─────────────────────────────────────────────
    println!("=== 6. Verifier-heavy Pattern ===");
    let verifiers: Vec<Box<dyn Verifier>> = vec![Box::new(ValidJsonVerifier)];
    let good_output = r#"{"answer": 42}"#;
    let bad_output = "forty-two";
    println!("Good output passes: {}", run_verifiers(good_output, &verifiers).passed());
    println!("Bad output passes:  {}\n", run_verifiers(bad_output, &verifiers).passed());

    // ── 7. Escalation ─────────────────────────────────────────────────────────
    println!("=== 7. Escalation ===");
    let policy = EscalationPolicy::default();
    let slm_fn = |_: &str| "not json".to_string();
    let llm_fn = |_: &str| r#"{"answer": "from large model"}"#.to_string();
    let orch = run_with_escalation(
        "Answer the question",
        &policy,
        &verifiers,
        slm_fn,
        llm_fn,
        "phi-3-mini",
        "llama-70b",
    );
    println!(
        "Escalated={}, model={}, text={}\n",
        orch.escalated, orch.model_id, orch.text
    );

    // ── 8. Few-shot injection ─────────────────────────────────────────────────
    println!("=== 8. Few-shot Injection ===");
    let mut lib = FewShotLibrary::new();
    lib.add(FewShotExample::new("ner", "Input: 'Alice at Acme'", r#"{"entities":["Alice","Acme"]}"#, 1.0));
    lib.add(FewShotExample::new("ner", "Input: 'Bob in Paris'", r#"{"entities":["Bob","Paris"]}"#, 0.9));
    let prompt = "Input: 'Carol runs StartupX'";
    let injected = inject_few_shots(prompt, &lib, "ner", 2);
    println!("Few-shot injected (first 300 chars):\n{}\n", &injected[..injected.len().min(300)]);

    // ── 9. Replay ─────────────────────────────────────────────────────────────
    println!("=== 9. Deterministic Replay ===");
    let pairs = vec![
        ("prompt-a".to_string(), r#"{"val": 1}"#.to_string()),
        ("prompt-b".to_string(), r#"{"val": 2}"#.to_string()),
    ];
    let replay_fn = make_replay_fn(pairs, "{}");
    println!("Replay prompt-a: {}", replay_fn("prompt-a"));
    println!("Replay prompt-a again: {}", replay_fn("prompt-a"));
    println!("Replay prompt-b: {}", replay_fn("prompt-b"));

    println!("\nAll patterns demonstrated successfully.");
}
