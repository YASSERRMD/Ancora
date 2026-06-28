use ancora_guard::{GuardrailJournal, GuardrailOutcome, GuardrailPolicy, InjectionInputGuardrail};
use crate::{known_attack_regression_set, GuardrailScorer};

fn injection_check(payload: &str) -> bool {
    let mut p = GuardrailPolicy::new();
    p.add_input(InjectionInputGuardrail);
    let mut j = GuardrailJournal::default();
    !matches!(p.check_input(payload, &mut j, 1), GuardrailOutcome::Pass)
}

#[test]
fn regression_set_not_empty() {
    let set = known_attack_regression_set();
    assert!(!set.is_empty());
}

#[test]
fn regression_all_expected_blocked() {
    let set = known_attack_regression_set();
    assert!(set.iter().all(|s| s.expected_blocked));
}

#[test]
fn regression_injection_attacks_blocked() {
    let set = known_attack_regression_set();
    let injection: Vec<_> = set.iter()
        .filter(|s| s.category == crate::AttackCategory::Injection)
        .collect();
    assert!(!injection.is_empty());
    for scenario in &injection {
        assert!(
            injection_check(&scenario.payload),
            "regression scenario {} not blocked: '{}'",
            scenario.id, scenario.payload
        );
    }
}

#[test]
fn regression_known_attacks_pass_scorer() {
    let set = known_attack_regression_set();
    let injectable: Vec<_> = set.iter()
        .filter(|s| s.category == crate::AttackCategory::Injection)
        .cloned()
        .collect();
    let report = GuardrailScorer::score(&injectable, |p| injection_check(p));
    assert_eq!(report.false_negatives(), 0, "regression: injection not all caught");
}

#[test]
fn regression_ids_are_unique() {
    let set = known_attack_regression_set();
    let mut ids: Vec<&str> = set.iter().map(|s| s.id.as_str()).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), set.len());
}
