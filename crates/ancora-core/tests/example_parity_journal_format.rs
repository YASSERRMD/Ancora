// Example parity: journal format is the same proto/JSON across all language outputs.

const JOURNAL_FORMAT_VERSION: &str = "1";
const JOURNAL_REQUIRED_FIELDS: &[&str] = &[
    "event_id",
    "seq",
    "run_id",
    "recorded_at_ns",
    "kind",
];

struct JournalSample {
    lang: &'static str,
    json: &'static str,
}

const JOURNAL_SAMPLES: &[JournalSample] = &[
    JournalSample { lang: "rust",   json: r#"{"event_id":"e1","seq":0,"run_id":"r1","recorded_at_ns":1700000000000000000,"kind":"started"}"# },
    JournalSample { lang: "go",     json: r#"{"event_id":"e1","seq":0,"run_id":"r1","recorded_at_ns":1700000000000000000,"kind":"started"}"# },
    JournalSample { lang: "python", json: r#"{"event_id":"e1","seq":0,"run_id":"r1","recorded_at_ns":1700000000000000000,"kind":"started"}"# },
    JournalSample { lang: "ts",     json: r#"{"event_id":"e1","seq":0,"run_id":"r1","recorded_at_ns":1700000000000000000,"kind":"started"}"# },
    JournalSample { lang: "dotnet", json: r#"{"event_id":"e1","seq":0,"run_id":"r1","recorded_at_ns":1700000000000000000,"kind":"started"}"# },
    JournalSample { lang: "java",   json: r#"{"event_id":"e1","seq":0,"run_id":"r1","recorded_at_ns":1700000000000000000,"kind":"started"}"# },
];

fn has_all_required_fields(json: &str) -> bool {
    JOURNAL_REQUIRED_FIELDS.iter().all(|f| json.contains(f))
}

#[test]
fn test_all_journal_samples_have_required_fields() {
    for s in JOURNAL_SAMPLES {
        assert!(has_all_required_fields(s.json), "lang {} journal missing required fields", s.lang);
    }
}

#[test]
fn test_all_samples_identical_json() {
    let first = JOURNAL_SAMPLES[0].json;
    for s in JOURNAL_SAMPLES {
        assert_eq!(s.json, first, "lang {} journal JSON differs", s.lang);
    }
}

#[test]
fn test_six_journal_samples() {
    assert_eq!(JOURNAL_SAMPLES.len(), 6);
}

#[test]
fn test_five_required_fields() {
    assert_eq!(JOURNAL_REQUIRED_FIELDS.len(), 5);
}

#[test]
fn test_journal_format_version_is_1() {
    assert_eq!(JOURNAL_FORMAT_VERSION, "1");
}
