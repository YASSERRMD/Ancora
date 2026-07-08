//! Integration tests: verify no false alarms on stable data.

use crate::alerting::{Alert, AlertAggregator, Severity};
use crate::cost_drift::CostDriftDetector;
use crate::input_drift::InputDriftDetector;
use crate::output_drift::OutputDriftDetector;
use crate::reference::{ReferenceBuilder, Stats};
use crate::tool_drift::ToolDriftDetector;
use std::collections::HashMap;

/// Build a reference where inputs, outputs, costs, and tools are stable.
fn stable_reference() -> crate::reference::ReferenceDistribution {
    let mut b = ReferenceBuilder::new();
    let tools = vec!["search".to_string()];
    for _ in 0..200 {
        b.add("hello world", "short answer", 100, 50, &tools, "openai");
    }
    b.build().unwrap()
}

#[test]
fn no_false_alarm_when_all_metrics_stable() {
    let reference = stable_reference();

    // Current stats mirror the reference distribution exactly.
    let input_current = Stats::from_slice(&[11.0]).unwrap();
    let output_current = Stats::from_slice(&[12.0]).unwrap();
    let cost_current = Stats::from_slice(&[100.0]).unwrap();

    let mut tool_counts = HashMap::new();
    tool_counts.insert("search".to_string(), 50);

    let input_result = InputDriftDetector::new(3.0).check(&reference, &input_current);
    let output_result = OutputDriftDetector::new(3.0).check(&reference, &output_current);
    let cost_result = CostDriftDetector::new(3.0).check(&reference, &cost_current);
    let tool_result = ToolDriftDetector::new(0.15).check(&reference, &tool_counts);

    let mut agg = AlertAggregator::new();

    if let Ok(r) = input_result {
        agg.push_if(
            r.drifted,
            Alert::new(Severity::Warning, "input_drift", "input drifted"),
        );
    }
    if let Ok(r) = output_result {
        agg.push_if(
            r.drifted,
            Alert::new(Severity::Warning, "output_drift", "output drifted"),
        );
    }
    if let Ok(r) = cost_result {
        agg.push_if(
            r.drifted,
            Alert::new(Severity::Warning, "cost_drift", "cost drifted"),
        );
    }
    agg.push_if(
        tool_result.any_drifted,
        Alert::new(Severity::Warning, "tool_drift", "tools drifted"),
    );

    let alerts = agg.flush();
    assert!(
        alerts.is_empty(),
        "no alerts expected on stable data, got: {:?}",
        alerts.iter().map(|a| &a.kind).collect::<Vec<_>>()
    );
}
