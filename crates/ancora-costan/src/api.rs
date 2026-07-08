/// Cost analytics public API - entry point for querying aggregated cost data.
use crate::{
    anomaly::{AnomalyAlert, AnomalyDetector},
    by_capability::{Capability, CapabilityCostBreakdown},
    by_model::ModelCostBreakdown,
    by_provider::ProviderCostBreakdown,
    by_tenant::TenantCostBreakdown,
    by_tool::ToolCostBreakdown,
    cache_savings::CacheSavingsTracker,
    dashboard::{DashboardBuilder, DashboardSnapshot},
    forecast::{CostForecaster, ForecastPoint},
    suggestions::{Suggestion, SuggestionEngine},
    timeseries::CostTimeSeries,
};

/// A single cost event fed into the analytics engine.
#[derive(Debug, Clone)]
pub struct CostEvent {
    pub timestamp: u64,
    pub cost_usd: f64,
    pub tokens: u64,
    pub model: String,
    pub provider: String,
    pub tool: Option<String>,
    pub tenant_id: String,
    pub project_id: String,
    pub capability: Capability,
    pub cache_hit: bool,
    /// Cost that would have been paid on cache miss (relevant only when cache_hit = true).
    pub full_cost_if_miss: f64,
    /// Actual cost paid for a cache hit (often much less).
    pub actual_cache_cost: f64,
}

/// Primary analytics engine - ingests events and provides query methods.
pub struct CostAnalytics {
    timeseries: CostTimeSeries,
    model_breakdown: ModelCostBreakdown,
    provider_breakdown: ProviderCostBreakdown,
    tool_breakdown: ToolCostBreakdown,
    tenant_breakdown: TenantCostBreakdown,
    capability_breakdown: CapabilityCostBreakdown,
    cache_tracker: CacheSavingsTracker,
    anomaly_detector: AnomalyDetector,
    forecaster: CostForecaster,
    period_counter: u32,
}

impl CostAnalytics {
    pub fn new(anomaly_threshold: f64) -> Self {
        Self {
            timeseries: CostTimeSeries::new(),
            model_breakdown: ModelCostBreakdown::new(),
            provider_breakdown: ProviderCostBreakdown::new(),
            tool_breakdown: ToolCostBreakdown::new(),
            tenant_breakdown: TenantCostBreakdown::new(),
            capability_breakdown: CapabilityCostBreakdown::new(),
            cache_tracker: CacheSavingsTracker::new(),
            anomaly_detector: AnomalyDetector::new(anomaly_threshold),
            forecaster: CostForecaster::new(),
            period_counter: 0,
        }
    }

    /// Ingest a cost event and update all breakdowns.
    pub fn ingest(&mut self, event: CostEvent) -> Option<AnomalyAlert> {
        let cost = event.cost_usd;

        self.timeseries.record(event.timestamp, cost, event.tokens);
        self.model_breakdown
            .record(&event.model, cost, event.tokens);
        self.provider_breakdown.record(&event.provider, cost);
        self.tenant_breakdown
            .record(&event.tenant_id, &event.project_id, cost);
        self.capability_breakdown.record(event.capability, cost);

        if let Some(tool) = &event.tool {
            self.tool_breakdown.record(tool, cost);
        }

        if event.cache_hit {
            self.cache_tracker.record_hit(
                event.full_cost_if_miss,
                event.actual_cache_cost,
                event.tokens,
            );
        } else {
            self.cache_tracker.record_miss(cost, event.tokens);
        }

        // Check for anomaly.
        let alert = self.anomaly_detector.observe(event.timestamp, cost);

        // Add to forecaster periodically.
        self.period_counter += 1;
        if self.period_counter % 10 == 0 {
            self.forecaster
                .add_observation(self.period_counter / 10, self.timeseries.total_cost());
        }

        alert
    }

    pub fn total_cost(&self) -> f64 {
        self.timeseries.total_cost()
    }

