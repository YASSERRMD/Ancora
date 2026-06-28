// Example parity: A2A handoff example envelope structure identical across language pairs.

const A2A_PROTOCOL: &str = "a2a/1.0";

struct A2AEnvelope {
    protocol: &'static str,
    sender_lang: &'static str,
    recipient_lang: &'static str,
    run_id: &'static str,
    payload_key: &'static str,
}

const A2A_EXAMPLES: &[A2AEnvelope] = &[
    A2AEnvelope { protocol: A2A_PROTOCOL, sender_lang: "rust", recipient_lang: "python", run_id: "a2a-rp-001", payload_key: "task" },
    A2AEnvelope { protocol: A2A_PROTOCOL, sender_lang: "go",   recipient_lang: "ts",     run_id: "a2a-gt-001", payload_key: "task" },
    A2AEnvelope { protocol: A2A_PROTOCOL, sender_lang: "ts",   recipient_lang: "dotnet", run_id: "a2a-td-001", payload_key: "task" },
    A2AEnvelope { protocol: A2A_PROTOCOL, sender_lang: "java", recipient_lang: "rust",   run_id: "a2a-jr-001", payload_key: "task" },
];

#[test]
fn test_all_envelopes_use_a2a_protocol() {
    for e in A2A_EXAMPLES {
        assert_eq!(e.protocol, A2A_PROTOCOL, "envelope for {}->{} has wrong protocol", e.sender_lang, e.recipient_lang);
    }
}

#[test]
fn test_all_run_ids_non_empty() {
    for e in A2A_EXAMPLES { assert!(!e.run_id.is_empty()); }
}

#[test]
fn test_payload_key_is_task_for_all() {
    for e in A2A_EXAMPLES { assert_eq!(e.payload_key, "task"); }
}

#[test]
fn test_four_a2a_examples() {
    assert_eq!(A2A_EXAMPLES.len(), 4);
}

#[test]
fn test_no_self_sender_recipient() {
    for e in A2A_EXAMPLES { assert_ne!(e.sender_lang, e.recipient_lang); }
}

#[test]
fn test_protocol_version_is_1_0() {
    assert!(A2A_PROTOCOL.ends_with("1.0"));
}
