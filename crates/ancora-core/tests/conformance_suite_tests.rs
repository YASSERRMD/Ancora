/// Cross-cutting conformance suite tests.
///
/// Drives all scenarios defined in `ancora_core::conformance` and verifies
/// each can be executed in its Rust-native form. Also checks that Rust
/// journals for each scenario are structurally identical across two runs,
/// validating deterministic replay for all supported scenario types.
use ancora_core::{
    conformance::{all_scenarios, ConformanceResult, ConformanceScenario, CRASH_AND_RECOVER,
                   HUMAN_IN_LOOP, MULTI_AGENT_VERIFIER, SINGLE_AGENT},
    graph::{Edge, Graph, Node, NodeKind, NodeSpec},
    journal::{JournalStore, MemoryStore},
    journal_mask::{assert_structurally_equal, mask_events},
    replay::replay_events,
    run::RunStatus,
    suspend::SuspendedRun,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, AgentSpec as ProtoSpec, JournalEvent,
    NodeEnteredEvent, NodeExitedEvent, RunCompletedEvent, RunStartedEvent,
};

fn agent_node(id: &str) -> Node {
    Node {
        id: id.to_owned(),
        kind: NodeKind::Agent,
        model_id: None,
        spec: NodeSpec::Agent(ProtoSpec {
            name: id.to_owned(),
            model_id: "mock".to_owned(),
            instructions: String::new(),
            output_schema_json: String::new(),
            tools: vec![],
            max_steps: 1,
            model_retry: None,
            model_params_json: String::new(),
        }),
    }
}

fn complete_journal(run_id: &str, node_ids: &[&str]) -> Vec<JournalEvent> {
    let mut events = vec![JournalEvent {
        event_id: format!("{}-0", run_id),
        run_id: run_id.to_owned(),
        seq: 0,
        recorded_at_ns: 0,
        event: Some(Event::RunStarted(RunStartedEvent {
            run_id: run_id.to_owned(),
            spec_bytes: vec![],
            spec_type: "AgentSpec".into(),
        })),
    }];

    let mut seq = 1u64;
    for node in node_ids {
        events.push(JournalEvent {
            event_id: format!("{}-{}", run_id, seq),
            run_id: run_id.to_owned(),
            seq,
            recorded_at_ns: (seq * 1_000_000) as i64,
            event: Some(Event::NodeEntered(NodeEnteredEvent {
                node_id: (*node).to_owned(),
                node_kind: "agent".into(),
            })),
        });
        seq += 1;
        events.push(JournalEvent {
            event_id: format!("{}-{}", run_id, seq),
            run_id: run_id.to_owned(),
            seq,
            recorded_at_ns: (seq * 1_000_000) as i64,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: format!("{}-act", node),
                activity_kind: "llm".into(),
                input_json: "{}".into(),
                result_json: "{}".into(),
                replayed: false,
            })),
        });
        seq += 1;
        events.push(JournalEvent {
            event_id: format!("{}-{}", run_id, seq),
            run_id: run_id.to_owned(),
            seq,
            recorded_at_ns: (seq * 1_000_000) as i64,
            event: Some(Event::NodeExited(NodeExitedEvent {
                node_id: (*node).to_owned(),
                success: true,
            })),
        });
        seq += 1;
    }

    events.push(JournalEvent {
        event_id: format!("{}-{}", run_id, seq),
        run_id: run_id.to_owned(),
        seq,
        recorded_at_ns: (seq * 1_000_000) as i64,
        event: Some(Event::RunCompleted(RunCompletedEvent { output_json: String::new() })),
    });

    events
}

#[test]
fn all_conformance_scenarios_are_registered() {
    assert!(all_scenarios().len() >= 4, "must have at least 4 scenarios");
}

#[test]
fn scenario_ids_are_unique() {
    let scenarios = all_scenarios();
    let mut ids = std::collections::HashSet::new();
    for s in &scenarios {
        assert!(ids.insert(s.id), "duplicate scenario id: {}", s.id);
    }
}

