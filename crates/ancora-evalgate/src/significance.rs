/// Statistical significance check.
///
/// Uses a simple z-test approximation to decide whether the observed delta
/// is large enough to be treated as real rather than noise.  No external
/// crates are used; the normal CDF is approximated with a rational fit.

/// Parameters that describe a set of repeated eval samples.
#[derive(Debug, Clone)]
pub struct SampleStats {
    /// Number of independent observations.
    pub n: usize,
    /// Sample mean.
    pub mean: f64,
    /// Sample standard deviation (unbiased, Bessel-corrected).
    pub std_dev: f64,
}

impl SampleStats {
    /// Construct stats, returning `None` when `n < 2` or `std_dev < 0`.
    pub fn new(n: usize, mean: f64, std_dev: f64) -> Option<Self> {
        if n < 2 || std_dev < 0.0 {
            return None;
        }
        Some(Self { n, mean, std_dev })
    }

    /// Standard error of the mean.
    pub fn sem(&self) -> f64 {
        self.std_dev / (self.n as f64).sqrt()
    }
}

/// Rational approximation of the standard normal CDF (Abramowitz & Stegun 26.2.17).
/// Accurate to about 7.5e-8 for all z.
fn normal_cdf(z: f64) -> f64 {
    if z < 0.0 {
        return 1.0 - normal_cdf(-z);
    }
    let t = 1.0 / (1.0 + 0.2316419 * z);
    let poly = t
        * (0.319381530
            + t * (-0.356563782 + t * (1.781477937 + t * (-1.821255978 + t * 1.330274429))));
    let pdf = (-0.5 * z * z).exp() / (2.0 * std::f64::consts::PI).sqrt();
    1.0 - pdf * poly
}

/// Return the two-tailed p-value for a one-sample z-test.
///
/// `observed_delta` - difference between candidate and baseline means.
/// `baseline`       - statistics of the baseline sample.
/// `candidate`      - statistics of the candidate sample (may equal baseline for 1-sample).
pub fn p_value(baseline: &SampleStats, candidate: &SampleStats) -> f64 {
    // Welch's z (approximation valid for large n).
    let se = (baseline.sem().powi(2) + candidate.sem().powi(2)).sqrt();
    if se == 0.0 {
        // No variance at all - treat as not significant if delta == 0, else significant.
        return if (baseline.mean - candidate.mean).abs() < f64::EPSILON {
            1.0
        } else {
            0.0
        };
    }
    let z = (candidate.mean - baseline.mean).abs() / se;
    2.0 * (1.0 - normal_cdf(z))
}

/// Returns `true` when the result is statistically significant at `alpha`.
///
/// A common choice for CI gates is `alpha = 0.05`.
pub fn is_significant(baseline: &SampleStats, candidate: &SampleStats, alpha: f64) -> bool {
    p_value(baseline, candidate) < alpha
}
