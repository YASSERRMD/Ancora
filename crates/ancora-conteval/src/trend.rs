/// Quality trend detection.
///
/// Analyses a sequence of rolling mean values to detect significant
/// upward or downward quality trends using a simple linear regression
/// over the most recent N data points.

/// Direction of a detected trend.
#[derive(Debug, Clone, PartialEq)]
pub enum TrendDirection {
    Improving,
    Degrading,
    Stable,
}

/// Result of a trend analysis.
#[derive(Debug, Clone)]
pub struct TrendResult {
    pub direction: TrendDirection,
    /// Slope of the fitted line (score change per step).
    pub slope: f64,
    /// Number of data points used.
    pub window: usize,
}

/// Analyse the trend of a score series.
///
/// Returns `None` if there are fewer than 2 data points.
/// `degradation_threshold` and `improvement_threshold` are the slope
/// magnitudes at which trend is considered non-stable.
pub fn analyse_trend(
    scores: &[f64],
    degradation_threshold: f64,
    improvement_threshold: f64,
) -> Option<TrendResult> {
    let n = scores.len();
    if n < 2 {
        return None;
    }

    // Simple linear regression: y = a + b*x, compute slope b.
    let n_f = n as f64;
    let x_mean = (n_f - 1.0) / 2.0;
    let y_mean: f64 = scores.iter().sum::<f64>() / n_f;

    let mut num = 0.0f64;
    let mut den = 0.0f64;
    for (i, &y) in scores.iter().enumerate() {
        let x = i as f64 - x_mean;
        num += x * (y - y_mean);
        den += x * x;
    }

    let slope = if den.abs() < f64::EPSILON { 0.0 } else { num / den };

    let direction = if slope <= -degradation_threshold.abs() {
        TrendDirection::Degrading
    } else if slope >= improvement_threshold.abs() {
        TrendDirection::Improving
    } else {
        TrendDirection::Stable
    };

    Some(TrendResult {
        direction,
        slope,
        window: n,
    })
}

/// Detector that watches a named series and fires when a trend is detected.
#[derive(Debug)]
pub struct TrendDetector {
    pub name: String,
    degradation_threshold: f64,
    improvement_threshold: f64,
    history: Vec<f64>,
    window: usize,
}

impl TrendDetector {
    /// Create a new trend detector.
    ///
    /// * `name` - label for the series (e.g. model or provider name)
    /// * `window` - number of most-recent observations to regress over
    /// * `degradation_threshold` - negative slope magnitude to flag degradation
    /// * `improvement_threshold` - positive slope magnitude to flag improvement
    pub fn new(
        name: impl Into<String>,
        window: usize,
        degradation_threshold: f64,
        improvement_threshold: f64,
    ) -> Self {
        TrendDetector {
            name: name.into(),
            degradation_threshold,
            improvement_threshold,
            history: Vec::new(),
            window,
        }
    }

    /// Push a new mean score and analyse the trend.
    ///
    /// Returns a `TrendResult` when enough data is available.
    pub fn push(&mut self, score: f64) -> Option<TrendResult> {
        self.history.push(score);
        let start = self.history.len().saturating_sub(self.window);
        let slice = &self.history[start..];
        analyse_trend(slice, self.degradation_threshold, self.improvement_threshold)
    }

    /// Number of observations recorded.
    pub fn history_len(&self) -> usize {
        self.history.len()
    }
}