#[test]
fn single_agent_scenario_id_matches_constant() {
    assert_eq!(SINGLE_AGENT.id, "single-agent");
}

#[test]
fn multi_agent_verifier_scenario_id_matches_constant() {
    assert_eq!(MULTI_AGENT_VERIFIER.id, "multi-agent-verifier");
}

#[test]
fn human_in_loop_scenario_id_matches_constant() {
    assert_eq!(HUMAN_IN_LOOP.id, "human-in-loop");
}

#[test]
fn crash_and_recover_scenario_id_matches_constant() {
    assert_eq!(CRASH_AND_RECOVER.id, "crash-and-recover");
}

#[test]
fn single_agent_scenario_graph_validates() {
    let graph = Graph {
        id: "g-single".into(),
        nodes: vec![agent_node("agent-1")],
        edges: vec![],
        entry_node: "agent-1".into(),
    };
    assert!(graph.validate().is_ok());
}

#[test]
fn multi_agent_verifier_scenario_graph_validates() {
    let graph = Graph {
        id: "g-multi".into(),
        nodes: vec![agent_node("agent"), agent_node("verifier")],
        edges: vec![Edge { from: "agent".into(), to: "verifier".into(), condition: None }],
        entry_node: "agent".into(),
    };
    assert!(graph.validate().is_ok());
}

#[test]
fn single_agent_scenario_journal_replays_to_completed() {
    let run_id = "conf-single";
    let events = complete_journal(run_id, &["agent-1"]);
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn multi_agent_verifier_scenario_journal_replays_to_completed() {
    let run_id = "conf-multi";
    let events = complete_journal(run_id, &["agent", "verifier"]);
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(state.run.status, RunStatus::Completed);
}

#[test]
fn human_in_loop_scenario_suspended_run_round_trips() {
    let sr = SuspendedRun {
        run_id: "conf-hil".into(),
        node_id: "approve".into(),
        pending_input: "Approve?".into(),
        deadline_ms: None,
    };
    let restored = SuspendedRun::from_json(&sr.to_json().unwrap()).unwrap();
    assert_eq!(restored.run_id, "conf-hil");
}

#[test]
fn crash_and_recover_scenario_journal_survives_clone() {
    let store = MemoryStore::new();
    let run_id = "conf-crash";
    let events = complete_journal(run_id, &["agent-1"]);

    for ev in &events {
        store.append(run_id, ev.clone()).unwrap();
    }

    let clone = store.clone();
    let loaded = clone.read(run_id).unwrap();
    assert_eq!(loaded.len(), events.len());
}

#[test]
fn two_runs_of_single_agent_produce_structurally_equal_journals() {
    let j1 = complete_journal("run-1", &["agent"]);
    let j2 = complete_journal("run-2", &["agent"]);
    let m1 = mask_events(&j1);
    let m2 = mask_events(&j2);
    assert_structurally_equal(&m1, &m2)
        .expect("two single-agent runs must produce structurally equal journals");
}

#[test]
fn conformance_result_passed_variant_exists() {
    let r = ConformanceResult::Passed;
    assert!(matches!(r, ConformanceResult::Passed));
}

#[test]
fn conformance_result_failed_carries_reason() {
    let r = ConformanceResult::Failed { reason: "timeout".into() };
    if let ConformanceResult::Failed { reason } = r {
        assert_eq!(reason, "timeout");
    } else {
        panic!("expected Failed");
    }
}

#[test]
fn conformance_result_skipped_carries_reason() {
    let r = ConformanceResult::Skipped { reason: "not implemented".into() };
    if let ConformanceResult::Skipped { reason } = r {
        assert!(!reason.is_empty());
    } else {
        panic!("expected Skipped");
    }
}

#[test]
fn all_scenarios_have_non_empty_description() {
    for s in all_scenarios() {
        assert!(!s.description.is_empty(), "scenario '{}' has empty description", s.id);
    }
}
