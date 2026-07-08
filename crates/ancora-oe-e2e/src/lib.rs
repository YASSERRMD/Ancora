pub mod ab_e2e;
pub mod conteval_e2e;
pub mod cost_e2e;
pub mod drift_e2e;
pub mod eval_e2e;
pub mod feedback_e2e;
pub mod gate_e2e;
pub mod perf;
pub mod plan;
pub mod privacy_e2e;
pub mod safety_e2e;
pub mod studio_e2e;
/// ancora-oe-e2e: End-to-end tests for the observability and eval stack.
/// All tests run offline without network calls.
pub mod trace_e2e;

#[cfg(test)]
mod tests {
    mod test_ab_concludes;
    mod test_all_offline;
    mod test_complete_trace;
    mod test_conteval_tracks;
    mod test_cost_reflects;
    mod test_cross_lang;
    mod test_determinism;
    mod test_drift_detected;
    mod test_eval_scores;
    mod test_feedback_feeds;
    mod test_gate_blocks;
    mod test_perf;
    mod test_redaction_holds;
    mod test_residency;
    mod test_safety_flags;
    mod test_trace_exports;
    mod test_zero_pii;
}
