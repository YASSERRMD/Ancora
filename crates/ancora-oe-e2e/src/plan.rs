//! Test plan module: describes the acceptance criteria for the obs and eval e2e suite.

/// A single acceptance criterion.
#[derive(Debug, Clone)]
pub struct Criterion {
    pub id: String,
    pub description: String,
    pub required: bool,
}

impl Criterion {
    pub fn required(id: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            required: true,
        }
    }

    pub fn optional(id: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            required: false,
        }
    }
}

/// The full test plan for phase 239.
#[derive(Debug, Default)]
pub struct TestPlan {
    pub name: String,
    pub criteria: Vec<Criterion>,
}

impl TestPlan {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            criteria: Vec::new(),
        }
    }

    pub fn add(&mut self, criterion: Criterion) {
        self.criteria.push(criterion);
    }

    pub fn required_count(&self) -> usize {
        self.criteria.iter().filter(|c| c.required).count()
    }

    pub fn optional_count(&self) -> usize {
        self.criteria.iter().filter(|c| !c.required).count()
    }
}

/// Build the canonical phase-239 test plan.
pub fn phase_239_plan() -> TestPlan {
    let mut plan = TestPlan::new("Phase 239 - Obs and Eval E2E");

    plan.add(Criterion::required(
        "P239-01",
        "Run produces a complete trace",
    ));
    plan.add(Criterion::required(
        "P239-02",
        "Trace exports to mock collector",
    ));
    plan.add(Criterion::required(
        "P239-03",
        "Cost analytics reflect a run",
    ));
    plan.add(Criterion::required("P239-04", "Eval run scores a suite"));
    plan.add(Criterion::required(
        "P239-05",
        "Regression gate blocks a bad change",
    ));
    plan.add(Criterion::required(
        "P239-06",
        "Drift detected on shifted input",
    ));
    plan.add(Criterion::required("P239-07", "A/B experiment concludes"));
    plan.add(Criterion::required(
        "P239-08",
        "Feedback feeds an eval dataset",
    ));
    plan.add(Criterion::required(
        "P239-09",
        "Studio renders a run end to end",
    ));
    plan.add(Criterion::required(
        "P239-10",
        "Safety monitor flags an unsafe output",
    ));
    plan.add(Criterion::required("P239-11", "Telemetry redaction holds"));
    plan.add(Criterion::required(
        "P239-12",
        "Continuous eval tracks quality",
    ));
    plan.add(Criterion::required(
        "P239-13",
        "All offline with local judge",
    ));
    plan.add(Criterion::required(
        "P239-14",
        "Residency respected by exporters",
    ));
    plan.add(Criterion::required(
        "P239-15",
        "Cross-language trace stitching",
    ));
    plan.add(Criterion::required(
        "P239-16",
        "Zero sensitive data in telemetry",
    ));
    plan.add(Criterion::optional(
        "P239-17",
        "Observability overhead measured",
    ));
    plan.add(Criterion::required(
        "P239-18",
        "Determinism of traces and evals on replay",
    ));

    plan
}
