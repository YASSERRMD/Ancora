pub mod backend;
pub mod cost_view;
pub mod diff;
pub mod drift_view;
pub mod eval_view;
pub mod feedback_view;
pub mod inspector;
pub mod redaction;
pub mod replay;
pub mod run_list;
/// ancora-studio - Local dev studio for rendering runs, traces, evals,
/// costs, drift, and feedback, offline and redaction-aware.
pub mod scaffold;
pub mod timeline;
pub mod trace_tree;

#[cfg(test)]
mod tests;
