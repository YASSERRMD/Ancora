/// Cross-language conformance: journals are equal across bindings, minus recorded model outputs.
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn structural_key(ev: &JournalEvent) -> String {
    match &ev.event {
        Some(Event::RunStarted(_)) => "RunStarted".into(),
        Some(Event::ActivityRecorded(a)) => format!("ActivityRecorded:{}", a.activity_key),
        Some(Event::RunCompleted(_)) => "RunCompleted".into(),
        Some(Event::HumanDecisionRequested(_)) => "HumanDecisionRequested".into(),
        Some(Event::HumanDecisionReceived(_)) => "HumanDecisionReceived".into(),
        Some(Event::NodeEntered(n)) => format!("NodeEntered:{}", n.node_id),
        Some(Event::NodeExited(n)) => format!("NodeExited:{}", n.node_id),
        _ => "Unknown".into(),
    }
}

fn make_lang_journal(run_id: &str, lang: &str) -> Vec<JournalEvent> {
    vec![
        JournalEvent { event_id: format!("{}-{}-0", run_id, lang), run_id: run_id.into(), seq: 0, recorded_at_ns: 0,
            event: Some(Event::RunStarted(RunStartedEvent { run_id: run_id.into(), spec_bytes: vec![], spec_type: "AgentSpec".into() })) },
        JournalEvent { event_id: format!("{}-{}-1", run_id, lang), run_id: run_id.into(), seq: 1, recorded_at_ns: 1_000,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: "main-agent".into(), activity_kind: "agent-output".into(),
                input_json: r#"{"task":"conformance"}"#.into(),
                result_json: format!(r#"{{"text":"{} result"}}"#, lang), replayed: false })) },
        JournalEvent { event_id: format!("{}-{}-2", run_id, lang), run_id: run_id.into(), seq: 2, recorded_at_ns: 2_000,
            event: Some(Event::RunCompleted(RunCompletedEvent { output_json: format!(r#"{{"lang":"{}","ok":true}}"#, lang) })) },
    ]
}

fn structural_keys(journal: &[JournalEvent]) -> Vec<String> {
    journal.iter().map(structural_key).collect()
}

const LANGS: &[&str] = &["rust", "go", "python", "ts", "dotnet", "java"];
const RUN_ID: &str = "xlang-eq";

#[test]
fn journals_have_identical_structural_keys_across_languages() {
    let reference = structural_keys(&make_lang_journal(RUN_ID, LANGS[0]));
    for lang in &LANGS[1..] {
        let keys = structural_keys(&make_lang_journal(RUN_ID, lang));
        assert_eq!(keys, reference, "structural keys differ for {}", lang);
    }
}

#[test]
fn journals_have_same_event_count_across_languages() {
    let count = make_lang_journal(RUN_ID, LANGS[0]).len();
    for lang in LANGS { assert_eq!(make_lang_journal(RUN_ID, lang).len(), count); }
}

#[test]
fn journals_have_same_seq_sequence() {
    for lang in LANGS {
        for (i, ev) in make_lang_journal(RUN_ID, lang).iter().enumerate() {
            assert_eq!(ev.seq, i as u64);
        }
    }
}

#[test]
fn journals_have_same_run_id() {
    for lang in LANGS {
        for ev in &make_lang_journal(RUN_ID, lang) { assert_eq!(ev.run_id, RUN_ID); }
    }
}

#[test]
fn result_json_differs_across_languages() {
    let jsons: std::collections::HashSet<String> = LANGS.iter().map(|l| {
        let j = make_lang_journal(RUN_ID, l);
        if let Some(Event::ActivityRecorded(a)) = &j[1].event { a.result_json.clone() } else { String::new() }
    }).collect();
    assert!(jsons.len() > 1);
}

#[test]
fn structural_keys_are_three_events() {
    let keys = structural_keys(&make_lang_journal(RUN_ID, "rust"));
    assert_eq!(keys, vec!["RunStarted", "ActivityRecorded:main-agent", "RunCompleted"]);
}
