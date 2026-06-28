use crate::grader::{Grader, Score};

/// Grader that returns 1.0 when candidate exactly matches expected, 0.0 otherwise.
#[derive(Debug, Clone, Default)]
pub struct ExactMatchGrader {
    /// When true, comparison is case-insensitive.
    pub case_insensitive: bool,
    /// When true, leading/trailing whitespace is stripped before comparison.
    pub trim: bool,
}

impl ExactMatchGrader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn case_insensitive(mut self) -> Self {
        self.case_insensitive = true;
        self
    }

    pub fn trimmed(mut self) -> Self {
        self.trim = true;
        self
    }

    fn normalize<'a>(&self, s: &'a str) -> std::borrow::Cow<'a, str> {
        let s = if self.trim { s.trim() } else { s };
        if self.case_insensitive {
            std::borrow::Cow::Owned(s.to_lowercase())
        } else {
            std::borrow::Cow::Borrowed(s)
        }
    }
}

impl Grader for ExactMatchGrader {
    fn grade(&self, candidate: &str, expected: &str) -> Score {
        let c = self.normalize(candidate);
        let e = self.normalize(expected);
        if c == e {
            Score::new(1.0).with_rationale("Exact match")
        } else {
            Score::new(0.0).with_rationale("No match")
        }
    }

    fn name(&self) -> &str {
        "exact_match"
    }
}
