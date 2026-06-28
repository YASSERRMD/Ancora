use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DetectionSource {
    Siem,
    Edr,
    Ids,
    NetworkMonitor,
    ManualReview,
    HoneyToken,
}

impl fmt::Display for DetectionSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            DetectionSource::Siem => "SIEM",
            DetectionSource::Edr => "EDR",
            DetectionSource::Ids => "IDS",
            DetectionSource::NetworkMonitor => "NETWORK_MONITOR",
            DetectionSource::ManualReview => "MANUAL_REVIEW",
            DetectionSource::HoneyToken => "HONEY_TOKEN",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct DetectionEvent {
    pub id: String,
    pub scenario_id: String,
    pub step_id: Option<String>,
    pub source: DetectionSource,
    pub description: String,
    pub tick: u64,
    pub true_positive: bool,
}

impl DetectionEvent {
    pub fn new(
        id: impl Into<String>,
        scenario_id: impl Into<String>,
        source: DetectionSource,
        description: impl Into<String>,
        tick: u64,
        true_positive: bool,
    ) -> Self {
        Self {
            id: id.into(),
            scenario_id: scenario_id.into(),
            step_id: None,
            source,
            description: description.into(),
            tick,
            true_positive,
        }
    }

    pub fn with_step(mut self, step_id: impl Into<String>) -> Self {
        self.step_id = Some(step_id.into());
        self
    }
}

pub struct DetectionLog {
    events: Vec<DetectionEvent>,
}

impl DetectionLog {
    pub fn new() -> Self { Self { events: Vec::new() } }
    pub fn record(&mut self, event: DetectionEvent) { self.events.push(event); }
    pub fn count(&self) -> usize { self.events.len() }
    pub fn true_positives(&self) -> Vec<&DetectionEvent> {
        self.events.iter().filter(|e| e.true_positive).collect()
    }
    pub fn false_positives(&self) -> Vec<&DetectionEvent> {
        self.events.iter().filter(|e| !e.true_positive).collect()
    }
    pub fn for_scenario<'a>(&'a self, scenario_id: &str) -> Vec<&'a DetectionEvent> {
        self.events.iter().filter(|e| e.scenario_id == scenario_id).collect()
    }
    pub fn by_source<'a>(&'a self, source: &DetectionSource) -> Vec<&'a DetectionEvent> {
        self.events.iter().filter(|e| &e.source == source).collect()
    }
    pub fn detection_rate(&self) -> f64 {
        if self.events.is_empty() { return 0.0; }
        self.true_positives().len() as f64 / self.events.len() as f64
    }
    pub fn all(&self) -> impl Iterator<Item = &DetectionEvent> { self.events.iter() }
}
