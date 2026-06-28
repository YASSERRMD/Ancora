/// ancora-studio - Local dev studio for rendering runs, traces, evals,
/// costs, drift, and feedback, offline and redaction-aware.

pub mod scaffold;
pub mod run_list;
pub mod timeline;
pub mod inspector;
pub mod trace_tree;
pub mod replay;
pub mod diff;
pub mod eval_view;
pub mod cost_view;
pub mod drift_view;
pub mod feedback_view;
pub mod redaction;
pub mod backend;

#[cfg(test)]
mod tests;
