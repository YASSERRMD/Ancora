/// Statistical analysis of experiment results using Welch's t-test.
use crate::outcome::VariantStats;

/// Result of a two-sample significance test.
#[derive(Debug, Clone)]
pub struct SignificanceResult {
    pub control_variant: String,
    pub treatment_variant: String,
    /// Observed difference in means: treatment - control.
    pub mean_difference: f64,
    /// Approximate two-tailed p-value from Welch's t-test.
    pub p_value: f64,
    /// Whether the result is significant at the given threshold.
    pub is_significant: bool,
    /// Significance threshold used.
    pub alpha: f64,
}

/// Error types for analysis.
#[derive(Debug, PartialEq)]
pub enum AnalysisError {
    InsufficientData { variant: String, n: usize },
    ZeroVariance { variant: String },
}

impl std::fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalysisError::InsufficientData { variant, n } => {
                write!(
                    f,
                    "variant '{variant}' has only {n} observations (need >= 2)"
                )
            }
            AnalysisError::ZeroVariance { variant } => {
                write!(
                    f,
                    "variant '{variant}' has zero variance; cannot run t-test"
                )
            }
        }
    }
}

/// Perform Welch's two-sample t-test between control and treatment stats.
///
/// Returns the approximate two-tailed p-value. Uses a simple approximation
/// of the t-distribution CDF based on the error function for portability
/// (no external crates needed).
pub fn welch_t_test(
    control: &VariantStats,
    treatment: &VariantStats,
    alpha: f64,
) -> Result<SignificanceResult, AnalysisError> {
    if control.n < 2 {
        return Err(AnalysisError::InsufficientData {
            variant: control.variant_name.clone(),
            n: control.n,
        });
    }
    if treatment.n < 2 {
        return Err(AnalysisError::InsufficientData {
            variant: treatment.variant_name.clone(),
            n: treatment.n,
        });
    }

    let se_c = control.variance / control.n as f64;
    let se_t = treatment.variance / treatment.n as f64;
    let se_total = se_c + se_t;

    if se_total == 0.0 {
        // Both variances are zero - check if means differ.
        let p_value = if (control.mean - treatment.mean).abs() < 1e-12 {
            1.0
        } else {
            0.0
        };
        return Ok(SignificanceResult {
            control_variant: control.variant_name.clone(),
            treatment_variant: treatment.variant_name.clone(),
            mean_difference: treatment.mean - control.mean,
            p_value,
            is_significant: p_value < alpha,
            alpha,
        });
    }

    let t_stat = (treatment.mean - control.mean) / se_total.sqrt();

    // Welch-Satterthwaite degrees of freedom.
    let df = se_total.powi(2)
        / (se_c.powi(2) / (control.n as f64 - 1.0) + se_t.powi(2) / (treatment.n as f64 - 1.0));

    // Approximate two-tailed p-value using the regularized incomplete beta function.
    let p_value = two_tailed_p(t_stat.abs(), df);

    Ok(SignificanceResult {
        control_variant: control.variant_name.clone(),
        treatment_variant: treatment.variant_name.clone(),
        mean_difference: treatment.mean - control.mean,
        p_value,
        is_significant: p_value < alpha,
        alpha,
    })
}

/// Approximate two-tailed p-value for a t-distribution using a rational approximation.
///
/// This is accurate enough for experiment significance decisions; for rigorous
/// work use a proper statistics library.
fn two_tailed_p(t_abs: f64, df: f64) -> f64 {
    // Use normal approximation when df is large.
    if df > 30.0 {
        let z = t_abs;
        2.0 * (1.0 - normal_cdf(z))
    } else {
        // Use regularized incomplete beta: p = I(df/(df+t^2), df/2, 0.5)
        let x = df / (df + t_abs * t_abs);
        regularized_incomplete_beta(x, df / 2.0, 0.5)
    }
}

