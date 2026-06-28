use crate::indicator::Indicator;
use crate::alert::AlertStore;
use crate::stats::ThreatIntelStats;

pub struct ThreatIntelSummary {
    pub tenant_id: String,
    pub total_indicators: usize,
    pub active_indicators: usize,
    pub critical_count: usize,
    pub open_alerts: usize,
    pub is_healthy: bool,
}

impl ThreatIntelSummary {
    pub fn generate(indicators: &[&Indicator], alerts: &AlertStore, tenant_id: &str) -> Self {
        let stats = ThreatIntelStats::for_tenant(indicators, tenant_id);
        let open_alerts = alerts.for_tenant(tenant_id).into_iter().filter(|a| a.is_open()).count();
        let is_healthy = stats.critical_count == 0 && open_alerts == 0;
        Self {
            tenant_id: tenant_id.to_string(),
            total_indicators: stats.total_indicators,
            active_indicators: stats.active_indicators,
            critical_count: stats.critical_count,
            open_alerts,
            is_healthy,
        }
    }
}
