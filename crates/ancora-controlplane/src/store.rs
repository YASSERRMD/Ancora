use crate::model::{
    CostSummary, JournalEntry, ResumeDecision, Run, RunId, RunPriority, RunState, Worker, WorkerId,
};
use crate::pagination::{Page, PageCursor};
use chrono::{Duration, Utc};
use std::collections::{BinaryHeap, HashMap, VecDeque};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("run not found: {0}")]
    RunNotFound(RunId),
    #[error("worker not found: {0}")]
    WorkerNotFound(WorkerId),
    #[error("run not in expected state: {0:?}")]
    InvalidState(RunState),
    #[error("lease already held")]
    LeaseConflict,
    #[error("poison run quarantined: {0}")]
    PoisonRun(RunId),
}

#[derive(Debug)]
struct QueuedRun {
    priority: RunPriority,
    created_at: chrono::DateTime<Utc>,
    run_id: RunId,
}

impl PartialEq for QueuedRun {
    fn eq(&self, other: &Self) -> bool {
        self.run_id == other.run_id
    }
}
impl Eq for QueuedRun {}

impl PartialOrd for QueuedRun {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueuedRun {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Higher priority first; earlier created_at breaks ties
        self.priority
            .cmp(&other.priority)
            .then(other.created_at.cmp(&self.created_at))
    }
}

#[derive(Default)]
pub struct ControlPlaneStore {
    pub runs: HashMap<RunId, Run>,
    pub workers: HashMap<WorkerId, Worker>,
    queue: BinaryHeap<QueuedRun>,
    journal: HashMap<RunId, VecDeque<JournalEntry>>,
    fail_counts: HashMap<RunId, usize>,
}

const LEASE_DURATION_SECS: i64 = 30;
const POISON_THRESHOLD: usize = 5;

impl ControlPlaneStore {
    pub fn new() -> Self {
        Self::default()
    }

    // --- Run management ---

    pub fn create_run(&mut self, tenant_id: impl Into<String>, priority: RunPriority) -> Run {
        let run = Run::new(tenant_id, priority);
        self.queue.push(QueuedRun {
            priority: run.priority,
            created_at: run.created_at,
            run_id: run.id.clone(),
        });
        self.runs.insert(run.id.clone(), run.clone());
        run
    }

    pub fn get_run(&self, id: &str) -> Option<&Run> {
        self.runs.get(id)
    }

    pub fn list_runs(
        &self,
        tenant_filter: Option<&str>,
        state_filter: Option<&RunState>,
        cursor: Option<&PageCursor>,
        limit: usize,
    ) -> Page<Run> {
        let mut runs: Vec<&Run> = self
            .runs
            .values()
            .filter(|r| {
                tenant_filter.map_or(true, |t| r.tenant_id == t)
                    && state_filter.map_or(true, |s| &r.state == s)
            })
            .collect();
        runs.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        let start = cursor
            .and_then(|c| runs.iter().position(|r| r.id == c.after_id))
            .map(|i| i + 1)
            .unwrap_or(0);

        let items: Vec<Run> = runs.iter().skip(start).take(limit).map(|r| (*r).clone()).collect();
        let next_cursor = if start + limit < runs.len() {
            items.last().map(|r| PageCursor { after_id: r.id.clone() })
        } else {
            None
        };
        Page { items, next_cursor }
    }

    pub fn cancel_run(&mut self, id: &str) -> Result<(), StoreError> {
        let run = self.runs.get_mut(id).ok_or_else(|| StoreError::RunNotFound(id.to_string()))?;
        match run.state {
            RunState::Queued | RunState::Assigned | RunState::Running | RunState::Paused => {
                run.state = RunState::Cancelled;
                run.updated_at = Utc::now();
                Ok(())
            }
            ref s => Err(StoreError::InvalidState(s.clone())),
        }
    }

    pub fn pause_run(&mut self, id: &str) -> Result<(), StoreError> {
        let run = self.runs.get_mut(id).ok_or_else(|| StoreError::RunNotFound(id.to_string()))?;
        match run.state {
            RunState::Running => {
                run.state = RunState::Paused;
                run.updated_at = Utc::now();
                Ok(())
            }
            ref s => Err(StoreError::InvalidState(s.clone())),
        }
    }

    pub fn resume_run(&mut self, id: &str, decision: ResumeDecision) -> Result<(), StoreError> {
        let run = self.runs.get_mut(id).ok_or_else(|| StoreError::RunNotFound(id.to_string()))?;
        if !decision.approved {
            run.state = RunState::Cancelled;
            run.updated_at = Utc::now();
            return Ok(());
        }
        match run.state {
            RunState::Paused => {
                run.state = RunState::Queued;
                run.updated_at = Utc::now();
                self.queue.push(QueuedRun {
                    priority: run.priority,
                    created_at: run.created_at,
                    run_id: run.id.clone(),
                });
                Ok(())
            }
            ref s => Err(StoreError::InvalidState(s.clone())),
        }
    }

    // --- Worker management ---

    pub fn register_worker(&mut self, concurrency_limit: usize) -> Worker {
        let w = Worker::new(concurrency_limit);
        self.workers.insert(w.id.clone(), w.clone());
        w
    }

    pub fn heartbeat_worker(&mut self, id: &str) -> Result<(), StoreError> {
        let w = self
            .workers
            .get_mut(id)
            .ok_or_else(|| StoreError::WorkerNotFound(id.to_string()))?;
        w.last_heartbeat = Utc::now();
        if let Some(exp) = w.lease_expires_at {
            if Utc::now() < exp {
                w.lease_expires_at = Some(Utc::now() + Duration::seconds(LEASE_DURATION_SECS));
            }
        }
        Ok(())
    }

