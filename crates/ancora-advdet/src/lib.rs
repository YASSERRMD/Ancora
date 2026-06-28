//! ancora-advdet: determinism and replay proofs for all Ancora advanced capabilities.
//!
//! Each module contains tests that prove a specific advanced capability produces
//! identical output when replayed with identical inputs, using only in-memory
//! stores and abstract u64 ticks (no wall-clock, no network).

#[cfg(test)]
mod tests {
    mod planner_determinism;
    mod reflection_determinism;
    mod routing_determinism;
    mod opt_artifact;
    mod memory_determinism;
    mod toolsynth_replay;
    mod skills_replay;
    mod lh_wakeup_replay;
    mod coord_replay;
    mod guard_replay;
    mod reason_replay;
    mod restart_replay;
    mod lang_parity;
    mod no_network;
    mod divergence;
    mod cost_replay;
    mod rand_seed;
    mod otel_replay;
    mod perf_replay;
}
