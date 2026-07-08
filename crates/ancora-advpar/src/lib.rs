//! ancora-advpar: cross-language advanced capability parity tests.
//!
//! Provides canonical reference values for all 7 advanced metrics so that
//! language ports (Go, Python, TypeScript, .NET, Java) can validate parity
//! against these Rust reference implementations.

#[cfg(test)]
mod tests {
    mod a2a_handoff;
    mod airgap_parity;
    mod coordination_parity;
    mod cost_parity;
    mod examples_run;
    mod guardrails_parity;
    mod journals_parity;
    mod lh_parity;
    mod memory_parity;
    mod otel_parity;
    mod planner_parity;
    mod reasoning_parity;
    mod reflection_parity;
    mod routing_parity;
    mod rust_go_batch;
    mod skills_parity;
    mod toolsynth_parity;
    mod ts_dotnet_java_batch;
}
