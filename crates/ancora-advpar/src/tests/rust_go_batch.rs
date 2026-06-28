// Rust+Go batch parity: confirms that canonical values Rust produces match
// what the Go advanced-parity example prints.
use ancora_ageval::{
    CoordinationMetric, GuardrailMetric, MemoryMetric, PlanningMetric, ReasoningMetric,
    ReflectionMetric, RoutingMetric,
};

const EPS: f64 = 1e-9;

struct Canonical {
    planning_3of4: f64,
    reflection_grew: f64,
    reflection_shrunk: f64,
    reflection_same: f64,
    routing_0_9_300: f64,
    routing_no_cost: f64,
    coordination_3of3: f64,
    guardrail_1of2: f64,
    reasoning_4of5: f64,
    memory_9of10: f64,
}

fn rust_canonical() -> Canonical {
    Canonical {
        planning_3of4: PlanningMetric::score(
            &["a".to_string(), "b".into(), "c".into(), "d".into()],
            &["a".to_string(), "b".into(), "c".into()],
        ),
        reflection_grew: ReflectionMetric::score("short", "longer answer"),
        reflection_shrunk: ReflectionMetric::score("longer text here", "short"),
        reflection_same: ReflectionMetric::score("x", "x"),
        routing_0_9_300: RoutingMetric::score(0.9, 300, 1000),
        routing_no_cost: RoutingMetric::score(0.85, 0, 1000),
        coordination_3of3: CoordinationMetric::score(3, 3),
        guardrail_1of2: GuardrailMetric::score(1, 2),
        reasoning_4of5: ReasoningMetric::score(4, 5),
        memory_9of10: MemoryMetric::score(9, 10),
    }
}

// Go canonical values (from sdk/go/examples/advanced-parity/main.go)
fn go_canonical() -> Canonical {
    Canonical {
        planning_3of4: 0.75,
        reflection_grew: 1.0,
        reflection_shrunk: 0.5,
        reflection_same: 0.0,
        routing_0_9_300: 0.8,
        routing_no_cost: 0.925,
        coordination_3of3: 1.0,
        guardrail_1of2: 0.5,
        reasoning_4of5: 0.8,
        memory_9of10: 0.9,
    }
}

#[test]
fn rust_go_planning_parity() {
    let r = rust_canonical();
    let g = go_canonical();
    assert!((r.planning_3of4 - g.planning_3of4).abs() < EPS);
}

#[test]
fn rust_go_reflection_parity() {
    let r = rust_canonical();
    let g = go_canonical();
    assert!((r.reflection_grew - g.reflection_grew).abs() < EPS);
    assert!((r.reflection_shrunk - g.reflection_shrunk).abs() < EPS);
    assert!((r.reflection_same - g.reflection_same).abs() < EPS);
}

#[test]
fn rust_go_routing_parity() {
    let r = rust_canonical();
    let g = go_canonical();
    assert!((r.routing_0_9_300 - g.routing_0_9_300).abs() < EPS);
    assert!((r.routing_no_cost - g.routing_no_cost).abs() < EPS);
}

#[test]
fn rust_go_coordination_parity() {
    let r = rust_canonical();
    let g = go_canonical();
    assert!((r.coordination_3of3 - g.coordination_3of3).abs() < EPS);
}

#[test]
fn rust_go_guardrail_parity() {
    let r = rust_canonical();
    let g = go_canonical();
    assert!((r.guardrail_1of2 - g.guardrail_1of2).abs() < EPS);
}

#[test]
fn rust_go_reasoning_parity() {
    let r = rust_canonical();
    let g = go_canonical();
    assert!((r.reasoning_4of5 - g.reasoning_4of5).abs() < EPS);
}

#[test]
fn rust_go_memory_parity() {
    let r = rust_canonical();
    let g = go_canonical();
    assert!((r.memory_9of10 - g.memory_9of10).abs() < EPS);
}
