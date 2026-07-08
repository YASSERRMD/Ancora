//! ancora-advdet: determinism and replay proofs for all Ancora advanced capabilities.
//!
//! Each module contains tests that prove a specific advanced capability produces
//! identical output when replayed with identical inputs, using only in-memory
//! stores and abstract u64 ticks (no wall-clock, no network).

#[cfg(test)]
mod tests {
    mod coord_replay;
    mod cost_replay;
    mod divergence;
    mod guard_replay;
    mod lang_parity;
    mod lh_wakeup_replay;
    mod memory_determinism;
    mod no_network;
    mod opt_artifact;
    mod otel_replay;
    mod perf_replay;
    mod planner_determinism;
    mod rand_seed;
    mod reason_replay;
    mod reflection_determinism;
    mod restart_replay;
    mod routing_determinism;
    mod skills_replay;
    mod toolsynth_replay;
}
