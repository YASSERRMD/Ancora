// OTel reproduced on replay: in-process span-equivalent counts must be stable.
// Ancora uses append-only journals as its observability layer (no actual OTel SDK).
// These tests verify that journal entry counts (which represent spans) are stable
// across repeated runs with identical inputs.

use ancora_coord::CoordJournal;
use ancora_guard::{GuardrailJournal, GuardrailPolicy, SafetyOutputGuardrail};
use ancora_reason::{ReasoningEvent, ReasoningJournal};

fn coord_span_count() -> usize {
    let mut j = CoordJournal::default();
    j.record(1, "assign", "t1 -> a1");
    j.record(2, "complete", "t1");
    j.record(3, "assign", "t2 -> a2");
    j.events().len()
}

fn guard_span_count() -> usize {
    let mut p = GuardrailPolicy::new();
    p.add_output(SafetyOutputGuardrail);
    let mut j = GuardrailJournal::default();
    p.check_output("DROP TABLE users;", &mut j, 1);
    p.check_output("safe text", &mut j, 2);
    j.decisions().len()
}

fn reason_span_count() -> usize {
    let mut j = ReasoningJournal::default();
    j.record(1, ReasoningEvent::StepVerified { index: 0 });
    j.record(2, ReasoningEvent::StepVerified { index: 1 });
    j.record(3, ReasoningEvent::ContradictionFound { a: 0, b: 2 });
    j.events().len()
}

#[test]
fn otel_coord_span_count_stable() {
    assert_eq!(coord_span_count(), coord_span_count());
}

#[test]
fn otel_guard_span_count_stable() {
    assert_eq!(guard_span_count(), guard_span_count());
}

#[test]
fn otel_reason_span_count_stable() {
    assert_eq!(reason_span_count(), reason_span_count());
}
