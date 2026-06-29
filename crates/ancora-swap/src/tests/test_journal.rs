use crate::journal::{JournalEntry, SwapEvent, SwapJournal};
use crate::model::ModelVersion;
use crate::runtime::{make_model, SwapRuntime};

#[test]
fn test_swap_journaled_and_replays() {
    let m1 = make_model("j1");
    let m2 = make_model("j2");
    let m1_ver = m1.version();
    let m2_ver = m2.version();

    let rt = SwapRuntime::new(m1);
    rt.swap(m2);

    let journal = rt.journal();
    assert_eq!(journal.len(), 1);

    let seq = journal.replay_sequence();
    assert_eq!(seq.len(), 1);
    assert_eq!(seq[0].0, m1_ver);
    assert_eq!(seq[0].1, m2_ver);
}

#[test]
fn test_journal_serialise_deserialise() {
    let mut journal = SwapJournal::new();
    journal.append(JournalEntry {
        event: SwapEvent::Swap {
            from: ModelVersion(1),
            to: ModelVersion(2),
        },
        timestamp_ns: 42,
    });

    let json = journal.to_json();
    let restored = SwapJournal::from_json(&json).expect("deserialize must succeed");
    assert_eq!(restored.len(), 1);
    let seq = restored.replay_sequence();
    assert_eq!(seq[0].0, ModelVersion(1));
    assert_eq!(seq[0].1, ModelVersion(2));
}

#[test]
fn test_rollback_journaled() {
    let m1 = make_model("rb1");
    let m2 = make_model("rb2");

    let rt = SwapRuntime::new(m1);
    rt.swap(m2);
    rt.rollback().unwrap();

    let journal = rt.journal();
    assert_eq!(journal.len(), 2, "swap + rollback = 2 entries");
}
