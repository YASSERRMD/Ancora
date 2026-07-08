use crate::routing::{RoutingOutcome, RoutingSuite};

#[test]
fn routing_default_catalog_all_pass() {
    let suite = RoutingSuite::default_catalog();
    let (passed, total) = suite.run_all();
    assert_eq!(total, 3, "expected 3 routing cases");
    assert_eq!(passed, total, "all routing cases should pass");
}

#[test]
fn simple_query_routes_to_light_model() {
    let suite = RoutingSuite::default_catalog();
    let case = &suite.cases[0]; // ro-001: simple arithmetic
    assert_eq!(suite.evaluate(case), RoutingOutcome::Correct);
}

#[test]
fn explain_query_routes_to_standard_model() {
    let suite = RoutingSuite::default_catalog();
    let case = &suite.cases[1]; // ro-002: explain TCP vs UDP
    assert_eq!(suite.evaluate(case), RoutingOutcome::Correct);
}

#[test]
fn complex_query_routes_to_advanced_model() {
    let suite = RoutingSuite::default_catalog();
    let case = &suite.cases[2]; // ro-003: multi-step proof
    assert_eq!(suite.evaluate(case), RoutingOutcome::Correct);
}
