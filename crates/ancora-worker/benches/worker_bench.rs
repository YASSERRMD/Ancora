use criterion::{criterion_group, criterion_main, Criterion};
use ancora_controlplane::model::RunPriority;
use ancora_controlplane::store::ControlPlaneStore;
use ancora_worker::pool::WorkerPool;
use ancora_worker::lifecycle::run_to_idle;

fn claim_and_execute_throughput(c: &mut Criterion) {
    c.bench_function("claim_execute_100_runs", |b| {
        b.iter(|| {
            let mut store = ControlPlaneStore::new();
            for _ in 0..100 {
                store.create_run("tenant", RunPriority::Normal);
            }
            let mut pool = WorkerPool::new(store, 4, 8);
            run_to_idle(&mut pool, 200);
        });
    });
}

criterion_group!(benches, claim_and_execute_throughput);
criterion_main!(benches);
