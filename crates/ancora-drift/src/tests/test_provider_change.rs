//! Integration tests for provider change detection.

use crate::provider_change::{ProviderChangeDetector, ProviderSnapshot};
use crate::reference::ReferenceBuilder;

fn baseline() -> crate::reference::ReferenceDistribution {
    let mut b = ReferenceBuilder::new();
    for _ in 0..100 {
        b.add("q", "a", 100, 50, &[], "openai");
    }
    b.build().unwrap()
}

#[test]
fn provider_change_detected_when_provider_switches() {
    let reference = baseline();
    let snapshots = vec![ProviderSnapshot {
        name: "anthropic".into(),
        frequency: 1.0,
        mean_latency_ms: 55.0,
        error_rate: 0.0,
    }];
    let detector = ProviderChangeDetector::default();
    let result = detector.check(&reference, &snapshots);
    assert!(result.any_changed, "expected change when provider switches");
}

#[test]
fn provider_stable_no_change() {
    let reference = baseline();
    let snapshots = vec![ProviderSnapshot {
        name: "openai".into(),
        frequency: 1.0,
        mean_latency_ms: 51.0, // within 3 std-devs of ref mean=50, std=0
        error_rate: 0.0,
    }];
    let detector = ProviderChangeDetector::default();
    let result = detector.check(&reference, &snapshots);
    // frequency unchanged, latency std=0 so latency_changed=false
    assert!(!result.any_changed);
}
