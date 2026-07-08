use crate::indicator::{Indicator, IndicatorKind, ThreatLevel};
use crate::score::{ThreatScore, ThreatScorer};

#[test]
fn score_critical_high_raw() {
    let i = Indicator::new(
        "i1",
        "t1",
        IndicatorKind::IpAddress,
        "x",
        ThreatLevel::Critical,
        "f",
        0,
    );
    let score = ThreatScorer::score(&i, 0, 1000);
    assert!(score.raw_score > 70.0);
}

#[test]
fn score_low_threat_low_raw() {
    let i = Indicator::new(
        "i1",
        "t1",
        IndicatorKind::Domain,
        "x",
        ThreatLevel::Informational,
        "f",
        0,
    );
    let score = ThreatScorer::score(&i, 0, 1000);
    assert!(score.raw_score < 10.0);
}

#[test]
fn score_actionable() {
    let s = ThreatScore::new("i1", 75.0, 0.8);
    assert!(s.is_actionable());
}

#[test]
fn score_not_actionable_low_confidence() {
    let s = ThreatScore::new("i1", 80.0, 0.3);
    assert!(!s.is_actionable());
}

#[test]
fn score_with_tags_higher_confidence() {
    let i = Indicator::new(
        "i1",
        "t1",
        IndicatorKind::IpAddress,
        "x",
        ThreatLevel::High,
        "f",
        0,
    )
    .with_tag("apt");
    let score = ThreatScorer::score(&i, 0, 1000);
    assert_eq!(score.confidence, 0.8);
}

#[test]
fn score_recency_decay() {
    let i = Indicator::new(
        "i1",
        "t1",
        IndicatorKind::IpAddress,
        "x",
        ThreatLevel::High,
        "f",
        0,
    );
    let fresh = ThreatScorer::score(&i, 0, 1000);
    let stale = ThreatScorer::score(&i, 1000, 1000);
    assert!(fresh.raw_score > stale.raw_score);
}
