use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use ancora_core::{
    graph::{Edge, Graph, Node, NodeKind, NodeSpec},
    replay::replay_events,
};
use ancora_proto::ancora::{
    journal_event::Event, AgentSpec as ProtoAgentSpec, JournalEvent, NodeEnteredEvent,
    NodeExitedEvent, RunCompletedEvent, RunStartedEvent,
};

fn make_agent_node(id: &str) -> Node {
    Node {
        id: id.to_owned(),
        kind: NodeKind::Agent,
        model_id: None,
        spec: NodeSpec::Agent(ProtoAgentSpec {
            name: id.to_owned(),
            model_id: "mock".into(),
            instructions: String::new(),
            output_schema_json: String::new(),
            tools: vec![],
            max_steps: 1,
            model_retry: None,
            model_params_json: String::new(),
        }),
    }
}

fn build_chain_graph(n: usize) -> Graph {
    let nodes: Vec<Node> = (0..n)
        .map(|i| make_agent_node(&format!("node-{}", i)))
        .collect();
    let edges: Vec<Edge> = (0..n.saturating_sub(1))
        .map(|i| Edge {
            from: format!("node-{}", i),
            to: format!("node-{}", i + 1),
            condition: None,
        })
        .collect();
    Graph {
        id: "bench-graph".into(),
        entry_node: "node-0".into(),
        nodes,
        edges,
    }
}

fn build_journal(run_id: &str, n_nodes: usize) -> Vec<JournalEvent> {
    let mut events = vec![JournalEvent {
        event_id: format!("{}-started", run_id),
        run_id: run_id.to_owned(),
        seq: 0,
        recorded_at_ns: 0,
        event: Some(Event::RunStarted(RunStartedEvent {
            run_id: run_id.to_owned(),
            spec_bytes: vec![],
            spec_type: "AgentSpec".into(),
        })),
    }];

    for i in 0..n_nodes {
        let id = format!("node-{}", i);
        events.push(JournalEvent {
            event_id: format!("{}-entered-{}", run_id, i),
            run_id: run_id.to_owned(),
            seq: (i * 2 + 1) as u64,
            recorded_at_ns: 0,
            event: Some(Event::NodeEntered(NodeEnteredEvent {
                node_id: id.clone(),
                node_kind: "agent".into(),
            })),
        });
        events.push(JournalEvent {
            event_id: format!("{}-exited-{}", run_id, i),
            run_id: run_id.to_owned(),
            seq: (i * 2 + 2) as u64,
            recorded_at_ns: 0,
            event: Some(Event::NodeExited(NodeExitedEvent {
                node_id: id,
                success: true,
            })),
        });
    }

    events.push(JournalEvent {
        event_id: format!("{}-completed", run_id),
        run_id: run_id.to_owned(),
        seq: (n_nodes * 2 + 1) as u64,
        recorded_at_ns: 0,
        event: Some(Event::RunCompleted(RunCompletedEvent {
            output_json: String::new(),
        })),
    });

    events
}

fn bench_graph_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_validation");
    for n in [1, 4, 16, 64] {
        let graph = build_chain_graph(n);
        group.bench_with_input(BenchmarkId::new("nodes", n), &graph, |b, g| {
            b.iter(|| black_box(g.validate()))
        });
    }
    group.finish();
}

fn bench_replay(c: &mut Criterion) {
    let mut group = c.benchmark_group("replay");
    for n in [1, 4, 16, 64] {
        let events = build_journal("bench-run", n);
        group.bench_with_input(BenchmarkId::new("nodes", n), &events, |b, evs| {
            b.iter(|| black_box(replay_events("bench-run", evs)))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_graph_validation, bench_replay);
criterion_main!(benches);
