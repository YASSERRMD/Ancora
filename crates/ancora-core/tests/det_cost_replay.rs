/// Determinism: cost is reproduced on replay.
/// The cost of a run is deterministic because token counts are recorded in the journal.
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

#[derive(Debug, Clone, PartialEq)]
struct ReplayCost {
    input_tokens: u64,
    output_tokens: u64,
    cost_usd: f64,
}

fn compute_cost(input_tokens: u64, output_tokens: u64) -> f64 {
    (input_tokens as f64 / 1_000_000.0) * 3.0 + (output_tokens as f64 / 1_000_000.0) * 15.0
}

fn build_cost_journal(run_id: &str, steps: &[(u64, u64)]) -> Vec<JournalEvent> {
    let mut events = vec![JournalEvent {
        event_id: format!("{}-0", run_id),
        run_id: run_id.into(),
        seq: 0,
        recorded_at_ns: 0,
        event: Some(Event::RunStarted(RunStartedEvent {
            run_id: run_id.into(),
            spec_bytes: vec![],
            spec_type: "AgentSpec".into(),
        })),
    }];
    for (i, (inp, out)) in steps.iter().enumerate() {
        let cost = compute_cost(*inp, *out);
        events.push(JournalEvent {
            event_id: format!("{}-{}", run_id, i + 1),
            run_id: run_id.into(),
            seq: (i + 1) as u64,
            recorded_at_ns: ((i + 1) * 1000) as i64,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: format!("llm-{}", i),
                activity_kind: "llm".into(),
                input_json: format!(r#"{{"input_tokens":{},"output_tokens":{}}}"#, inp, out),
                result_json: format!(
                    r#"{{"cost_usd":{:.10},"input_tokens":{},"output_tokens":{}}}"#,
                    cost, inp, out
                ),
                replayed: false,
            })),
        });
    }
    let n = events.len();
    events.push(JournalEvent {
        event_id: format!("{}-{}", run_id, n),
        run_id: run_id.into(),
        seq: n as u64,
        recorded_at_ns: (n * 1000) as i64,
        event: Some(Event::RunCompleted(RunCompletedEvent {
            output_json: r#"{"done":true}"#.into(),
        })),
    });
    events
}

fn replay_total_cost(j: &[JournalEvent]) -> f64 {
    j.iter()
        .filter_map(|e| {
            if let Some(Event::ActivityRecorded(a)) = &e.event {
                let v: serde_json::Value = serde_json::from_str(&a.result_json).ok()?;
                v["cost_usd"].as_f64()
            } else {
                None
            }
        })
        .sum()
}

const STEPS: &[(u64, u64)] = &[(1000, 500), (2000, 800), (500, 200)];

#[test]
fn cost_is_identical_on_two_replays() {
    let c1 = replay_total_cost(&build_cost_journal("det-cost-1", STEPS));
    let c2 = replay_total_cost(&build_cost_journal("det-cost-2", STEPS));
    assert!((c1 - c2).abs() < 1e-12);
}

#[test]
fn cost_formula_matches_direct_computation() {
    let j = build_cost_journal("det-cost-formula", STEPS);
    let replayed = replay_total_cost(&j);
    let direct: f64 = STEPS.iter().map(|(i, o)| compute_cost(*i, *o)).sum();
    assert!((replayed - direct).abs() < 1e-12);
}

#[test]
fn cost_is_non_zero_for_non_empty_steps() {
    assert!(replay_total_cost(&build_cost_journal("det-cost-nz", STEPS)) > 0.0);
}

#[test]
fn zero_tokens_produce_zero_cost() {
    assert_eq!(compute_cost(0, 0), 0.0);
}

#[test]
fn cost_journal_event_count_matches_steps() {
    let j = build_cost_journal("det-cost-cnt", STEPS);
    assert_eq!(j.len(), STEPS.len() + 2);
}
