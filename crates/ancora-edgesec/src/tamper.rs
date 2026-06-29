use std::collections::VecDeque;
use std::fmt;

/// Type of tamper event detected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TamperEventKind {
    HashMismatch,
    UnexpectedReboot,
    UnauthorizedAccess,
    IntegrityViolation,
    ClockSkew,
}

impl fmt::Display for TamperEventKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TamperEventKind::HashMismatch => "HASH_MISMATCH",
            TamperEventKind::UnexpectedReboot => "UNEXPECTED_REBOOT",
            TamperEventKind::UnauthorizedAccess => "UNAUTHORIZED_ACCESS",
            TamperEventKind::IntegrityViolation => "INTEGRITY_VIOLATION",
            TamperEventKind::ClockSkew => "CLOCK_SKEW",
        };
        f.write_str(s)
    }
}

/// A tamper detection event.
#[derive(Debug, Clone)]
pub struct TamperEvent {
    pub device_id: String,
    pub kind: TamperEventKind,
    pub detail: String,
    pub tick: u64,
}

impl TamperEvent {
    pub fn new(
        device_id: impl Into<String>,
        kind: TamperEventKind,
        detail: impl Into<String>,
        tick: u64,
    ) -> Self {
        Self {
            device_id: device_id.into(),
            kind,
            detail: detail.into(),
            tick,
        }
    }
}

/// Tamper detection monitor for edge devices.
pub struct TamperMonitor {
    events: VecDeque<TamperEvent>,
    max_events: usize,
}

impl TamperMonitor {
    pub fn new(max_events: usize) -> Self {
        Self {
            events: VecDeque::new(),
            max_events,
        }
    }

    /// Record a tamper event.
    pub fn record(&mut self, event: TamperEvent) {
        if self.events.len() >= self.max_events {
            self.events.pop_front();
        }
        self.events.push_back(event);
    }

    /// Returns true if any tamper events exist for the given device.
    pub fn is_tampered(&self, device_id: &str) -> bool {
        self.events.iter().any(|e| e.device_id == device_id)
    }

    /// Get all events for a device.
    pub fn events_for(&self, device_id: &str) -> Vec<&TamperEvent> {
        self.events.iter().filter(|e| e.device_id == device_id).collect()
    }

    /// Get all events.
    pub fn all_events(&self) -> impl Iterator<Item = &TamperEvent> {
        self.events.iter()
    }

    /// Total event count.
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Check a data slice against an expected hash (simulated: expected == measured).
    /// Returns a TamperEvent if mismatch is detected.
    pub fn check_hash(
        &mut self,
        device_id: &str,
        component: &str,
        expected: &[u8],
        measured: &[u8],
        tick: u64,
    ) -> bool {
        if expected != measured {
            self.record(TamperEvent::new(
                device_id,
                TamperEventKind::HashMismatch,
                format!("component {} hash mismatch", component),
                tick,
            ));
            false
        } else {
            true
        }
    }
}
