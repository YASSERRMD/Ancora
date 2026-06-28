/// ancora-debug - Trace replay debugging for the Ancora agent framework.
///
/// Runs can be replayed, inspected, diffed, branched, and re-run from any
/// point.  All operations are offline: no live LLM or tool calls are made.
///
/// # Modules
///
/// - [`loader`] - Load and validate a run journal.
/// - [`replay`] - Step-through replay of a loaded journal.
/// - [`inspector`] - Inspect state, prompts, and tool I/O at any seq.
/// - [`diff`] - Diff two run journals to find where they diverge.
/// - [`branch`] - Branch from a seq to explore alternative run paths.
/// - [`annotate`] - Attach developer annotations to specific entries.
/// - [`api`] - High-level debug API surface for the Ancora Studio.

pub mod annotate;
pub mod api;
pub mod branch;
pub mod diff;
pub mod inspector;
pub mod loader;
pub mod replay;

#[cfg(test)]
mod tests {
    mod test_annotations;
    mod test_branch_run;
    mod test_debug_api;
    mod test_no_live_calls;
    mod test_rerun_from_point;
    mod test_run_diff;
    mod test_state_inspection;
    mod test_step_through;
}
