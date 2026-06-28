//! Tool-usage drift detection.
//!
//! Detects when the frequency of tool calls changes significantly relative to
//! the reference distribution - an early signal that the model's behaviour or
//! the workload composition has shifted.

use crate::reference::ReferenceDistribution;
use std::collections::HashMap;

/// Per-tool drift result.
#[derive(Debug, Clone, PartialEq)]
pub struct ToolDriftEntry {
    pub tool: String,
    /// Reference relative frequency.
    pub ref_freq: f64,
    /// Current relative frequency.
    pub cur_freq: f64,
    /// Absolute difference.
    pub abs_diff: f64,
    pub drifted: bool,
}

/// Aggregate result for tool-usage drift.
#[derive(Debug, Clone)]
pub struct ToolDriftResult {
    pub entries: Vec<ToolDriftEntry>,
    /// True if any individual tool shows drift.
    pub any_drifted: bool,
}

/// Detector for changes in tool-call frequency distributions.
#[derive(Debug, Clone)]
pub struct ToolDriftDetector {
    /// Maximum tolerated absolute frequency difference per tool (0.0 - 1.0).
    pub threshold: f64,
}

impl Default for ToolDriftDetector {
    fn default() -> Self {
        Self { threshold: 0.15 }
    }
}

impl ToolDriftDetector {
    pub fn new(threshold: f64) -> Self {
        Self { threshold }
    }

    /// Compare `current_tool_counts` against the reference distribution.
    ///
    /// `current_tool_counts` maps tool name to count of invocations in the
    /// current window.
    pub fn check(
        &self,
        reference: &ReferenceDistribution,
        current_tool_counts: &HashMap<String, usize>,
    ) -> ToolDriftResult {
        let total_cur: usize = current_tool_counts.values().sum();
        let cur_freqs: HashMap<String, f64> = if total_cur == 0 {
            HashMap::new()
        } else {
            current_tool_counts
                .iter()
                .map(|(k, &v)| (k.clone(), v as f64 / total_cur as f64))
                .collect()
        };

        // Union of tool names from reference and current
        let all_tools: std::collections::HashSet<String> = reference
            .tool_frequencies
            .keys()
            .cloned()
            .chain(cur_freqs.keys().cloned())
            .collect();

        let mut entries: Vec<ToolDriftEntry> = all_tools
            .into_iter()
            .map(|tool| {
                let ref_freq = reference.tool_frequencies.get(&tool).cloned().unwrap_or(0.0);
                let cur_freq = cur_freqs.get(&tool).cloned().unwrap_or(0.0);
                let abs_diff = (cur_freq - ref_freq).abs();
                ToolDriftEntry {
                    drifted: abs_diff > self.threshold,
                    tool,
                    ref_freq,
                    cur_freq,
                    abs_diff,
                }
            })
            .collect();

        entries.sort_by(|a, b| a.tool.cmp(&b.tool));
        let any_drifted = entries.iter().any(|e| e.drifted);
        ToolDriftResult { entries, any_drifted }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reference::ReferenceBuilder;

    fn make_ref_with_search() -> ReferenceDistribution {
        let mut b = ReferenceBuilder::new();
        for _ in 0..100 {
            b.add("q", "a", 100, 50, &["search".to_string()], "openai");
        }
        b.build().unwrap()
    }

    #[test]
    fn stable_tool_usage_no_drift() {
        let reference = make_ref_with_search();
        let mut counts = HashMap::new();
        counts.insert("search".to_string(), 10);
        let detector = ToolDriftDetector::new(0.15);
        let result = detector.check(&reference, &counts);
        assert!(!result.any_drifted);
    }

    #[test]
    fn new_tool_appears_causes_drift() {
        let reference = make_ref_with_search();
        let mut counts = HashMap::new();
        // Previously unseen tool dominates
        counts.insert("code_exec".to_string(), 100);
        let detector = ToolDriftDetector::new(0.15);
        let result = detector.check(&reference, &counts);
        assert!(result.any_drifted);
    }
}
