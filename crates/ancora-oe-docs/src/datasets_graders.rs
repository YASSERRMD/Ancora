//! Dataset management and grader definitions for evaluation.

/// A single evaluation example.
#[derive(Debug, Clone)]
pub struct EvalSample {
    pub id: String,
    pub input: String,
    pub expected_output: Option<String>,
    pub tags: Vec<String>,
}

impl EvalSample {
    pub fn new(id: impl Into<String>, input: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            input: input.into(),
            expected_output: None,
            tags: Vec::new(),
        }
    }

    pub fn with_expected(mut self, expected: impl Into<String>) -> Self {
        self.expected_output = Some(expected.into());
        self
    }

    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }
}

/// A dataset is a named collection of eval samples.
#[derive(Debug, Default)]
pub struct Dataset {
    pub name: String,
    pub samples: Vec<EvalSample>,
}

impl Dataset {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            samples: Vec::new(),
        }
    }

    pub fn add(&mut self, sample: EvalSample) {
        self.samples.push(sample);
    }

    pub fn len(&self) -> usize {
        self.samples.len()
    }

    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }
}

/// Trait implemented by all graders.
pub trait Grader: Send + Sync {
    fn grade(&self, actual: &str, expected: &str) -> f64;
    fn id(&self) -> &str;
}

/// Exact-match grader: returns 1.0 if strings are equal, 0.0 otherwise.
pub struct ExactMatchGrader;

impl Grader for ExactMatchGrader {
    fn grade(&self, actual: &str, expected: &str) -> f64 {
        if actual.trim() == expected.trim() {
            1.0
        } else {
            0.0
        }
    }

    fn id(&self) -> &str {
        "exact_match"
    }
}

/// Substring-match grader: returns 1.0 if expected is a substring of actual.
pub struct SubstringGrader;

impl Grader for SubstringGrader {
    fn grade(&self, actual: &str, expected: &str) -> f64 {
        if actual.contains(expected) {
            1.0
        } else {
            0.0
        }
    }

    fn id(&self) -> &str {
        "substring_match"
    }
}
