use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct RequeueEntry {
    pub run_id: String,
    pub attempt: u32,
    pub earliest_retry_secs: u64,
}

pub struct AutoRequeue {
    pub max_attempts: u32,
    queue: VecDeque<RequeueEntry>,
    counters: std::collections::HashMap<String, u32>,
}

impl AutoRequeue {
    pub fn new(max_attempts: u32) -> Self {
        Self {
            max_attempts,
            queue: VecDeque::new(),
            counters: Default::default(),
        }
    }

    pub fn enqueue(&mut self, run_id: &str, now: u64, backoff_secs: u64) -> bool {
        let attempt = self.counters.entry(run_id.to_string()).or_insert(0);
        if *attempt >= self.max_attempts {
            return false;
        }
        *attempt += 1;
        let attempt_num = *attempt;
        self.queue.push_back(RequeueEntry {
            run_id: run_id.to_string(),
            attempt: attempt_num,
            earliest_retry_secs: now + backoff_secs,
        });
        true
    }

    pub fn pop_due(&mut self, now: u64) -> Vec<RequeueEntry> {
        let mut due = vec![];
        while let Some(front) = self.queue.front() {
            if front.earliest_retry_secs <= now {
                due.push(self.queue.pop_front().unwrap());
            } else {
                break;
            }
        }
        due
    }

    pub fn pending_count(&self) -> usize {
        self.queue.len()
    }

    pub fn attempts_for(&self, run_id: &str) -> u32 {
        self.counters.get(run_id).copied().unwrap_or(0)
    }
}
