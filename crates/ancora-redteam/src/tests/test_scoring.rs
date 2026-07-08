use crate::{AdversarialScenario, AttackCategory, EffectivenessReport, GuardrailScorer};

fn perfect_checker(scenarios: &[AdversarialScenario]) -> EffectivenessReport {
    GuardrailScorer::score(scenarios, |_| true) // blocks everything
}

fn blind_checker(scenarios: &[AdversarialScenario]) -> EffectivenessReport {
    GuardrailScorer::score(scenarios, |_| false) // blocks nothing
}

fn make_mixed() -> Vec<AdversarialScenario> {
    vec![
        AdversarialScenario::new("s1", AttackCategory::Injection, "attack-A", true),
        AdversarialScenario::new("s2", AttackCategory::Injection, "safe", false),
        AdversarialScenario::new("s3", AttackCategory::Jailbreak, "attack-B", true),
    ]
}

#[test]
fn scoring_perfect_blocker() {
    let scenarios = make_mixed();
    let report = perfect_checker(&scenarios);
    // blocks everything: attacks correctly blocked, safe incorrectly blocked -> fp=1
    assert_eq!(report.false_negatives(), 0);
    assert_eq!(report.false_positives(), 1);
}

#[test]
fn scoring_blind_checker() {
    let scenarios = make_mixed();
    let report = blind_checker(&scenarios);
    // blocks nothing: attacks not caught -> fn=2, safe allowed correctly
    assert_eq!(report.false_negatives(), 2);
    assert_eq!(report.false_positives(), 0);
}

#[test]
fn scoring_effectiveness_range() {
    let scenarios = make_mixed();
    let report = perfect_checker(&scenarios);
    assert!(report.effectiveness() >= 0.0);
    assert!(report.effectiveness() <= 1.0);
}

#[test]
fn scoring_summary_format() {
    let scenarios = make_mixed();
    let report = perfect_checker(&scenarios);
    let summary = report.summary();
    assert!(summary.contains("RedTeam:"));
    assert!(summary.contains("effectiveness="));
}

#[test]
fn scoring_empty_report() {
    let report = EffectivenessReport::new(vec![]);
    assert!((report.effectiveness() - 1.0).abs() < f64::EPSILON);
    assert_eq!(report.false_negatives(), 0);
}