    pub fn claim_run(&mut self, worker_id: &str) -> Result<Option<Run>, StoreError> {
        let worker = self
            .workers
            .get(worker_id)
            .ok_or_else(|| StoreError::WorkerNotFound(worker_id.to_string()))?;

        if worker.active_count >= worker.concurrency_limit {
            return Ok(None);
        }

        // Drain expired workers
        self.expire_leases();

        // Pop from priority queue until we find a Queued run
        let run_id = loop {
            let entry = match self.queue.pop() {
                Some(e) => e,
                None => return Ok(None),
            };
            let run = match self.runs.get(&entry.run_id) {
                Some(r) => r,
                None => continue,
            };
            if run.state == RunState::Queued {
                break entry.run_id;
            }
        };

        // Check for poison run
        let fails = self.fail_counts.get(&run_id).copied().unwrap_or(0);
        if fails >= POISON_THRESHOLD {
            let run = self.runs.get_mut(&run_id).unwrap();
            run.state = RunState::Quarantined;
            run.updated_at = Utc::now();
            return Err(StoreError::PoisonRun(run_id));
        }

        let now = Utc::now();
        let run = self.runs.get_mut(&run_id).unwrap();
        run.state = RunState::Assigned;
        run.assigned_worker = Some(worker_id.to_string());
        run.updated_at = now;

        let worker = self.workers.get_mut(worker_id).unwrap();
        worker.current_run = Some(run_id.clone());
        worker.lease_expires_at = Some(now + Duration::seconds(LEASE_DURATION_SECS));
        worker.active_count += 1;

        Ok(Some(self.runs[&run_id].clone()))
    }

    pub fn release_lease(&mut self, worker_id: &str, run_id: &str, success: bool) {
        if let Some(w) = self.workers.get_mut(worker_id) {
            if w.active_count > 0 {
                w.active_count -= 1;
            }
            w.current_run = None;
            w.lease_expires_at = None;
        }
        if let Some(run) = self.runs.get_mut(run_id) {
            run.assigned_worker = None;
            run.updated_at = Utc::now();
            if success {
                run.state = RunState::Completed;
            } else {
                *self.fail_counts.entry(run_id.to_string()).or_insert(0) += 1;
                run.state = RunState::Queued;
                self.queue.push(QueuedRun {
                    priority: run.priority,
                    created_at: run.created_at,
                    run_id: run_id.to_string(),
                });
            }
        }
    }

    pub fn expire_leases(&mut self) {
        let now = Utc::now();
        let mut requeued = vec![];
        for w in self.workers.values_mut() {
            if let Some(exp) = w.lease_expires_at {
                if now >= exp {
                    if let Some(ref rid) = w.current_run.clone() {
                        requeued.push(rid.clone());
                    }
                    w.lease_expires_at = None;
                    w.current_run = None;
                    if w.active_count > 0 {
                        w.active_count -= 1;
                    }
                }
            }
        }
        for rid in requeued {
            if let Some(run) = self.runs.get_mut(&rid) {
                if run.state == RunState::Assigned || run.state == RunState::Running {
                    *self.fail_counts.entry(rid.clone()).or_insert(0) += 1;
                    run.state = RunState::Queued;
                    run.assigned_worker = None;
                    run.updated_at = now;
                    self.queue.push(QueuedRun {
                        priority: run.priority,
                        created_at: run.created_at,
                        run_id: rid,
                    });
                }
            }
        }
    }

    // --- Journal tail ---

    pub fn append_journal(&mut self, run_id: &str, payload: String) {
        let entries = self.journal.entry(run_id.to_string()).or_default();
        let seq = entries.len() as u64;
        entries.push_back(JournalEntry {
            run_id: run_id.to_string(),
            seq,
            payload,
            ts: Utc::now(),
        });
        if let Some(r) = self.runs.get_mut(run_id) {
            r.journal_seq = seq;
        }
    }

    pub fn tail_journal(&self, run_id: &str, from_seq: u64) -> Vec<JournalEntry> {
        self.journal
            .get(run_id)
            .map(|entries| {
                entries
                    .iter()
                    .filter(|e| e.seq >= from_seq)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    // --- Cost ---

    pub fn record_cost(&mut self, run_id: &str, tokens: u64, usd_micro: u64) {
        if let Some(r) = self.runs.get_mut(run_id) {
            r.cost_tokens += tokens;
            r.cost_usd_micro += usd_micro;
        }
    }

    pub fn cost_per_run(&self, run_id: &str) -> Option<CostSummary> {
        self.runs.get(run_id).map(|r| CostSummary {
            run_id: Some(r.id.clone()),
            tenant_id: Some(r.tenant_id.clone()),
            total_tokens: r.cost_tokens,
            total_usd_micro: r.cost_usd_micro,
            run_count: 1,
        })
    }

    pub fn cost_aggregate(&self, tenant_id: &str) -> CostSummary {
        let (tokens, usd, count) = self
            .runs
            .values()
            .filter(|r| r.tenant_id == tenant_id)
            .fold((0u64, 0u64, 0usize), |(t, u, c), r| {
                (t + r.cost_tokens, u + r.cost_usd_micro, c + 1)
            });
        CostSummary {
            run_id: None,
            tenant_id: Some(tenant_id.to_string()),
            total_tokens: tokens,
            total_usd_micro: usd,
            run_count: count,
        }
    }

    // --- Health ---

    pub fn worker_count(&self) -> usize {
        self.workers.len()
    }

    pub fn queue_depth(&self) -> usize {
        self.runs
            .values()
            .filter(|r| r.state == RunState::Queued)
            .count()
    }
}
