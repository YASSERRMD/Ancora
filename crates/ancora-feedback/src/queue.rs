/// A run entry in the low-confidence review queue.
#[derive(Debug, Clone)]
pub struct QueueEntry {
    /// The run identifier.
    pub run_id: String,
    /// The confidence score (0.0 to 1.0) that triggered queueing.
    pub confidence: f64,
    /// Whether this entry has been claimed by a reviewer.
    pub claimed: bool,
}

/// Queue for runs that fall below the confidence threshold.
#[derive(Debug, Default)]
pub struct ReviewQueue {
    entries: Vec<QueueEntry>,
    threshold: f64,
}

impl ReviewQueue {
    /// Create a new queue with the given confidence threshold.
    pub fn with_threshold(threshold: f64) -> Self {
        Self {
            entries: Vec::new(),
            threshold,
        }
    }

    /// Submit a run for queueing. Returns true if the run was added (confidence below threshold).
    pub fn submit(&mut self, run_id: impl Into<String>, confidence: f64) -> bool {
        if confidence < self.threshold {
            self.entries.push(QueueEntry {
                run_id: run_id.into(),
                confidence,
                claimed: false,
            });
            true
        } else {
            false
        }
    }

    /// Return all pending (unclaimed) entries.
    pub fn pending(&self) -> Vec<&QueueEntry> {
        self.entries.iter().filter(|e| !e.claimed).collect()
    }

    /// Claim an entry for review. Returns true if successfully claimed.
    pub fn claim(&mut self, run_id: &str) -> bool {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.run_id == run_id) {
            if !entry.claimed {
                entry.claimed = true;
                return true;
            }
        }
        false
    }

    /// Total entries in the queue (pending and claimed).
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true if the queue has no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn low_confidence_is_queued() {
        let mut q = ReviewQueue::with_threshold(0.7);
        let added = q.submit("run-42", 0.4);
        assert!(added);
        assert_eq!(q.len(), 1);
    }

    #[test]
    fn high_confidence_not_queued() {
        let mut q = ReviewQueue::with_threshold(0.7);
        let added = q.submit("run-99", 0.9);
        assert!(!added);
        assert!(q.is_empty());
    }
}
