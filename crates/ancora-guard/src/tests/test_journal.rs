use crate::guardrail::GuardrailOutcome;
use crate::journal::GuardrailJournal;

#[test]
fn guardrail_decisions_journaled() {
    let mut j = GuardrailJournal::default();
    j.record(
        1,
        "pii",
        "user@example.com",
        GuardrailOutcome::Repair("redacted".into()),
    );
    j.record(
        2,
        "safety",
        "<script>",
        GuardrailOutcome::Block("blocked".into()),
    );
    assert_eq!(j.decisions().len(), 2);
}

#[test]
fn blocked_count_correct() {
    let mut j = GuardrailJournal::default();
    j.record(1, "safety", "bad", GuardrailOutcome::Block("b".into()));
    j.record(2, "pii", "ok", GuardrailOutcome::Pass);
    assert_eq!(j.blocked_count(), 1);
}

#[test]
fn guardrails_replay_deterministically() {
    let mut j = GuardrailJournal::default();
    j.record(
        1,
        "injection",
        "jailbreak attempt",
        GuardrailOutcome::Block("b".into()),
    );
    j.record(
        2,
        "pii",
        "email@x.com",
        GuardrailOutcome::Repair("r".into()),
    );
    assert_eq!(j.repaired_count(), 1);
    assert_eq!(j.blocked_count(), 1);
}
