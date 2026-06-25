use ancora_core::journal_mask::mask_events;
use ancora_proto::ancora::JournalEvent;
use proptest::prelude::*;
use prost::Message;

/// Attempt to decode arbitrary bytes as a `JournalEvent` protobuf.
///
/// This must never panic -- it should always return `Ok` or a decode error.
/// Only sanity-check that the round-trip is stable when decoding succeeds.
fn decode_journal_event(bytes: &[u8]) -> Option<JournalEvent> {
    JournalEvent::decode(bytes).ok()
}

proptest! {
    #[test]
    fn wire_decoder_never_panics_on_arbitrary_bytes(
        bytes in proptest::collection::vec(any::<u8>(), 0..512)
    ) {
        // Must not panic; errors are acceptable.
        let _ = decode_journal_event(&bytes);
    }

    #[test]
    fn wire_decoder_accepts_empty_bytes(
        _sentinel in Just(())
    ) {
        let result = decode_journal_event(&[]);
        // Empty bytes decode to the default JournalEvent (all zero/empty fields).
        prop_assert!(result.is_some(), "empty bytes should decode to default JournalEvent");
    }

    #[test]
    fn wire_decoder_round_trips_valid_events(
        run_id in "[a-z]{4,12}",
        seq in 0u64..1000
    ) {
        use ancora_proto::ancora::{journal_event::Event, RunStartedEvent};
        let original = JournalEvent {
            event_id: format!("{}-{}", run_id, seq),
            run_id: run_id.clone(),
            seq,
            recorded_at_ns: 0,
            event: Some(Event::RunStarted(RunStartedEvent {
                run_id: run_id.clone(),
                spec_bytes: vec![],
                spec_type: "AgentSpec".into(),
            })),
        };

        let mut buf = Vec::new();
        original.encode(&mut buf).unwrap();

        let decoded = JournalEvent::decode(buf.as_slice()).unwrap();
        prop_assert_eq!(&decoded.run_id, &run_id);
        prop_assert_eq!(decoded.seq, seq);
    }

    #[test]
    fn mask_events_on_arbitrary_decoded_bytes_does_not_panic(
        bytes in proptest::collection::vec(any::<u8>(), 0..256)
    ) {
        if let Some(event) = decode_journal_event(&bytes) {
            let _ = mask_events(&[event]);
        }
    }

    #[test]
    fn wire_decoder_high_seq_numbers_round_trip(
        seq in (u64::MAX - 1000)..=u64::MAX
    ) {
        use ancora_proto::ancora::{journal_event::Event, RunCompletedEvent};
        let ev = JournalEvent {
            event_id: "e".into(),
            run_id: "r".into(),
            seq,
            recorded_at_ns: 0,
            event: Some(Event::RunCompleted(RunCompletedEvent { output_json: String::new() })),
        };
        let mut buf = Vec::new();
        ev.encode(&mut buf).unwrap();
        let decoded = JournalEvent::decode(buf.as_slice()).unwrap();
        prop_assert_eq!(decoded.seq, seq);
    }
}
