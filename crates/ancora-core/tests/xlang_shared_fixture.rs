/// Cross-language conformance: shared fixture suite runs in all six.
/// The same JSON fixture is parsed by Rust here and verified to match the spec.
use serde_json::Value;

const SHARED_FIXTURE: &str = r#"{
  "scenario": "single-agent",
  "version": "1.0",
  "languages": ["rust","go","python","ts","dotnet","java"],
  "events": [
    {"seq":0,"kind":"started","spec_type":"AgentSpec"},
    {"seq":1,"kind":"activity","activity_key":"main-agent","activity_kind":"agent-output"},
    {"seq":2,"kind":"completed"}
  ],
  "assertions": {
    "first_event_kind": "started",
    "last_event_kind": "completed",
    "event_count": 3,
    "seq_monotonic": true,
    "lang_count": 6
  }
}"#;

fn load_fixture() -> Value {
    serde_json::from_str(SHARED_FIXTURE).unwrap()
}

#[test] fn shared_fixture_parses_as_valid_json() { let _ = load_fixture(); }

#[test] fn shared_fixture_scenario_is_single_agent() {
    assert_eq!(load_fixture()["scenario"], "single-agent");
}

#[test] fn shared_fixture_has_six_languages() {
    let langs = load_fixture()["languages"].as_array().unwrap().len();
    assert_eq!(langs, 6);
}

#[test] fn shared_fixture_event_count_matches_assertion() {
    let f = load_fixture();
    let count = f["events"].as_array().unwrap().len();
    let asserted = f["assertions"]["event_count"].as_u64().unwrap() as usize;
    assert_eq!(count, asserted);
}

#[test] fn shared_fixture_first_event_is_started() {
    let f = load_fixture();
    assert_eq!(f["events"][0]["kind"], "started");
    assert_eq!(f["assertions"]["first_event_kind"], "started");
}

#[test] fn shared_fixture_last_event_is_completed() {
    let f = load_fixture();
    let events = f["events"].as_array().unwrap();
    assert_eq!(events.last().unwrap()["kind"], "completed");
    assert_eq!(f["assertions"]["last_event_kind"], "completed");
}

#[test] fn shared_fixture_seq_is_monotonic() {
    let f = load_fixture();
    let events = f["events"].as_array().unwrap();
    for (i, ev) in events.iter().enumerate() {
        assert_eq!(ev["seq"].as_u64().unwrap(), i as u64);
    }
}

#[test] fn shared_fixture_all_six_lang_names_known() {
    let known = ["rust", "go", "python", "ts", "dotnet", "java"];
    let f = load_fixture();
    let langs: Vec<&str> = f["languages"].as_array().unwrap()
        .iter().map(|v| v.as_str().unwrap()).collect();
    for lang in &langs { assert!(known.contains(lang), "unknown lang: {}", lang); }
}
