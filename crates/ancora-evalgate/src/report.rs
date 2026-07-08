/// Gate report comment format.
///
/// Renders the results of a gate evaluation as a Markdown string suitable
/// for posting as a PR comment.
use crate::gate::{GateDecision, MetricVerdict};
use crate::regression::RegressionResult;

/// A complete gate report ready for rendering.
#[derive(Debug, Clone)]
pub struct GateReport {
    pub dataset: String,
    pub decision: GateDecision,
    pub verdicts: Vec<MetricVerdict>,
}

impl GateReport {
    pub fn new(
        dataset: impl Into<String>,
        decision: GateDecision,
        verdicts: Vec<MetricVerdict>,
    ) -> Self {
        Self {
            dataset: dataset.into(),
            decision,
            verdicts,
        }
    }

    /// Render the report as a Markdown comment body.
    pub fn to_markdown(&self) -> String {
        let status = match self.decision {
            GateDecision::Pass => "PASS",
            GateDecision::Fail => "FAIL",
        };
        let badge = match self.decision {
            GateDecision::Pass => "green",
            GateDecision::Fail => "red",
        };

        let mut lines = vec![
            format!("## Eval Gate Report - dataset: `{}`", self.dataset),
            format!("**Status:** `{}` ({})", status, badge),
            String::new(),
            "| Metric | Delta | Threshold | Significant | Blocks |".to_string(),
            "|--------|-------|-----------|-------------|--------|".to_string(),
        ];

        for v in &self.verdicts {
            let (delta, threshold) = match &v.regression {
                RegressionResult::Improvement { delta } => (*delta, "-".to_string()),
                RegressionResult::WithinThreshold { delta } => (*delta, "ok".to_string()),
                RegressionResult::Regression { delta, threshold } => {
                    (*delta, format!("{:.4}", threshold))
                }
            };
            let sig = if v.significant { "yes" } else { "no" };
            let blocks = if v.blocks {
                ":x: yes"
            } else {
                ":white_check_mark: no"
            };
            lines.push(format!(
                "| {} | {:.4} | {} | {} | {} |",
                v.metric, delta, threshold, sig, blocks
            ));
        }

        lines.push(String::new());
        if self.decision == GateDecision::Fail {
            lines.push(
                "> One or more metrics regressed significantly. Please investigate before merging."
                    .to_string(),
            );
        } else {
            lines.push("> All metrics are within acceptable bounds.".to_string());
        }

        lines.join("\n")
    }
}
