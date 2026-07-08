use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use ancora_core::journal::{JournalStore, MemoryStore};
use ancora_proto::ancora::{journal_event::Event, JournalEvent, RunStartedEvent};

fn make_event(run_id: &str, seq: u64) -> JournalEvent {
    JournalEvent {
        event_id: format!("{}-{}", run_id, seq),
        run_id: run_id.to_owned(),
        seq,
        recorded_at_ns: 0,
        event: Some(Event::RunStarted(RunStartedEvent {
            run_id: run_id.to_owned(),
            spec_bytes: vec![],
            spec_type: "AgentSpec".into(),
        })),
    }
}

fn bench_journal_append(c: &mut Criterion) {
    let mut group = c.benchmark_group("journal_append");
    for n in [10u64, 100, 1000, 10_000] {
        group.bench_with_input(BenchmarkId::new("events", n), &n, |b, &n| {
            b.iter(|| {
                let store = MemoryStore::new();
                for i in 0..n {
                    store
                        .append("bench-run", make_event("bench-run", i))
                        .unwrap();
                }
                black_box(store.read("bench-run").unwrap().len())
            })
        });
    }
    group.finish();
}

fn bench_journal_read(c: &mut Criterion) {
    let mut group = c.benchmark_group("journal_read");
    for n in [10usize, 100, 1000] {
        let store = MemoryStore::new();
        for i in 0..n {
            store.append("r", make_event("r", i as u64)).unwrap();
        }
        group.bench_with_input(BenchmarkId::new("events", n), &store, |b, s| {
            b.iter(|| black_box(s.read("r").unwrap()))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_journal_append, bench_journal_read);
criterion_main!(benches);
