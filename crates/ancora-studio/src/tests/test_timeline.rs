use crate::timeline::{Timeline, TimelineStep, StepKind};

fn make_step(index: usize, start_ms: u64, end_ms: u64, redacted: bool) -> TimelineStep {
    TimelineStep {
        index,
        kind: StepKind::LlmCall,
        label: format!("step-{}", index),
        start_ms,
        end_ms,
        tokens_in: Some(10),
        tokens_out: Some(5),
        cost_usd: Some(0.001),
        redacted,
    }
}

#[test]
fn test_timeline_renders_a_run() {
    let tl = Timeline::new(
        "r1",
        vec![
            make_step(0, 0, 100, false),
            make_step(1, 100, 250, false),
        ],
    );
    assert_eq!(tl.steps().len(), 2);
    assert_eq!(tl.total_duration_ms(), 250);
}

#[test]
fn test_timeline_redacted_steps_hidden() {
    let tl = Timeline::new(
        "r1",
        vec![
            make_step(0, 0, 100, false),
            make_step(1, 100, 200, true),
        ],
    );
    assert_eq!(tl.visible_steps().len(), 1);
}

#[test]
fn test_timeline_cost_sum() {
    let tl = Timeline::new(
        "r1",
        vec![make_step(0, 0, 100, false), make_step(1, 100, 200, false)],
    );
    assert!((tl.total_cost_usd() - 0.002).abs() < 1e-9);
}

#[test]
fn test_timeline_step_at() {
    let tl = Timeline::new("r1", vec![make_step(0, 0, 10, false), make_step(1, 10, 20, false)]);
    assert!(tl.step_at(1).is_some());
    assert!(tl.step_at(99).is_none());
}
