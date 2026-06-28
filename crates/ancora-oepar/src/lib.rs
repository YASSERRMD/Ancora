//! ancora-oepar: Observability and eval parity across all six language SDKs.
//!
//! This crate provides utilities to verify that traces, costs, eval scores,
//! graders, regression gates, drift signals, feedback, redaction, and
//! exporters produce identical results across Rust, Python, TypeScript, Go,
//! Java, and C# agent SDK implementations.

pub mod trace_parity;
pub mod cost_parity;
pub mod eval_parity;
pub mod grader_parity;
pub mod gate_parity;
pub mod drift_parity;
pub mod feedback_parity;
pub mod redact_parity;
pub mod exporter_parity;
pub mod polyglot;

#[cfg(test)]
mod tests {
    mod test_trace_parity;
    mod test_cost_parity;
    mod test_eval_parity;
    mod test_grader_parity;
    mod test_gate_parity;
    mod test_drift_parity;
    mod test_redact_parity;
    mod test_traces_equal;
    mod test_scores_equal;
    mod test_polyglot_stitching;
}
