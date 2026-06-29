//! Edge evaluation report generation.
//!
//! Aggregates results from multiple eval dimensions into a structured report.

use crate::model::{SmallModel, SampleResult};
use crate::runtime::{MemoryFootprint, PowerProxy};

/// Summary of a single model's edge evaluation.
#[derive(Debug, Clone)]
pub struct ModelEvalSummary {
    pub model_name: String,
    pub capability_pass_rate: f64,
    pub mean_latency_ms: f64,
    pub memory_total_mib: f64,
    pub power_tokens_per_joule: f64,
    pub reliability_score: f64,
    pub best_quant_format: Option<String>,
}

impl ModelEvalSummary {
    /// Overall edge score: weighted combination of key metrics.
    pub fn edge_score(&self) -> f64 {
        let cap = self.capability_pass_rate;
        let lat = (1.0 - (self.mean_latency_ms / 10_000.0).min(1.0)).max(0.0);
        let mem = (1.0 - (self.memory_total_mib / 16_384.0).min(1.0)).max(0.0);
        let rel = self.reliability_score;
        // Weights: capability 40%, latency 20%, memory 20%, reliability 20%
        0.4 * cap + 0.2 * lat + 0.2 * mem + 0.2 * rel
    }
}

/// Edge evaluation report aggregating all dimensions.
#[derive(Debug, Default)]
pub struct EdgeEvalReport {
    pub title: String,
    pub summaries: Vec<ModelEvalSummary>,
}

impl EdgeEvalReport {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            summaries: Vec::new(),
        }
    }

    /// Add a model summary.
    pub fn add_summary(&mut self, summary: ModelEvalSummary) {
        self.summaries.push(summary);
    }

    /// Find the best model by edge score.
    pub fn best_model(&self) -> Option<&ModelEvalSummary> {
        self.summaries
            .iter()
            .max_by(|a, b| a.edge_score().partial_cmp(&b.edge_score()).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Render a plain-text summary of the report.
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("# {}\n\n", self.title));
        out.push_str("| Model | Cap% | Lat(ms) | Mem(MiB) | Rel | EdgeScore |\n");
        out.push_str("|-------|------|---------|----------|-----|----------|\n");
        for s in &self.summaries {
            out.push_str(&format!(
                "| {} | {:.1}% | {:.1} | {:.1} | {:.2} | {:.3} |\n",
                s.model_name,
                s.capability_pass_rate * 100.0,
                s.mean_latency_ms,
                s.memory_total_mib,
                s.reliability_score,
                s.edge_score(),
            ));
        }
        if let Some(best) = self.best_model() {
            out.push_str(&format!("\nRecommended: {}\n", best.model_name));
        }
        out
    }

    /// Build a report from raw evaluation data.
    pub fn build_from_results(
        title: impl Into<String>,
        model: &SmallModel,
        capability_results: &[SampleResult],
        footprint: &MemoryFootprint,
        power: &PowerProxy,
        mean_latency_ms: f64,
        reliability_score: f64,
        best_quant_format: Option<String>,
    ) -> Self {
        let pass_rate = crate::model::SmallModelSuite::pass_rate(capability_results);
        let summary = ModelEvalSummary {
            model_name: model.name.clone(),
            capability_pass_rate: pass_rate,
            mean_latency_ms,
            memory_total_mib: footprint.total_mib(),
            power_tokens_per_joule: power.tokens_per_joule(),
            reliability_score,
            best_quant_format,
        };
        let mut report = Self::new(title);
        report.add_summary(summary);
        report
    }
}
