/// A single recorded coordination event for replay.
#[derive(Debug, Clone)]
pub struct CoordEvent {
    pub tick: u64,
    pub kind: String,
    pub description: String,
}

/// Append-only journal of coordination events.
#[derive(Debug, Default)]
pub struct CoordJournal {
    events: Vec<CoordEvent>,
}

impl CoordJournal {
    pub fn record(&mut self, tick: u64, kind: &str, description: &str) {
        self.events.push(CoordEvent {
            tick,
            kind: kind.to_string(),
            description: description.to_string(),
        });
    }

    pub fn events(&self) -> &[CoordEvent] {
        &self.events
    }

    pub fn replay(&self) -> Vec<(&str, &str)> {
        self.events.iter().map(|e| (e.kind.as_str(), e.description.as_str())).collect()
    }
}
