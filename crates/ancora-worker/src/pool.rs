use ancora_controlplane::store::ControlPlaneStore;
use crate::executor::{StepFn, WorkerExecutor};
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PoolError {
    #[error("pool already shut down")]
    ShutDown,
}

pub struct WorkerPool {
    store: Arc<Mutex<ControlPlaneStore>>,
    executors: Vec<WorkerExecutor>,
    concurrency_per_worker: usize,
    draining: bool,
}

impl WorkerPool {
    pub fn new(store: ControlPlaneStore, worker_count: usize, concurrency: usize) -> Self {
        let store = Arc::new(Mutex::new(store));
        let mut executors = Vec::with_capacity(worker_count);
        for _ in 0..worker_count {
            let wid = {
                let mut s = store.lock().unwrap();
                let w = s.register_worker(concurrency);
                w.id
            };
            executors.push(WorkerExecutor::new(
                wid,
                Arc::clone(&store),
                Box::new(|_run| Ok("no-op step".to_string())),
            ));
        }
        WorkerPool {
            store,
            executors,
            concurrency_per_worker: concurrency,
            draining: false,
        }
    }

    pub fn with_step_fn(mut self, f: StepFn) -> Self {
        let sf = Arc::new(f);
        let store = Arc::clone(&self.store);
        self.executors = self
            .executors
            .iter()
            .map(|e| {
                let wid = e.worker_id().to_string();
                let sf = Arc::clone(&sf);
                WorkerExecutor::new(
                    wid,
                    Arc::clone(&store),
                    Box::new(move |run| sf(run)),
                )
            })
            .collect();
        self
    }

    pub fn tick(&mut self) -> Vec<Result<Option<String>, String>> {
        if self.draining {
            return vec![];
        }
        self.executors
            .iter()
            .map(|e| {
                e.claim_and_execute()
                    .map(|r| r.map(|id| id))
                    .map_err(|e| e.to_string())
            })
            .collect()
    }

    pub fn start_drain(&mut self) {
        self.draining = true;
    }

    pub fn is_idle(&self) -> bool {
        self.store.lock().unwrap().queue_depth() == 0
    }

    pub fn expire_leases(&mut self) {
        self.store.lock().unwrap().expire_leases();
    }

    pub fn store(&self) -> Arc<Mutex<ControlPlaneStore>> {
        Arc::clone(&self.store)
    }
}
