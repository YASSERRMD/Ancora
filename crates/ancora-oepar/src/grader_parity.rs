//! Grader parity - validates that graders produce the same scores across language SDKs.

/// A grader evaluates an output against an expected answer.
pub trait Grader {
    fn grade(&self, expected: &str, actual: &str) -> f64;
    fn name(&self) -> &str;
}

/// Exact-match grader: 1.0 if equal, 0.0 otherwise.
#[derive(Debug, Clone)]
pub struct ExactMatchGrader;

impl Grader for ExactMatchGrader {
    fn grade(&self, expected: &str, actual: &str) -> f64 {
        if expected.trim().to_lowercase() == actual.trim().to_lowercase() {
            1.0
        } else {
            0.0
        }
    }

    fn name(&self) -> &str {
        "exact_match"
    }
}

/// Contains-grader: 1.0 if actual contains expected substring.
#[derive(Debug, Clone)]
pub struct ContainsGrader;

impl Grader for ContainsGrader {
    fn grade(&self, expected: &str, actual: &str) -> f64 {
        if actual.to_lowercase().contains(&expected.to_lowercase()) {
            1.0
        } else {
            0.0
        }
    }

    fn name(&self) -> &str {
        "contains"
    }
}

/// F1-score grader based on token overlap.
#[derive(Debug, Clone)]
pub struct F1Grader;

impl F1Grader {
    fn tokenize(text: &str) -> Vec<String> {
        text.split_whitespace()
            .map(|t| t.to_lowercase().trim_matches(|c: char| !c.is_alphanumeric()).to_string())
            .filter(|t| !t.is_empty())
            .collect()
    }
}

impl Grader for F1Grader {
    fn grade(&self, expected: &str, actual: &str) -> f64 {
        let expected_tokens = Self::tokenize(expected);
        let actual_tokens = Self::tokenize(actual);

        if expected_tokens.is_empty() && actual_tokens.is_empty() {
            return 1.0;
        }
        if expected_tokens.is_empty() || actual_tokens.is_empty() {
            return 0.0;
        }

        let common = expected_tokens
            .iter()
            .filter(|t| actual_tokens.contains(t))
            .count();

        let precision = common as f64 / actual_tokens.len() as f64;
        let recall = common as f64 / expected_tokens.len() as f64;

        if precision + recall == 0.0 {
            0.0
        } else {
            2.0 * precision * recall / (precision + recall)
        }
    }

    fn name(&self) -> &str {
        "f1"
    }
}

/// A grader result from one language SDK.
#[derive(Debug, Clone)]
pub struct GraderResult {
    pub language: String,
    pub grader_name: String,
    pub case_id: String,
    pub score: f64,
}

/// Check that grader scores match across languages within tolerance.
pub fn check_grader_parity(results: &[GraderResult], tolerance: f64) -> Vec<String> {
    let mut issues = Vec::new();
    use std::collections::HashMap;

    // Group by (case_id, grader_name)
    let mut grouped: HashMap<(&str, &str), Vec<&GraderResult>> = HashMap::new();
    for r in results {
        grouped.entry((&r.case_id, &r.grader_name)).or_default().push(r);
    }

    for ((case_id, grader), group) in &grouped {
        if let Some(first) = group.first() {
            for other in group.iter().skip(1) {
                let diff = (first.score - other.score).abs();
                if diff > tolerance {
                    issues.push(format!(
                        "grader {:?} on case {:?}: {:?}={:.4} vs {:?}={:.4} (diff={:.4})",
                        grader, case_id,
                        first.language, first.score,
                        other.language, other.score,
                        diff
                    ));
                }
            }
        }
    }

    issues
}

/// Build reference grader results for parity testing.
pub fn reference_grader_results(languages: &[&str]) -> Vec<GraderResult> {
    let graders: &[(&str, f64)] = &[
        ("exact_match", 1.0),
        ("contains", 1.0),
        ("f1", 0.9),
    ];
    let cases = &["case-001", "case-002", "case-003"];

    let mut results = Vec::new();
    for &lang in languages {
        for &(grader, score) in graders {
            for &case_id in cases {
                results.push(GraderResult {
                    language: lang.to_string(),
                    grader_name: grader.to_string(),
                    case_id: case_id.to_string(),
                    score,
                });
            }
        }
    }
    results
}
