/// A silence suppresses all alerts within a named maintenance window.
#[derive(Clone, Debug)]
pub struct MaintenanceWindow {
    pub name: String,
    pub start_secs: u64,
    pub end_secs: u64,
    /// If non-empty, only silence alerts whose rule_name matches.
    pub rule_filter: Option<String>,
}

impl MaintenanceWindow {
    pub fn new(
        name: impl Into<String>,
        start_secs: u64,
        end_secs: u64,
        rule_filter: Option<String>,
    ) -> Self {
        Self {
            name: name.into(),
            start_secs,
            end_secs,
            rule_filter,
        }
    }

    pub fn is_active(&self, now: u64) -> bool {
        now >= self.start_secs && now < self.end_secs
    }

    pub fn silences(&self, rule_name: &str, now: u64) -> bool {
        if !self.is_active(now) {
            return false;
        }
        match &self.rule_filter {
            Some(f) => f == rule_name,
            None => true,
        }
    }
}

/// Registry of maintenance windows.
#[derive(Default)]
pub struct SilenceRegistry {
    windows: Vec<MaintenanceWindow>,
}

impl SilenceRegistry {
    pub fn add(&mut self, w: MaintenanceWindow) {
        self.windows.push(w);
    }

    pub fn is_silenced(&self, rule_name: &str, now: u64) -> bool {
        self.windows.iter().any(|w| w.silences(rule_name, now))
    }
}

/// Total active window count at a given time.
impl SilenceRegistry {
    pub fn active_count(&self, now: u64) -> usize {
        self.windows.iter().filter(|w| w.is_active(now)).count()
    }
}
