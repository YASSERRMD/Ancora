use ancora_core::activity::{record_or_replay, Activity};
use ancora_core::error::AncoraError;
use ancora_core::idempotency::{write_once, WriteActivity};
use ancora_core::journal::{JournalStore, MemoryStore};

struct StaticActivity {
    key: String,
    result: String,
}

impl Activity for StaticActivity {
    fn execute(&self) -> Result<String, AncoraError> {
        Ok(self.result.clone())
    }
    fn key(&self) -> String {
        self.key.clone()
    }
}

struct EmptyKeyActivity;

impl Activity for EmptyKeyActivity {
    fn execute(&self) -> Result<String, AncoraError> {
        Ok(r#""done""#.to_string())
    }
    fn key(&self) -> String {
        String::new()
    }
}

#[test]
fn write_once_requires_non_empty_key() {
    let act = EmptyKeyActivity;
    let err = WriteActivity::new(&act);
    assert!(
        err.is_err(),
        "WriteActivity must reject an empty idempotency key"
    );
}

#[test]
fn write_once_records_on_first_call() {
    let store = MemoryStore::new();
    let act = StaticActivity {
        key: "write-1".into(),
        result: r#""first""#.into(),
    };
    let wa = WriteActivity::new(&act).unwrap();
    let result = write_once("run-w1", wa, &store).unwrap();
    assert_eq!(result, r#""first""#);

    let events = store.read("run-w1").unwrap();
    assert_eq!(events.len(), 1, "one event must be journaled");
}

#[test]
fn write_once_replays_on_second_call_without_re_executing() {
    let store = MemoryStore::new();

    let act1 = StaticActivity {
        key: "write-replay".into(),
        result: r#""original""#.into(),
    };
    let wa1 = WriteActivity::new(&act1).unwrap();
    write_once("run-wr", wa1, &store).unwrap();

    let act2 = StaticActivity {
        key: "write-replay".into(),
        result: r#""overwrite-attempt""#.into(),
    };
    let wa2 = WriteActivity::new(&act2).unwrap();
    let result = write_once("run-wr", wa2, &store).unwrap();

    assert_eq!(
        result, r#""original""#,
        "second call must return the journaled result"
    );
    let events = store.read("run-wr").unwrap();
    assert_eq!(events.len(), 1, "no new event must be appended on replay");
}

#[test]
fn record_or_replay_idempotent_for_same_key() {
    let store = MemoryStore::new();

    for _ in 0..5 {
        let act = StaticActivity {
            key: "idempotent-key".into(),
            result: r#""value""#.into(),
        };
        let result = record_or_replay("run-idem", &act, &store).unwrap();
        assert_eq!(result, r#""value""#);
    }

    let events = store.read("run-idem").unwrap();
    assert_eq!(events.len(), 1, "only one event must ever be appended");
}

#[test]
fn different_keys_produce_independent_events() {
    let store = MemoryStore::new();

    for key in ["key-alpha", "key-beta", "key-gamma"] {
        let act = StaticActivity {
            key: key.into(),
            result: format!(r#""{key}""#),
        };
        record_or_replay("run-multi-key", &act, &store).unwrap();
    }

    let events = store.read("run-multi-key").unwrap();
    assert_eq!(events.len(), 3);
}

#[test]
fn write_activity_key_accessor_returns_non_empty_string() {
    let act = StaticActivity {
        key: "my-key".into(),
        result: r#""r""#.into(),
    };
    let wa = WriteActivity::new(&act).unwrap();
    assert!(!wa.key().is_empty(), "key() must return a non-empty string");
    assert_eq!(wa.key(), "my-key");
}
