use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

pub(crate) struct RunEntry {
    pub events: VecDeque<String>,
}

impl RunEntry {
    pub fn new() -> Self {
        let mut events = VecDeque::new();
        events.push_back("started".into());
        events.push_back("completed".into());
        Self { events }
    }

    pub fn poll(&mut self) -> Option<String> {
        self.events.pop_front()
    }

    pub fn resume(&mut self, decision: &str) {
        self.events.push_back(format!("resumed:{decision}"));
        self.events.push_back("completed".into());
    }
}

#[derive(Default)]
pub(crate) struct RunStore {
    runs: Mutex<HashMap<String, RunEntry>>,
}

impl RunStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&self, id: String) {
        self.runs.lock().unwrap().insert(id, RunEntry::new());
    }

    pub fn poll(&self, id: &str) -> Option<String> {
        self.runs.lock().unwrap().get_mut(id)?.poll()
    }

    pub fn resume(&self, id: &str, decision: &str) -> bool {
        let mut map = self.runs.lock().unwrap();
        if let Some(e) = map.get_mut(id) {
            e.resume(decision);
            true
        } else {
            false
        }
    }

    pub fn event_count(&self, id: &str) -> usize {
        self.runs.lock().unwrap().get(id).map_or(0, |e| e.events.len())
    }

    pub fn contains(&self, id: &str) -> bool {
        self.runs.lock().unwrap().contains_key(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_run_has_two_events() {
        let store = RunStore::new();
        store.insert("r1".into());
        assert_eq!(store.event_count("r1"), 2);
    }

    #[test]
    fn poll_decrements_event_count() {
        let store = RunStore::new();
        store.insert("r2".into());
        store.poll("r2");
        assert_eq!(store.event_count("r2"), 1);
    }
}
