use ancora_core::stream::{emit_tokens, open_stream, StreamEvent};

#[test]
fn open_stream_creates_working_channel() {
    let (tx, rx) = open_stream();
    tx.send(StreamEvent::RunCompleted { output: "done".into() }).unwrap();
    let ev = rx.recv().unwrap();
    assert_eq!(ev, StreamEvent::RunCompleted { output: "done".into() });
}

#[test]
fn emit_tokens_sends_one_event_per_character() {
    let (tx, rx) = open_stream();
    emit_tokens(&tx, "abc");
    drop(tx);

    let events: Vec<_> = rx.into_iter().collect();
    assert_eq!(events.len(), 3);
    assert_eq!(events[0], StreamEvent::Token { text: "a".into() });
    assert_eq!(events[1], StreamEvent::Token { text: "b".into() });
    assert_eq!(events[2], StreamEvent::Token { text: "c".into() });
}

#[test]
fn emit_tokens_on_empty_string_sends_no_events() {
    let (tx, rx) = open_stream();
    emit_tokens(&tx, "");
    drop(tx);
    let events: Vec<_> = rx.into_iter().collect();
    assert!(events.is_empty());
}

#[test]
fn node_entered_event_roundtrips_through_channel() {
    let (tx, rx) = open_stream();
    tx.send(StreamEvent::NodeEntered {
        node_id: "my-node".into(),
        node_kind: "agent".into(),
    })
    .unwrap();
    let ev = rx.recv().unwrap();
    assert_eq!(
        ev,
        StreamEvent::NodeEntered {
            node_id: "my-node".into(),
            node_kind: "agent".into()
        }
    );
}

#[test]
fn emit_tokens_on_disconnected_receiver_does_not_panic() {
    let (tx, rx) = open_stream();
    drop(rx); // disconnect receiver
    emit_tokens(&tx, "should not panic"); // must silently succeed
}

#[test]
fn event_ordering_preserved_in_channel() {
    let (tx, rx) = open_stream();
    let events = vec![
        StreamEvent::NodeEntered { node_id: "n1".into(), node_kind: "agent".into() },
        StreamEvent::Token { text: "tok".into() },
        StreamEvent::NodeExited { node_id: "n1".into() },
        StreamEvent::RunCompleted { output: "result".into() },
    ];
    for ev in &events {
        tx.send(ev.clone()).unwrap();
    }
    drop(tx);
    let received: Vec<_> = rx.into_iter().collect();
    assert_eq!(received, events, "event ordering must be preserved");
}

#[test]
fn emit_tokens_handles_unicode_multibyte_chars() {
    let (tx, rx) = open_stream();
    emit_tokens(&tx, "Hi\u{1F600}");
    drop(tx);
    let received: Vec<_> = rx.into_iter().collect();
    assert_eq!(received.len(), 3, "3 chars: 'H', 'i', emoji");
    assert_eq!(received[2], StreamEvent::Token { text: "\u{1F600}".into() });
}
