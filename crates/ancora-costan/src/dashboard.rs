/// Cost dashboard - aggregates all dimensions into a JSON-serializable summary.
use crate::{
    anomaly::AnomalyAlert, by_capability::CapabilityCostBreakdown, by_model::ModelCostBreakdown,
    by_provider::ProviderCostBreakdown, by_tenant::TenantCostBreakdown, by_tool::ToolCostBreakdown,
    cache_savings::CacheSavingsTracker, suggestions::Suggestion, timeseries::CostTimeSeries,
};

/// A simple key-value pair for JSON output.
#[derive(Debug, Clone)]
pub struct KV {
    pub key: String,
    pub value: String,
}

impl KV {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DashboardSnapshot {
    pub total_cost_usd: f64,
    pub period_label: String,
    pub top_models: Vec<(String, f64)>,
    pub top_providers: Vec<(String, f64)>,
    pub top_tools: Vec<(String, f64)>,
    pub top_tenants: Vec<(String, f64)>,
    pub top_capabilities: Vec<(String, f64)>,
    pub cache_hit_rate: f64,
    pub cache_savings_usd: f64,
    pub anomalies: Vec<String>,
    pub suggestions: Vec<String>,
    pub hourly_buckets: Vec<(u64, f64)>,
}

impl DashboardSnapshot {
    /// Render snapshot as a minimal JSON string (no external deps).
    pub fn to_json(&self) -> String {
        let mut out = String::from("{\n");
        out.push_str(&format!(
            "  \"total_cost_usd\": {:.6},\n",
            self.total_cost_usd
        ));
        out.push_str(&format!("  \"period\": \"{}\",\n", self.period_label));
        out.push_str(&format!(
            "  \"cache_hit_rate\": {:.4},\n",
            self.cache_hit_rate
        ));
        out.push_str(&format!(
            "  \"cache_savings_usd\": {:.6},\n",
            self.cache_savings_usd
        ));

        // top_models
        out.push_str("  \"top_models\": [\n");
        for (i, (m, c)) in self.top_models.iter().enumerate() {
            let comma = if i + 1 < self.top_models.len() {
                ","
            } else {
                ""
            };
            out.push_str(&format!(
                "    {{\"model\": \"{}\", \"cost\": {:.6}}}{}",
                m, c, comma
            ));
            out.push('\n');
        }
        out.push_str("  ],\n");

        // top_providers
        out.push_str("  \"top_providers\": [\n");
        for (i, (p, c)) in self.top_providers.iter().enumerate() {
            let comma = if i + 1 < self.top_providers.len() {
                ","
            } else {
                ""
            };
            out.push_str(&format!(
                "    {{\"provider\": \"{}\", \"cost\": {:.6}}}{}",
                p, c, comma
            ));
            out.push('\n');
        }
        out.push_str("  ],\n");

        // anomalies
        out.push_str("  \"anomalies\": [\n");
        for (i, a) in self.anomalies.iter().enumerate() {
            let comma = if i + 1 < self.anomalies.len() {
                ","
            } else {
                ""
            };
            out.push_str(&format!("    \"{}\"{}", a.replace('"', "\\\""), comma));
            out.push('\n');
        }
        out.push_str("  ],\n");

        // suggestions
        out.push_str("  \"suggestions\": [\n");
        for (i, s) in self.suggestions.iter().enumerate() {
            let comma = if i + 1 < self.suggestions.len() {
                ","
            } else {
                ""
            };
            out.push_str(&format!("    \"{}\"{}", s.replace('"', "\\\""), comma));
            out.push('\n');
        }
        out.push_str("  ]\n");

        out.push('}');
        out
    }
}

pub struct DashboardBuilder {
    period_label: String,
    timeseries: CostTimeSeries,
    model_breakdown: ModelCostBreakdown,
    provider_breakdown: ProviderCostBreakdown,
    tool_breakdown: ToolCostBreakdown,
    tenant_breakdown: TenantCostBreakdown,
    capability_breakdown: CapabilityCostBreakdown,
    cache_tracker: CacheSavingsTracker,
    anomalies: Vec<AnomalyAlert>,
    suggestions: Vec<Suggestion>,
}

impl DashboardBuilder {
    pub fn new(period_label: impl Into<String>) -> Self {
        Self {
            period_label: period_label.into(),
            timeseries: CostTimeSeries::new(),
            model_breakdown: ModelCostBreakdown::new(),
            provider_breakdown: ProviderCostBreakdown::new(),
            tool_breakdown: ToolCostBreakdown::new(),
            tenant_breakdown: TenantCostBreakdown::new(),
            capability_breakdown: CapabilityCostBreakdown::new(),
            cache_tracker: CacheSavingsTracker::new(),
            anomalies: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    pub fn timeseries_mut(&mut self) -> &mut CostTimeSeries {
        &mut self.timeseries
    }

    pub fn model_mut(&mut self) -> &mut ModelCostBreakdown {
        &mut self.model_breakdown
    }

    pub fn provider_mut(&mut self) -> &mut ProviderCostBreakdown {
        &mut self.provider_breakdown
    }

    pub fn tool_mut(&mut self) -> &mut ToolCostBreakdown {
        &mut self.tool_breakdown
    }

    pub fn tenant_mut(&mut self) -> &mut TenantCostBreakdown {
        &mut self.tenant_breakdown
    }

    pub fn capability_mut(&mut self) -> &mut CapabilityCostBreakdown {
        &mut self.capability_breakdown
    }

    pub fn cache_mut(&mut self) -> &mut CacheSavingsTracker {
        &mut self.cache_tracker
    }

    pub fn add_anomaly(&mut self, alert: AnomalyAlert) {
        self.anomalies.push(alert);
    }

    pub fn add_suggestion(&mut self, suggestion: Suggestion) {
        self.suggestions.push(suggestion);
    }

    pub fn build(self) -> DashboardSnapshot {
        DashboardSnapshot {
            total_cost_usd: self.timeseries.total_cost(),
            period_label: self.period_label,
            top_models: self.model_breakdown.top_models(),
            top_providers: self.provider_breakdown.top_providers(),
            top_tools: self.tool_breakdown.top_tools_by_cost(),
            top_tenants: self.tenant_breakdown.top_tenants(),
            top_capabilities: self.capability_breakdown.top_capabilities(),
            cache_hit_rate: self.cache_tracker.hit_rate(),
            cache_savings_usd: self.cache_tracker.total_savings(),
            anomalies: self
                .anomalies
                .iter()
                .map(|a| a.description.clone())
                .collect(),
            suggestions: self.suggestions.iter().map(|s| s.detail.clone()).collect(),
            hourly_buckets: self.timeseries.hourly_buckets(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dashboard_json_contains_total_cost() {
        let mut builder = DashboardBuilder::new("2025-01");
        builder.timeseries_mut().record(1000, 5.0, 1000);
        builder.timeseries_mut().record(2000, 3.0, 600);
        let snap = builder.build();
        let json = snap.to_json();
        assert!(json.contains("total_cost_usd"));
        assert!((snap.total_cost_usd - 8.0).abs() < 1e-9);
    }
}
