//! ancora-advpar: cross-language advanced capability parity tests.
//!
//! Provides canonical reference values for all 7 advanced metrics so that
//! language ports (Go, Python, TypeScript, .NET, Java) can validate parity
//! against these Rust reference implementations.

#[cfg(test)]
mod tests {
    mod planner_parity;
    mod reflection_parity;
    mod routing_parity;
    mod memory_parity;
    mod skills_parity;
    mod coordination_parity;
    mod guardrails_parity;
    mod reasoning_parity;
    mod lh_parity;
    mod toolsynth_parity;
    mod journals_parity;
    mod cost_parity;
    mod otel_parity;
    mod a2a_handoff;
    mod rust_go_batch;
    mod ts_dotnet_java_batch;
    mod examples_run;
    mod airgap_parity;
}
