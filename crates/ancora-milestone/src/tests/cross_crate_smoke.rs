use ancora_ageval::PlanningMetric;
use ancora_guard::{GuardrailOutcome, InjectionInputGuardrail, InputGuardrail};
use ancora_preset::{assemble, coding_agent};
use ancora_reason::CitationStore;

#[test]
fn cross_crate_smoke_planning_to_preset() {
    let quality = PlanningMetric::score(
        &["step-1".into(), "step-2".into()],
        &["step-1".into(), "step-2".into()],
    );
    assert_eq!(quality, 1.0);

    let spec = assemble(&coding_agent()).expect("assemble");
    assert!(spec.tools.contains(&"planning".to_string()));
}

#[test]
fn cross_crate_smoke_guard_to_reason() {
    let guard = InjectionInputGuardrail;
    let safe = guard.check_input("explain rust lifetimes");
    assert!(matches!(safe, GuardrailOutcome::Pass));

    let mut cs = CitationStore::new();
    cs.add("rust lifetimes", "rust reference".to_string());
    assert!(cs.has_citations("rust lifetimes"));
}
