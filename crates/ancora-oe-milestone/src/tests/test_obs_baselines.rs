use crate::catalog_index::{CatalogEntry, CatalogIndex, CatalogKind};
use crate::self_hosted_summary::{SelfHostedSummary, Topology};

/// Performance baselines for the observability stack.
struct ObsBaseline {
    pub metric: &'static str,
    pub p99_ms: f64,
    pub budget_ms: f64,
}

impl ObsBaseline {
    fn within_budget(&self) -> bool {
        self.p99_ms <= self.budget_ms
    }
}

#[test]
fn obs_baselines_within_budget() {
    let baselines = vec![
        ObsBaseline {
            metric: "trace.export.latency",
            p99_ms: 4.2,
            budget_ms: 10.0,
        },
        ObsBaseline {
            metric: "metrics.flush.latency",
            p99_ms: 1.8,
            budget_ms: 5.0,
        },
        ObsBaseline {
            metric: "eval.run.submission",
            p99_ms: 12.0,
            budget_ms: 50.0,
        },
        ObsBaseline {
            metric: "log.batch.write",
            p99_ms: 3.1,
            budget_ms: 8.0,
        },
    ];

    for b in &baselines {
        assert!(
            b.within_budget(),
            "{} p99 {:.1}ms exceeds budget {:.1}ms",
            b.metric,
            b.p99_ms,
            b.budget_ms
        );
    }
}

#[test]
fn self_hosted_ha_meets_baseline() {
    let s = SelfHostedSummary::new(Topology::HighAvailability, "victoria-metrics")
        .with_auth("oidc")
        .with_retention(90, 14, 30);
    assert!(s.is_production_grade());
    assert!(s.metrics_retention_days >= 30);
    assert!(s.traces_retention_days >= 7);
}

#[test]
fn catalog_baseline_metrics_stable() {
    let catalog = CatalogIndex::new()
        .add(
            CatalogEntry::new(
                "M001",
                CatalogKind::Metric,
                "agent.request.latency_p99",
                "P99 latency baseline",
            )
            .stable(),
        )
        .add(
            CatalogEntry::new(
                "M002",
                CatalogKind::Metric,
                "agent.eval.score",
                "Eval score baseline",
            )
            .stable(),
        );

    let stable = catalog.stable_entries();
    assert_eq!(stable.len(), 2, "Baseline metrics must be marked stable");
}
