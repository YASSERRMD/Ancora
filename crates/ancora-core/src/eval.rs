/// An evaluation case: an input, expected output, and a scorer name.
#[derive(Debug, Clone)]
pub struct EvalCase {
    pub id: String,
    pub input: String,
    pub expected: String,
    pub scorer: String,
}

impl EvalCase {
    pub fn new(
        id: impl Into<String>,
        input: impl Into<String>,
        expected: impl Into<String>,
        scorer: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            input: input.into(),
            expected: expected.into(),
            scorer: scorer.into(),
        }
    }
}

/// A scorer receives a candidate answer and the expected answer and returns
/// a score in `[0.0, 1.0]`.
pub trait EvalScorer: Send + Sync {
    fn name(&self) -> &str;
    fn score(&self, candidate: &str, expected: &str) -> f64;
}

/// Exact-match scorer: 1.0 if strings are equal, 0.0 otherwise.
pub struct ExactMatchScorer;

impl EvalScorer for ExactMatchScorer {
    fn name(&self) -> &str {
        "exact_match"
    }
    fn score(&self, candidate: &str, expected: &str) -> f64 {
        if candidate.trim() == expected.trim() {
            1.0
        } else {
            0.0
        }
    }
}

/// Contains-match scorer: 1.0 if expected is a substring of candidate.
pub struct ContainsScorer;

/// Normalized edit-distance scorer: 1 - edit_distance / max_len.
pub struct NormalizedEditScorer;

fn edit_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let (m, n) = (a.len(), b.len());
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }
    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = if a[i - 1] == b[j - 1] {
                dp[i - 1][j - 1]
            } else {
                1 + dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i][j - 1])
            };
        }
    }
    dp[m][n]
}

impl EvalScorer for NormalizedEditScorer {
    fn name(&self) -> &str {
        "normalized_edit"
    }
    fn score(&self, candidate: &str, expected: &str) -> f64 {
        let max_len = candidate.len().max(expected.len());
        if max_len == 0 {
            return 1.0;
        }
        1.0 - edit_distance(candidate, expected) as f64 / max_len as f64
    }
}

impl EvalScorer for ContainsScorer {
    fn name(&self) -> &str {
        "contains"
    }
    fn score(&self, candidate: &str, expected: &str) -> f64 {
        if candidate.contains(expected) {
            1.0
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_case_stores_fields() {
        let c = EvalCase::new("id1", "inp", "exp", "exact_match");
        assert_eq!(c.id, "id1");
        assert_eq!(c.input, "inp");
        assert_eq!(c.expected, "exp");
        assert_eq!(c.scorer, "exact_match");
    }

    #[test]
    fn exact_match_scorer_matches_equal_strings() {
        let s = ExactMatchScorer;
        assert_eq!(s.score("hello", "hello"), 1.0);
    }

    #[test]
    fn exact_match_scorer_fails_different_strings() {
        let s = ExactMatchScorer;
        assert_eq!(s.score("hello", "world"), 0.0);
    }

    #[test]
    fn exact_match_scorer_trims_whitespace() {
        let s = ExactMatchScorer;
        assert_eq!(s.score("  hello  ", "hello"), 1.0);
    }

    #[test]
    fn contains_scorer_matches_substring() {
        let s = ContainsScorer;
        assert_eq!(s.score("the answer is 42", "42"), 1.0);
    }

    #[test]
    fn contains_scorer_fails_absent_substring() {
        let s = ContainsScorer;
        assert_eq!(s.score("no match here", "42"), 0.0);
    }

    #[test]
    fn normalized_edit_scorer_identical_is_one() {
        let s = NormalizedEditScorer;
        assert!((s.score("hello", "hello") - 1.0).abs() < 1e-9);
    }

    #[test]
    fn normalized_edit_scorer_completely_different_less_than_one() {
        let s = NormalizedEditScorer;
        let v = s.score("abc", "xyz");
        assert!(v < 1.0 && v >= 0.0);
    }

    #[test]
    fn normalized_edit_scorer_empty_both_is_one() {
        let s = NormalizedEditScorer;
        assert!((s.score("", "") - 1.0).abs() < 1e-9);
    }
}
