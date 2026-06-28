use std::collections::VecDeque;

/// An external signal injected into a long-horizon run.
#[derive(Debug, Clone)]
pub struct ExternalSignal {
    pub run_id: String,
    pub kind: String,
    pub payload: String,
    pub tick: u64,
}

/// Queue of pending external signals for a run.
#[derive(Debug, Default)]
pub struct SignalQueue {
    queue: VecDeque<ExternalSignal>,
}

impl SignalQueue {
    pub fn inject(&mut self, signal: ExternalSignal) {
        self.queue.push_back(signal);
    }

    pub fn pop(&mut self) -> Option<ExternalSignal> {
        self.queue.pop_front()
    }

    pub fn pending(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
