use crate::grader::{Grader, Score};

/// A rubric criterion with a name and a weight.
#[derive(Debug, Clone)]
pub struct Criterion {
    pub name: String,
    pub description: String,
    pub weight: f64,
}

impl Criterion {
    pub fn new(name: impl Into<String>, description: impl Into<String>, weight: f64) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            weight,
        }
    }
}

/// A rubric is a collection of weighted criteria.
#[derive(Debug, Clone, Default)]
pub struct Rubric {
    pub criteria: Vec<Criterion>,
}

impl Rubric {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_criterion(mut self, criterion: Criterion) -> Self {
        self.criteria.push(criterion);
        self
    }
}

/// Scoring function type: given (candidate, expected, criterion_description) returns a score in [0.0, 1.0].
pub type JudgeFn = Box<dyn Fn(&str, &str, &str) -> f64 + Send + Sync>;

/// LLM-as-judge grader that applies a rubric using a pluggable judge function.
///
/// The judge function is called once per criterion. In offline mode a simple
/// keyword-overlap heuristic is used so no network is required.
pub struct LlmJudgeGrader {
    pub rubric: Rubric,
    judge: JudgeFn,
}

impl LlmJudgeGrader {
    /// Build with a custom judge function.
    pub fn new(rubric: Rubric, judge: JudgeFn) -> Self {
        Self { rubric, judge }
    }

    /// Build an offline judge that uses keyword overlap as a proxy for quality.
    pub fn offline(rubric: Rubric) -> Self {
        Self {
            rubric,
            judge: Box::new(|candidate, expected, _desc| {
                let cand_lower = candidate.to_lowercase();
                let exp_words: Vec<&str> = expected.split_whitespace().collect();
                if exp_words.is_empty() {
                    return 0.5;
                }
                let hits = exp_words
                    .iter()
                    .filter(|w| cand_lower.contains(&w.to_lowercase()))
                    .count();
                hits as f64 / exp_words.len() as f64
            }),
        }
    }
}

impl Grader for LlmJudgeGrader {
    fn grade(&self, candidate: &str, expected: &str) -> Score {
        if self.rubric.criteria.is_empty() {
            return Score::new(0.0).with_rationale("No criteria in rubric");
        }

        let total_weight: f64 = self.rubric.criteria.iter().map(|c| c.weight).sum();
        let mut weighted_sum = 0.0;

        for criterion in &self.rubric.criteria {
            let raw = (self.judge)(candidate, expected, &criterion.description);
            let clamped = raw.clamp(0.0, 1.0);
            weighted_sum += clamped * criterion.weight;
        }

        let value = if total_weight > 0.0 {
            (weighted_sum / total_weight).clamp(0.0, 1.0)
        } else {
            0.0
        };

        Score::new(value).with_rationale(format!(
            "Weighted rubric score over {} criteria",
            self.rubric.criteria.len()
        ))
    }

    fn name(&self) -> &str {
        "llm_judge"
    }
}
