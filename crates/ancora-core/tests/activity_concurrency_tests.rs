use std::sync::{Arc, Mutex};
use std::thread;

use ancora_core::activity::{record_or_replay, Activity};
use ancora_core::error::AncoraError;
use ancora_core::journal::{JournalStore, MemoryStore};

// MemoryStore is Clone + Send + Sync, so we can share it across threads directly.
// Arc is not needed since MemoryStore already wraps its state in Arc<Mutex<_>>.

struct CountingActivity {
    key: String,
    call_count: Arc<Mutex<u32>>,
    result: String,
}

impl Activity for CountingActivity {
    fn execute(&self) -> Result<String, AncoraError> {
        let mut count = self.call_count.lock().unwrap();
        *count += 1;
        Ok(self.result.clone())
    }

    fn key(&self) -> String {
        self.key.clone()
    }
}

#[test]
fn activity_executed_once_on_fresh_run() {
    let store = MemoryStore::new();
    let calls = Arc::new(Mutex::new(0u32));

    let activity = CountingActivity {
        key: "step-1".into(),
        call_count: Arc::clone(&calls),
        result: r#""hello""#.into(),
    };

    let result = record_or_replay("run-fresh", &activity, &store).unwrap();
    assert_eq!(result, r#""hello""#);
    assert_eq!(*calls.lock().unwrap(), 1, "must execute once on fresh run");
}

#[test]
fn activity_replayed_without_re_execution() {
    let store = MemoryStore::new();
    let calls = Arc::new(Mutex::new(0u32));

    let activity = CountingActivity {
        key: "step-replay".into(),
        call_count: Arc::clone(&calls),
        result: r#""cached""#.into(),
    };

    // First call -- records result
    record_or_replay("run-replay", &activity, &store).unwrap();
    assert_eq!(*calls.lock().unwrap(), 1);

    // Second call -- must use journal, not re-execute
    let result = record_or_replay("run-replay", &activity, &store).unwrap();
    assert_eq!(result, r#""cached""#);
    assert_eq!(*calls.lock().unwrap(), 1, "must not re-execute on replay");
}

#[test]
fn two_activities_with_different_keys_are_independent() {
    let store = MemoryStore::new();

    for key in ["act-a", "act-b"] {
        let act = CountingActivity {
            key: key.into(),
            call_count: Arc::new(Mutex::new(0)),
            result: format!(r#""{key}-result""#),
        };
        let result = record_or_replay("run-two", &act, &store).unwrap();
        assert!(result.contains(key));
    }
}

#[test]
fn concurrent_append_produces_no_mixed_events() {
    let store = MemoryStore::new();
    let n = 50usize;
    let mut handles = vec![];

    for i in 0..n {
        let store = store.clone();
        handles.push(thread::spawn(move || {
            let act = CountingActivity {
                key: format!("step-{}", i),
                call_count: Arc::new(Mutex::new(0)),
                result: format!(r#""{i}""#),
            };
            record_or_replay("run-concurrent", &act, &store).unwrap();
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    let events = store.read("run-concurrent").unwrap();
    assert_eq!(events.len(), n, "all {n} activities must be journaled");
}

#[test]
fn activity_result_survives_replay_across_multiple_steps() {
    let store = MemoryStore::new();
    let steps = vec!["alpha", "beta", "gamma"];

    for key in &steps {
        let act = CountingActivity {
            key: (*key).to_owned(),
            call_count: Arc::new(Mutex::new(0)),
            result: format!(r#""{key}-done""#),
        };
        record_or_replay("run-multi", &act, &store).unwrap();
    }

    // Replay all steps -- each must return its stored result
    for key in &steps {
        let calls = Arc::new(Mutex::new(0u32));
        let act = CountingActivity {
            key: (*key).to_owned(),
            call_count: Arc::clone(&calls),
            result: format!(r#""{key}-DIFFERENT""#),
        };
        let result = record_or_replay("run-multi", &act, &store).unwrap();
        assert!(
            result.contains(key) && !result.contains("DIFFERENT"),
            "replay must return original result for key={key}"
        );
        assert_eq!(*calls.lock().unwrap(), 0, "{key} must not be re-executed");
    }
}
