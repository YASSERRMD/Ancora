use crate::schema::FiredAlert;
use std::collections::HashSet;

/// Deduplication tracker: suppresses repeat firings of the same alert fingerprint
/// within a configurable window.
pub struct AlertDedup {
    seen: HashSet<String>,
    /// Minimum seconds between repeat alerts with the same fingerprint.
    cooldown_secs: u64,
    last_seen: std::collections::HashMap<String, u64>,
}

impl AlertDedup {
    pub fn new(cooldown_secs: u64) -> Self {
        Self {
            seen: HashSet::new(),
            cooldown_secs,
            last_seen: Default::default(),
        }
    }

    /// Returns true if the alert should be routed (not suppressed).
    pub fn should_route(&mut self, alert: &FiredAlert) -> bool {
        let fp = &alert.fingerprint;
        if let Some(&last) = self.last_seen.get(fp) {
            if alert.fired_at_secs < last + self.cooldown_secs {
                return false;
            }
        }
        self.seen.insert(fp.clone());
        self.last_seen.insert(fp.clone(), alert.fired_at_secs);
        true
    }
}