    pub fn top_models(&self) -> Vec<(String, f64)> {
        self.model_breakdown.top_models()
    }

    pub fn top_providers(&self) -> Vec<(String, f64)> {
        self.provider_breakdown.top_providers()
    }

    pub fn top_tools(&self) -> Vec<(String, f64)> {
        self.tool_breakdown.top_tools_by_cost()
    }

    pub fn cache_hit_rate(&self) -> f64 {
        self.cache_tracker.hit_rate()
    }

    pub fn cache_savings(&self) -> f64 {
        self.cache_tracker.total_savings()
    }

    pub fn forecast_next_n(&self, from_period: u32, n: u32) -> Vec<ForecastPoint> {
        self.forecaster.forecast_next_n(from_period, n)
    }

    pub fn generate_suggestions(&self) -> Vec<Suggestion> {
        let model_costs = self.model_breakdown.top_models();
        let tool_costs = self.tool_breakdown.top_tools_by_cost();
        SuggestionEngine::analyze(
            &model_costs,
            self.cache_hit_rate(),
            &tool_costs,
            self.total_cost(),
        )
    }

    /// Build a dashboard snapshot for the given period label.
    pub fn snapshot(&self, period_label: &str) -> DashboardSnapshot {
        let suggestions = self.generate_suggestions();
        let mut builder = DashboardBuilder::new(period_label);

        // Feed timeseries data.
        for point in self.timeseries.points() {
            builder
                .timeseries_mut()
                .record(point.timestamp, point.cost_usd, point.tokens);
        }

        // Feed model data.
        for (model, cost) in self.model_breakdown.top_models() {
            builder.model_mut().record(&model, cost, 0);
        }

        // Feed provider data.
        for (provider, _) in self.provider_breakdown.top_providers() {
            builder
                .provider_mut()
                .record(&provider, self.provider_breakdown.cost_for(&provider));
        }

        // Feed tool data.
        for (tool, cost) in self.tool_breakdown.top_tools_by_cost() {
            builder.tool_mut().record(&tool, cost);
        }

        // Feed tenant data.
        for (tenant, cost) in self.tenant_breakdown.top_tenants() {
            builder.tenant_mut().record(&tenant, "default", cost);
        }

        // Feed capability data.
        for (cap, cost) in self.capability_breakdown.top_capabilities() {
            builder
                .capability_mut()
                .record(Capability::from_str(&cap), cost);
        }

        // Feed cache data (re-apply totals via synthetic entries).
        {
            let savings = self.cache_tracker.total_savings();
            let actual = self.cache_tracker.actual_cost();
            if savings > 0.0 {
                builder.cache_mut().record_hit(actual + savings, actual, 0);
            }
        }

        for s in suggestions {
            builder.add_suggestion(s);
        }

        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_event(ts: u64, cost: f64, model: &str) -> CostEvent {
        CostEvent {
            timestamp: ts,
            cost_usd: cost,
            tokens: 100,
            model: model.to_string(),
            provider: "anthropic".to_string(),
            tool: None,
            tenant_id: "t1".to_string(),
            project_id: "p1".to_string(),
            capability: Capability::Generation,
            cache_hit: false,
            full_cost_if_miss: cost,
            actual_cache_cost: 0.0,
        }
    }

    #[test]
    fn ingest_accumulates_cost() {
        let mut analytics = CostAnalytics::new(3.0);
        analytics.ingest(make_event(1000, 1.0, "model-a"));
        analytics.ingest(make_event(2000, 2.0, "model-b"));
        assert!((analytics.total_cost() - 3.0).abs() < 1e-9);
    }

    #[test]
    fn snapshot_json_valid() {
        let mut analytics = CostAnalytics::new(3.0);
        analytics.ingest(make_event(1000, 1.5, "model-a"));
        let snap = analytics.snapshot("2025-01");
        let json = snap.to_json();
        assert!(json.starts_with('{'));
        assert!(json.ends_with('}'));
        assert!(json.contains("total_cost_usd"));
    }
}
