/// Tests that exporters respect data residency rules.
/// Traces tagged with a region are only sent to exporters for that region.
use crate::trace_e2e::{build_run_trace, MockCollector, TraceExporter};

/// A regioned exporter that only accepts traces for its configured region.
struct RegionedCollector {
    pub region: String,
    pub inner: MockCollector,
}

impl RegionedCollector {
    fn new(region: impl Into<String>) -> Self {
        Self {
            region: region.into(),
            inner: MockCollector::new(),
        }
    }

    fn export_if_allowed(
        &mut self,
        trace: &crate::trace_e2e::Trace,
        trace_region: &str,
    ) -> Result<(), String> {
        if trace_region != self.region {
            return Err(format!(
                "Residency violation: trace region '{}' not allowed in exporter region '{}'",
                trace_region, self.region
            ));
        }
        self.inner.export(trace)
    }
}

#[test]
fn residency_respected_same_region() {
    let trace = build_run_trace("res-001");
    let mut collector = RegionedCollector::new("eu-west-1");

    let result = collector.export_if_allowed(&trace, "eu-west-1");
    assert!(result.is_ok(), "same-region export must succeed");
    assert_eq!(collector.inner.count(), 1);
}

#[test]
fn residency_respected_blocks_cross_region() {
    let trace = build_run_trace("res-002");
    let mut collector = RegionedCollector::new("eu-west-1");

    let result = collector.export_if_allowed(&trace, "us-east-1");
    assert!(result.is_err(), "cross-region export must be blocked");
    assert_eq!(collector.inner.count(), 0, "no traces must be stored");
}

#[test]
fn multiple_regions_each_get_their_own_traces() {
    let trace_eu = build_run_trace("res-eu-001");
    let trace_us = build_run_trace("res-us-001");

    let mut eu_collector = RegionedCollector::new("eu-west-1");
    let mut us_collector = RegionedCollector::new("us-east-1");

    eu_collector
        .export_if_allowed(&trace_eu, "eu-west-1")
        .unwrap();
    us_collector
        .export_if_allowed(&trace_us, "us-east-1")
        .unwrap();

    // Cross-region blocked.
    assert!(eu_collector
        .export_if_allowed(&trace_us, "us-east-1")
        .is_err());
    assert!(us_collector
        .export_if_allowed(&trace_eu, "eu-west-1")
        .is_err());

    assert_eq!(eu_collector.inner.count(), 1);
    assert_eq!(us_collector.inner.count(), 1);
}