/// Standard normal CDF approximation (Abramowitz & Stegun 26.2.17).
fn normal_cdf(z: f64) -> f64 {
    if z < 0.0 {
        return 1.0 - normal_cdf(-z);
    }
    let t = 1.0 / (1.0 + 0.2316419 * z);
    let poly = t
        * (0.319381530
            + t * (-0.356563782 + t * (1.781477937 + t * (-1.821255978 + t * 1.330274429))));
    1.0 - pdf_standard_normal(z) * poly
}

fn pdf_standard_normal(z: f64) -> f64 {
    (-0.5 * z * z).exp() / (2.0 * std::f64::consts::PI).sqrt()
}

/// Regularized incomplete beta function I_x(a, b) via continued fraction.
/// Returns a value in [0, 1].
fn regularized_incomplete_beta(x: f64, a: f64, b: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    // Use symmetry relation when x > (a+1)/(a+b+2) for better convergence.
    if x > (a + 1.0) / (a + b + 2.0) {
        return 1.0 - regularized_incomplete_beta(1.0 - x, b, a);
    }
    let ln_beta = ln_gamma(a) + ln_gamma(b) - ln_gamma(a + b);
    let front = (x.ln() * a + (1.0 - x).ln() * b - ln_beta).exp() / a;
    front * beta_continued_fraction(x, a, b)
}

/// Continued fraction for incomplete beta (Lentz method, max 200 iterations).
fn beta_continued_fraction(x: f64, a: f64, b: f64) -> f64 {
    let max_iter = 200;
    let eps = 1e-12;
    let mut c = 1.0_f64;
    let mut d = 1.0 - (a + b) * x / (a + 1.0);
    if d.abs() < f64::MIN_POSITIVE {
        d = f64::MIN_POSITIVE;
    }
    d = 1.0 / d;
    let mut h = d;
    for m in 1..=max_iter {
        let m = m as f64;
        // Even step
        let numerator = m * (b - m) * x / ((a + 2.0 * m - 1.0) * (a + 2.0 * m));
        d = 1.0 + numerator * d;
        if d.abs() < f64::MIN_POSITIVE {
            d = f64::MIN_POSITIVE;
        }
        c = 1.0 + numerator / c;
        if c.abs() < f64::MIN_POSITIVE {
            c = f64::MIN_POSITIVE;
        }
        d = 1.0 / d;
        h *= d * c;
        // Odd step
        let numerator = -(a + m) * (a + b + m) * x / ((a + 2.0 * m) * (a + 2.0 * m + 1.0));
        d = 1.0 + numerator * d;
        if d.abs() < f64::MIN_POSITIVE {
            d = f64::MIN_POSITIVE;
        }
        c = 1.0 + numerator / c;
        if c.abs() < f64::MIN_POSITIVE {
            c = f64::MIN_POSITIVE;
        }
        d = 1.0 / d;
        let delta = d * c;
        h *= delta;
        if (delta - 1.0).abs() < eps {
            break;
        }
    }
    h
}

/// Natural log of the gamma function (Stirling approximation for x > 0).
fn ln_gamma(x: f64) -> f64 {
    // Lanczos approximation coefficients (g=7, n=9)
    const G: f64 = 7.0;
    const C: &[f64] = &[
        0.99999999999980993,
        676.5203681218851,
        -1259.1392167224028,
        771.32342877765313,
        -176.61502916214059,
        12.507343278686905,
        -0.13857109526572012,
        9.9843695780195716e-6,
        1.5056327351493116e-7,
    ];
    let mut x = x;
    if x < 0.5 {
        return std::f64::consts::PI.ln()
            - (std::f64::consts::PI * x).sin().ln()
            - ln_gamma(1.0 - x);
    }
    x -= 1.0;
    let mut sum = C[0];
    for (i, &c) in C[1..].iter().enumerate() {
        sum += c / (x + i as f64 + 1.0);
    }
    let t = x + G + 0.5;
    0.5 * (2.0 * std::f64::consts::PI).ln() + (x + 0.5) * t.ln() - t + sum.ln()
}
