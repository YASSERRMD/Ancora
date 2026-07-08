/// Drift and quality view - tracks prompt/output drift across run history.

#[derive(Debug, Clone)]
pub struct DriftPoint {
    pub run_id: String,
    pub timestamp: u64,
    pub metric: String,
    pub value: f64,
    pub baseline: f64,
}

impl DriftPoint {
    pub fn delta(&self) -> f64 {
        self.value - self.baseline
    }

    pub fn relative_drift_pct(&self) -> f64 {
        if self.baseline == 0.0 {
            return 0.0;
        }
        (self.delta() / self.baseline.abs()) * 100.0
    }

    pub fn is_regressing(&self, threshold_pct: f64) -> bool {
        self.relative_drift_pct().abs() > threshold_pct
    }
}

#[derive(Debug, Clone)]
pub struct QualityMetric {
    pub name: String,
    pub description: String,
    pub current_value: f64,
    pub baseline_value: f64,
    pub unit: String,
}

pub struct DriftView {
    pub points: Vec<DriftPoint>,
    pub quality_metrics: Vec<QualityMetric>,
}

impl DriftView {
    pub fn new(points: Vec<DriftPoint>, quality_metrics: Vec<QualityMetric>) -> Self {
        Self {
            points,
            quality_metrics,
        }
    }

    pub fn points_for_metric(&self, metric: &str) -> Vec<&DriftPoint> {
        self.points.iter().filter(|p| p.metric == metric).collect()
    }

    pub fn regressing_metrics(&self, threshold_pct: f64) -> Vec<&DriftPoint> {
        self.points
            .iter()
            .filter(|p| p.is_regressing(threshold_pct))
            .collect()
    }

    pub fn metric_names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.points.iter().map(|p| p.metric.as_str()).collect();
        names.sort_unstable();
        names.dedup();
        names
    }

    pub fn latest_point_for_metric(&self, metric: &str) -> Option<&DriftPoint> {
        self.points
            .iter()
            .filter(|p| p.metric == metric)
            .max_by_key(|p| p.timestamp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_view() -> DriftView {
        DriftView::new(
            vec![
                DriftPoint {
                    run_id: "r1".into(),
                    timestamp: 100,
                    metric: "latency_ms".into(),
                    value: 120.0,
                    baseline: 100.0,
                },
                DriftPoint {
                    run_id: "r2".into(),
                    timestamp: 200,
                    metric: "latency_ms".into(),
                    value: 95.0,
                    baseline: 100.0,
                },
            ],
            vec![],
        )
    }

    #[test]
    fn test_regressing() {
        let view = sample_view();
        let regressing = view.regressing_metrics(15.0);
        assert_eq!(regressing.len(), 1);
        assert_eq!(regressing[0].run_id, "r1");
    }

    #[test]
    fn test_latest_point() {
        let view = sample_view();
        let latest = view.latest_point_for_metric("latency_ms").unwrap();
        assert_eq!(latest.run_id, "r2");
    }
}
