use std::collections::VecDeque;

/// Short-term working memory: a fixed-size ring buffer for the current agent turn.
pub struct WorkingMemory {
    pub capacity: usize,
    buffer: VecDeque<String>,
}

impl WorkingMemory {
    pub fn new(capacity: usize) -> Self {
        Self { capacity, buffer: VecDeque::with_capacity(capacity) }
    }

    pub fn push(&mut self, item: String) {
        if self.buffer.len() >= self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(item);
    }

    pub fn peek_recent(&self, n: usize) -> Vec<&str> {
        self.buffer.iter().rev().take(n).map(|s| s.as_str()).collect()
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}
