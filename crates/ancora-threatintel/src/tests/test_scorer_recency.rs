use crate::indicator::{Indicator, IndicatorKind, ThreatLevel};
use crate::score::ThreatScorer;

#[test]
fn recency_zero_max_gives_full_weight() {
    let i = Indicator::new(
        "i1",
        "t1",
        IndicatorKind::Domain,
        "x",
        ThreatLevel::High,
        "f",
        0,
    );
    let score = ThreatScorer::score(&i, 0, 0);
    assert!(score.raw_score > 0.0);
}

#[test]
fn fresh_indicator_scores_higher() {
    let i = Indicator::new(
        "i1",
        "t1",
        IndicatorKind::IpAddress,
        "x",
        ThreatLevel::High,
        "f",
        0,
    );
    let fresh = ThreatScorer::score(&i, 0, 100);
    let stale = ThreatScorer::score(&i, 100, 100);
    assert!(fresh.raw_score >= stale.raw_score);
}
