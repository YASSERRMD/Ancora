use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use ancora_core::{
    activity::Activity,
    error::AncoraError,
    idempotency::{write_once, WriteActivity},
    journal::{JournalStore, MemoryStore},
    output::{repair_prompt, validate_output},
    replay::replay_events,
    routing::ModelRouter,
    stream::{emit_tokens, open_stream},
};
use ancora_proto::ancora::{
    journal_event::Event, ActivityRecordedEvent, JournalEvent, RunCompletedEvent, RunStartedEvent,
};

struct NoopActivity {
    key: String,
}

impl Activity for NoopActivity {
    fn key(&self) -> String { self.key.clone() }
    fn execute(&self) -> Result<String, AncoraError> {
        Ok(r#"{"result":"ok"}"#.into())
    }
}

fn activity_journal(run_id: &str, n: usize) -> Vec<JournalEvent> {
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
    for i in 0..n {
        events.push(JournalEvent {
            event_id: format!("{}-{}", run_id, i + 1),
            run_id: run_id.to_owned(),
            seq: (i + 1) as u64,
            recorded_at_ns: 0,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: format!("act-{}", i),
                activity_kind: "llm".into(),
                input_json: "{}".into(),
                result_json: "{}".into(),
                replayed: false,
            })),
        });
    }
    let last = n + 1;
    events.push(JournalEvent {
        event_id: format!("{}-{}", run_id, last),
        run_id: run_id.to_owned(),
        seq: last as u64,
        recorded_at_ns: 0,
        event: Some(Event::RunCompleted(RunCompletedEvent { output_json: String::new() })),
    });
    events
}

fn bench_activity_record(c: &mut Criterion) {
    let mut group = c.benchmark_group("activity_record");
    for n in [1usize, 10, 100] {
        group.bench_with_input(BenchmarkId::new("activities", n), &n, |b, &n| {
            b.iter(|| {
                let store = MemoryStore::new();
                for i in 0..n {
                    let act = NoopActivity { key: format!("act-{}", i) };
                    let wa = WriteActivity::new(&act).unwrap();
                    black_box(write_once("bench-run", wa, &store).unwrap());
                }
            })
        });
    }
    group.finish();
}

fn bench_replay_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("replay_overhead");
    for n in [1usize, 10, 100] {
        let events = activity_journal("bench-replay", n);
        group.bench_with_input(BenchmarkId::new("activities", n), &events, |b, evs| {
            b.iter(|| black_box(replay_events("bench-replay", evs)))
        });
    }
    group.finish();
}

fn bench_stream_emit(c: &mut Criterion) {
    let mut group = c.benchmark_group("stream_emit");
    for n in [10usize, 100, 1000] {
        let text: String = "x".repeat(n);
        group.bench_with_input(BenchmarkId::new("chars", n), &text, |b, t| {
            b.iter(|| {
                let (tx, rx) = open_stream();
                emit_tokens(&tx, t);
                drop(tx);
                black_box(rx.into_iter().count())
            })
        });
    }
    group.finish();
}

fn bench_output_validate(c: &mut Criterion) {
    let mut group = c.benchmark_group("output_validate");
    let schema = r#"{"type":"object"}"#;
    let valid = r#"{"result":"ok","confidence":0.99}"#;
    let invalid = "not json at all";

    group.bench_function("valid", |b| {
        b.iter(|| black_box(validate_output(black_box(valid), black_box(schema))))
    });
    group.bench_function("invalid", |b| {
        b.iter(|| black_box(validate_output(black_box(invalid), black_box(schema))))
    });
    group.finish();
}

fn bench_repair_prompt(c: &mut Criterion) {
    c.bench_function("repair_prompt", |b| {
        b.iter(|| black_box(repair_prompt(black_box("bad output"), black_box("not JSON"))))
    });
}

fn bench_model_router(c: &mut Criterion) {
    let mut group = c.benchmark_group("model_router");

    let mut router = ModelRouter::new("default-model");
    for i in 0..100 {
        router.bind(format!("node-{}", i), format!("model-{}", i % 5));
    }

    group.bench_function("resolve_bound", |b| {
        b.iter(|| black_box(router.resolve("node-42", None)))
    });
    group.bench_function("resolve_unbound", |b| {
        b.iter(|| black_box(router.resolve("node-999", None)))
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_activity_record,
    bench_replay_overhead,
    bench_stream_emit,
    bench_output_validate,
    bench_repair_prompt,
    bench_model_router
);
criterion_main!(benches);
