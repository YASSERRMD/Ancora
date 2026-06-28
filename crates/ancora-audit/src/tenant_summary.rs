use std::collections::HashMap;
use crate::entry::AuditEntry;
use crate::stats::AuditStats;

pub struct TenantSummary {
    pub tenant_id: String,
    pub stats: AuditStats,
}

pub fn summarize_by_tenant(entries: &[&AuditEntry]) -> Vec<TenantSummary> {
    let mut groups: HashMap<&str, Vec<&AuditEntry>> = HashMap::new();
    for e in entries { groups.entry(&e.tenant_id).or_default().push(e); }
    let mut summaries: Vec<TenantSummary> = groups.into_iter().map(|(tenant_id, group)| {
        TenantSummary {
            tenant_id: tenant_id.to_string(),
            stats: AuditStats::from_entries(group.into_iter()),
        }
    }).collect();
    summaries.sort_by(|a, b| a.tenant_id.cmp(&b.tenant_id));
    summaries
}
