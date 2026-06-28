use crate::indicator::Indicator;
use crate::feed::FeedStore;
use crate::alert::AlertStore;
use crate::audit::ThreatIntelAuditLog;

pub struct ThreatIntelReport {
    pub tenant_id: String,
    pub total_indicators: usize,
    pub active_indicators: usize,
    pub total_feeds: usize,
    pub open_alerts: usize,
    pub audit_entries: usize,
    pub tick: u64,
}

impl ThreatIntelReport {
    pub fn generate(
        indicators: &[&Indicator],
        feeds: &FeedStore,
        alerts: &AlertStore,
        audit: &ThreatIntelAuditLog,
        tenant_id: &str,
        tick: u64,
    ) -> Self {
        let tenant_indicators: Vec<&&Indicator> = indicators.iter().filter(|i| i.tenant_id == tenant_id).collect();
        let total_indicators = tenant_indicators.len();
        let active_indicators = tenant_indicators.iter().filter(|i| i.active).count();
        Self {
            tenant_id: tenant_id.to_string(),
            total_indicators,
            active_indicators,
            total_feeds: feeds.for_tenant(tenant_id).len(),
            open_alerts: alerts.for_tenant(tenant_id).iter().filter(|a| a.is_open()).count(),
            audit_entries: audit.for_tenant(tenant_id).len(),
            tick,
        }
    }
}
