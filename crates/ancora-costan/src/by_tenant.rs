/// Cost breakdown by tenant and project.
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TenantProject {
    pub tenant_id: String,
    pub project_id: String,
}

impl TenantProject {
    pub fn new(tenant_id: impl Into<String>, project_id: impl Into<String>) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            project_id: project_id.into(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct TenantCostBreakdown {
    costs: HashMap<TenantProject, f64>,
    requests: HashMap<TenantProject, u64>,
}

impl TenantCostBreakdown {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, tenant_id: &str, project_id: &str, cost_usd: f64) {
        let key = TenantProject::new(tenant_id, project_id);
        *self.costs.entry(key.clone()).or_insert(0.0) += cost_usd;
        *self.requests.entry(key).or_insert(0) += 1;
    }

    pub fn cost_for(&self, tenant_id: &str, project_id: &str) -> f64 {
        let key = TenantProject::new(tenant_id, project_id);
        self.costs.get(&key).copied().unwrap_or(0.0)
    }

    pub fn total_cost(&self) -> f64 {
        self.costs.values().sum()
    }

    pub fn tenant_total(&self, tenant_id: &str) -> f64 {
        self.costs
            .iter()
            .filter(|(k, _)| k.tenant_id == tenant_id)
            .map(|(_, v)| v)
            .sum()
    }

    pub fn top_tenants(&self) -> Vec<(String, f64)> {
        let mut agg: HashMap<String, f64> = HashMap::new();
        for (key, cost) in &self.costs {
            *agg.entry(key.tenant_id.clone()).or_insert(0.0) += cost;
        }
        let mut v: Vec<(String, f64)> = agg.into_iter().collect();
        v.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        v
    }

    pub fn projects_for_tenant(&self, tenant_id: &str) -> Vec<(String, f64)> {
        let mut v: Vec<(String, f64)> = self
            .costs
            .iter()
            .filter(|(k, _)| k.tenant_id == tenant_id)
            .map(|(k, v)| (k.project_id.clone(), *v))
            .collect();
        v.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tenant_total_aggregates_projects() {
        let mut b = TenantCostBreakdown::new();
        b.record("tenant-a", "proj-1", 1.0);
        b.record("tenant-a", "proj-2", 2.0);
        b.record("tenant-b", "proj-1", 5.0);
        assert!((b.tenant_total("tenant-a") - 3.0).abs() < 1e-9);
    }
}
