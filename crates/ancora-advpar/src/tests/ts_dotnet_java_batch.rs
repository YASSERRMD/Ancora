// TypeScript, .NET, and Java parity: these ports must produce the same canonical
// values as Rust and Go. This file documents the expected values for each
// language's test suite to compare against.
//
// When porting to TS/dotnet/Java: use these constants in your test assertions.
//
// All values are produced by pure arithmetic with no floating-point accumulation.

use ancora_ageval::{
    CoordinationMetric, GuardrailMetric, MemoryMetric, PlanningMetric, ReasoningMetric,
    ReflectionMetric, RoutingMetric,
};

const EPS: f64 = 1e-9;

// All ports must produce these exact values
pub const PLANNING_3_OF_4: f64 = 0.75;
pub const REFLECTION_GREW: f64 = 1.0;
pub const REFLECTION_SHRUNK: f64 = 0.5;
pub const REFLECTION_SAME: f64 = 0.0;
pub const ROUTING_0_9_300: f64 = 0.8;
pub const ROUTING_NO_COST: f64 = 0.925;
pub const COORDINATION_PERFECT: f64 = 1.0;
pub const GUARDRAIL_HALF: f64 = 0.5;
pub const REASONING_4_OF_5: f64 = 0.8;
pub const MEMORY_9_OF_10: f64 = 0.9;

#[test]
fn ts_dotnet_java_planning_canonical() {
    let got = PlanningMetric::score(
        &["a".to_string(), "b".into(), "c".into(), "d".into()],
        &["a".to_string(), "b".into(), "c".into()],
    );
    assert!((got - PLANNING_3_OF_4).abs() < EPS, "Rust: {got}, Expected: {PLANNING_3_OF_4}");
}

#[test]
fn ts_dotnet_java_reflection_canonical() {
    assert!((ReflectionMetric::score("short", "longer answer") - REFLECTION_GREW).abs() < EPS);
    assert!((ReflectionMetric::score("longer text here", "short") - REFLECTION_SHRUNK).abs() < EPS);
    assert!((ReflectionMetric::score("x", "x") - REFLECTION_SAME).abs() < EPS);
}

#[test]
fn ts_dotnet_java_routing_canonical() {
    assert!((RoutingMetric::score(0.9, 300, 1000) - ROUTING_0_9_300).abs() < EPS);
    assert!((RoutingMetric::score(0.85, 0, 1000) - ROUTING_NO_COST).abs() < EPS);
}

#[test]
fn ts_dotnet_java_coordination_canonical() {
    assert!((CoordinationMetric::score(3, 3) - COORDINATION_PERFECT).abs() < EPS);
}

#[test]
fn ts_dotnet_java_guardrail_canonical() {
    assert!((GuardrailMetric::score(1, 2) - GUARDRAIL_HALF).abs() < EPS);
}

#[test]
fn ts_dotnet_java_reasoning_canonical() {
    assert!((ReasoningMetric::score(4, 5) - REASONING_4_OF_5).abs() < EPS);
}

#[test]
fn ts_dotnet_java_memory_canonical() {
    assert!((MemoryMetric::score(9, 10) - MEMORY_9_OF_10).abs() < EPS);
}
