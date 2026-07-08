use crate::indicator::ThreatLevel;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlertStatus {
    Open,
    Acknowledged,
    Suppressed,
    Closed,
}

impl fmt::Display for AlertStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AlertStatus::Open => "OPEN",
            AlertStatus::Acknowledged => "ACKNOWLEDGED",
            AlertStatus::Suppressed => "SUPPRESSED",
            AlertStatus::Closed => "CLOSED",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct ThreatAlert {
    pub id: String,
    pub tenant_id: String,
    pub indicator_id: String,
    pub threat_level: ThreatLevel,
    pub status: AlertStatus,
    pub message: String,
    pub tick: u64,
}

impl ThreatAlert {
    pub fn new(
        id: impl Into<String>,
        tenant_id: impl Into<String>,
        indicator_id: impl Into<String>,
        threat_level: ThreatLevel,
        message: impl Into<String>,
        tick: u64,
    ) -> Self {
        Self {
            id: id.into(),
            tenant_id: tenant_id.into(),
            indicator_id: indicator_id.into(),
            threat_level,
            status: AlertStatus::Open,
            message: message.into(),
            tick,
        }
    }

    pub fn acknowledge(&mut self) {
        self.status = AlertStatus::Acknowledged;
    }
    pub fn suppress(&mut self) {
        self.status = AlertStatus::Suppressed;
    }
    pub fn close(&mut self) {
        self.status = AlertStatus::Closed;
    }
    pub fn is_open(&self) -> bool {
        self.status == AlertStatus::Open
    }
}

pub struct AlertStore {
    alerts: Vec<ThreatAlert>,
}

impl Default for AlertStore {
    fn default() -> Self {
        Self::new()
    }
}

impl AlertStore {
    pub fn new() -> Self {
        Self { alerts: Vec::new() }
    }
    pub fn add(&mut self, alert: ThreatAlert) {
        self.alerts.push(alert);
    }
    pub fn get_mut(&mut self, id: &str) -> Option<&mut ThreatAlert> {
        self.alerts.iter_mut().find(|a| a.id == id)
    }
    pub fn open(&self) -> Vec<&ThreatAlert> {
        self.alerts.iter().filter(|a| a.is_open()).collect()
    }
    pub fn for_tenant<'a>(&'a self, tenant_id: &str) -> Vec<&'a ThreatAlert> {
        self.alerts
            .iter()
            .filter(|a| a.tenant_id == tenant_id)
            .collect()
    }
    pub fn count(&self) -> usize {
        self.alerts.len()
    }
}
