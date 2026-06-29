/// Determinism: property test -- random graphs replay correctly.
/// Generates N random DAG shapes and verifies replay always reaches Completed.
use ancora_core::{
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

fn random_graph_journal(run_id: &str, node_count: usize, seed: u64) -> Vec<JournalEvent> {
    let mut events = vec![
        JournalEvent { event_id: format!("{}-0", run_id), run_id: run_id.into(), seq: 0, recorded_at_ns: 0,
            event: Some(Event::RunStarted(RunStartedEvent { run_id: run_id.into(), spec_bytes: vec![], spec_type: "AgentSpec".into() })) },
    ];
    for i in 0..node_count {
        let node_output = (seed.wrapping_mul((i as u64 + 1).wrapping_mul(6364136223846793005)).wrapping_add(1442695040888963407)) % 1000;
        events.push(JournalEvent {
            event_id: format!("{}-{}", run_id, i+1),
            run_id: run_id.into(),
            seq: (i+1) as u64,
            recorded_at_ns: ((i+1) * 1000) as i64,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: format!("node-{}", i), activity_kind: "compute".into(),
                input_json: format!(r#"{{"node":{},"seed":{}}}"#, i, seed),
                result_json: format!(r#"{{"out":{}}}"#, node_output), replayed: false })) });
    }
    let n = events.len();
    events.push(JournalEvent { event_id: format!("{}-{}", run_id, n), run_id: run_id.into(), seq: n as u64, recorded_at_ns: (n * 1000) as i64,
        event: Some(Event::RunCompleted(RunCompletedEvent { output_json: r#"{"done":true}"#.into() })) });
    events
}

const GRAPH_SEEDS: &[(usize, u64)] = &[(1,1),(3,42),(5,100),(7,999),(10,12345),(2,77),(4,314)];

#[test] fn random_graphs_all_replay_to_completed() {
    for (nodes, seed) in GRAPH_SEEDS {
        let rid = format!("det-graph-{}-{}", nodes, seed);
        let j = random_graph_journal(&rid, *nodes, *seed);
        let store = MemoryStore::new();
        for ev in &j { store.append(&rid, ev.clone()).unwrap(); }
        let status = replay_events(&rid, &store.read(&rid).unwrap()).unwrap().run.status;
        assert_eq!(status, RunStatus::Completed, "failed for nodes={} seed={}", nodes, seed);
    }
}

#[test] fn random_graph_event_count_equals_nodes_plus_two() {
    for (nodes, seed) in GRAPH_SEEDS {
        let j = random_graph_journal(&format!("det-gcnt-{}", seed), *nodes, *seed);
        assert_eq!(j.len(), nodes + 2, "expected nodes+2 for seed={}", seed);
    }
}

#[test] fn random_graph_seq_is_monotonic_for_all_seeds() {
    for (nodes, seed) in GRAPH_SEEDS {
        let j = random_graph_journal(&format!("det-gseq-{}", seed), *nodes, *seed);
        for (i, ev) in j.iter().enumerate() { assert_eq!(ev.seq, i as u64); }
    }
}

#[test] fn different_seeds_produce_different_results_for_same_node_count() {
    let j1 = random_graph_journal("det-gdiff-1", 3, 1);
    let j2 = random_graph_journal("det-gdiff-2", 3, 999);
    let res1: Vec<&str> = j1.iter().filter_map(|e| if let Some(Event::ActivityRecorded(a)) = &e.event { Some(a.result_json.as_str()) } else { None }).collect();
    let res2: Vec<&str> = j2.iter().filter_map(|e| if let Some(Event::ActivityRecorded(a)) = &e.event { Some(a.result_json.as_str()) } else { None }).collect();
    assert_ne!(res1, res2, "different seeds must produce different outputs");
}

#[test] fn same_seed_produces_same_graph() {
    let j1 = random_graph_journal("det-gsame-1", 5, 42);
    let j2 = random_graph_journal("det-gsame-2", 5, 42);
    let keys1: Vec<String> = j1.iter().filter_map(|e| if let Some(Event::ActivityRecorded(a)) = &e.event { Some(a.result_json.clone()) } else { None }).collect();
    let keys2: Vec<String> = j2.iter().filter_map(|e| if let Some(Event::ActivityRecorded(a)) = &e.event { Some(a.result_json.clone()) } else { None }).collect();
    assert_eq!(keys1, keys2);
}
