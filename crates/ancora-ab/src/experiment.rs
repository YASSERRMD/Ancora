/// Defines an A/B experiment: its variants and the primary metric to optimize.

/// A single variant in an experiment.
#[derive(Debug, Clone, PartialEq)]
pub struct Variant {
    /// Unique name for this variant (e.g. "control", "treatment_a").
    pub name: String,
    /// Fraction of traffic allocated to this variant (0.0..=1.0).
    pub traffic_weight: f64,
}

impl Variant {
    pub fn new(name: impl Into<String>, traffic_weight: f64) -> Self {
        Variant {
            name: name.into(),
            traffic_weight,
        }
    }
}

/// The primary metric type an experiment optimizes for.
#[derive(Debug, Clone, PartialEq)]
pub enum MetricKind {
    /// Higher is better (e.g. success rate, satisfaction score).
    Maximize,
    /// Lower is better (e.g. latency, error rate).
    Minimize,
}

/// Definition of the metric tracked by the experiment.
#[derive(Debug, Clone)]
pub struct Metric {
    pub name: String,
    pub kind: MetricKind,
}

impl Metric {
    pub fn new(name: impl Into<String>, kind: MetricKind) -> Self {
        Metric {
            name: name.into(),
            kind,
        }
    }
}

/// An experiment definition: name, variants, and primary metric.
#[derive(Debug, Clone)]
pub struct Experiment {
    pub id: String,
    pub description: String,
    pub variants: Vec<Variant>,
    pub metric: Metric,
}

/// Errors that can arise when constructing an experiment.
#[derive(Debug, PartialEq)]
pub enum ExperimentError {
    NoVariants,
    WeightsMustSumToOne { actual: f64 },
    DuplicateVariantName(String),
    InvalidWeight { name: String, weight: f64 },
}

impl std::fmt::Display for ExperimentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExperimentError::NoVariants => write!(f, "experiment must have at least one variant"),
            ExperimentError::WeightsMustSumToOne { actual } => {
                write!(f, "variant weights must sum to 1.0, got {actual:.4}")
            }
            ExperimentError::DuplicateVariantName(n) => {
                write!(f, "duplicate variant name: {n}")
            }
            ExperimentError::InvalidWeight { name, weight } => {
                write!(f, "variant '{name}' has invalid weight {weight}")
            }
        }
    }
}

impl Experiment {
    /// Create and validate an experiment definition.
    pub fn new(
        id: impl Into<String>,
        description: impl Into<String>,
        variants: Vec<Variant>,
        metric: Metric,
    ) -> Result<Self, ExperimentError> {
        if variants.is_empty() {
            return Err(ExperimentError::NoVariants);
        }
        let mut seen = std::collections::HashSet::new();
        for v in &variants {
            if v.traffic_weight < 0.0 || v.traffic_weight > 1.0 {
                return Err(ExperimentError::InvalidWeight {
                    name: v.name.clone(),
                    weight: v.traffic_weight,
                });
            }
            if !seen.insert(v.name.clone()) {
                return Err(ExperimentError::DuplicateVariantName(v.name.clone()));
            }
        }
        let total: f64 = variants.iter().map(|v| v.traffic_weight).sum();
        if (total - 1.0).abs() > 1e-9 {
            return Err(ExperimentError::WeightsMustSumToOne { actual: total });
        }
        Ok(Experiment {
            id: id.into(),
            description: description.into(),
            variants,
            metric,
        })
    }

    /// Return the variant with the given name, if it exists.
    pub fn variant(&self, name: &str) -> Option<&Variant> {
        self.variants.iter().find(|v| v.name == name)
    }
}
