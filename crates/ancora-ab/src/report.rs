/// Generates a structured experiment report summarising results.
///
/// The report captures the state, per-variant statistics, significance result,
/// and an optional winner at the time of conclusion.

use crate::analysis::SignificanceResult;
use crate::experiment::Experiment;
use crate::lifecycle::{ExperimentState, LifecycleManager};
use crate::outcome::OutcomeStore;

/// Per-variant summary included in the report.
#[derive(Debug, Clone)]
pub struct VariantSummary {
    pub variant_name: String,
    pub n: usize,
    pub mean: f64,
    pub std_dev: f64,
}

/// The complete experiment report.
#[derive(Debug, Clone)]
pub struct ExperimentReport {
    pub experiment_id: String,
    pub experiment_description: String,
    pub metric_name: String,
    pub state_summary: String,
    pub winner: Option<String>,
    pub variants: Vec<VariantSummary>,
    pub significance: Option<SignificanceResult>,
    pub notes: Vec<String>,
}

impl ExperimentReport {
    /// Build a report from the experiment definition, lifecycle, outcome store, and optional
    /// significance result.
    pub fn build(
        experiment: &Experiment,
        lifecycle: &LifecycleManager,
        store: &OutcomeStore,
        significance: Option<SignificanceResult>,
    ) -> Self {
        let state_summary = match &lifecycle.state {
            ExperimentState::Pending => "Pending".to_string(),
            ExperimentState::Running { started_at } => format!("Running (started {})", started_at),
            ExperimentState::Stopped { stopped_at, .. } => {
                format!("Stopped (at {})", stopped_at)
            }
            ExperimentState::Concluded {
                concluded_at,
                winner,
                ..
            } => {
                let w = winner.as_deref().unwrap_or("none (inconclusive)");
                format!("Concluded at {} - winner: {}", concluded_at, w)
            }
        };

        let winner = lifecycle.winner().map(|s| s.to_string());

        let variants = experiment
            .variants
            .iter()
            .map(|v| {
                if let Some(stats) = store.stats_for_variant(&experiment.id, &v.name) {
                    VariantSummary {
                        variant_name: v.name.clone(),
                        n: stats.n,
                        mean: stats.mean,
                        std_dev: stats.std_dev(),
                    }
                } else {
                    VariantSummary {
                        variant_name: v.name.clone(),
                        n: 0,
                        mean: 0.0,
                        std_dev: 0.0,
                    }
                }
            })
            .collect();

        let mut notes = Vec::new();
        if let Some(ref sig) = significance {
            if sig.is_significant {
                notes.push(format!(
                    "Statistically significant at alpha={:.2}: p={:.4}",
                    sig.alpha, sig.p_value
                ));
            } else {
                notes.push(format!(
                    "Not statistically significant at alpha={:.2}: p={:.4}",
                    sig.alpha, sig.p_value
                ));
            }
        }

        ExperimentReport {
            experiment_id: experiment.id.clone(),
            experiment_description: experiment.description.clone(),
            metric_name: experiment.metric.name.clone(),
            state_summary,
            winner,
            variants,
            significance,
            notes,
        }
    }

    /// Render the report as a human-readable string.
    pub fn render(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!("=== Experiment Report: {} ===", self.experiment_id));
        lines.push(format!("Description : {}", self.experiment_description));
        lines.push(format!("Metric      : {}", self.metric_name));
        lines.push(format!("State       : {}", self.state_summary));
        if let Some(ref w) = self.winner {
            lines.push(format!("Winner      : {}", w));
        }
        lines.push(String::from("--- Variants ---"));
        for v in &self.variants {
            lines.push(format!(
                "  {:20} n={:5}  mean={:.4}  std={:.4}",
                v.variant_name, v.n, v.mean, v.std_dev
            ));
        }
        if !self.notes.is_empty() {
            lines.push(String::from("--- Notes ---"));
            for note in &self.notes {
                lines.push(format!("  {}", note));
            }
        }
        lines.join("\n")
    }
}
