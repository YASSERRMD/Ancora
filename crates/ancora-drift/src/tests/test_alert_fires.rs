//! Integration tests: verify that an alert fires when drift is detected.

use crate::alerting::{Alert, AlertAggregator, AlertPolicy, Severity};
use crate::cost_drift::CostDriftDetector;
use crate::reference::{ReferenceBuilder, Stats};

#[test]
fn alert_fires_on_cost_drift() {
    // Build reference with mean cost ~100 micros, std ~50
    let mut b = ReferenceBuilder::new();
    b.add("q", "a", 50, 50, &[], "openai");
    b.add("q", "a", 150, 50, &[], "openai");
    let reference = b.build().unwrap();

    // Current mean is enormously higher
    let current = Stats::from_slice(&[10_000.0]).unwrap();
    let detector = CostDriftDetector::new(3.0);
    let result = detector.check(&reference, &current).unwrap();

    let mut agg = AlertAggregator::new();
    agg.push_if(
        result.drifted,
        Alert::new(Severity::Critical, "cost_drift", "cost spiked")
            .with_metric(result.mean_z_score),
    );

    let alerts = agg.flush();
    assert_eq!(alerts.len(), 1);
    assert_eq!(alerts[0].severity, Severity::Critical);
    assert_eq!(alerts[0].kind, "cost_drift");
}

#[test]
fn alert_respects_policy_min_severity() {
    let mut agg = AlertAggregator::new();
    agg.push(Alert::new(Severity::Info, "noise", "ignore me"));
    agg.push(Alert::new(Severity::Warning, "tool_drift", "tool changed"));

    let alerts = agg.flush();
    let policy = AlertPolicy::new(Severity::Warning);
    let filtered = policy.filter(&alerts);
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].kind, "tool_drift");
}
