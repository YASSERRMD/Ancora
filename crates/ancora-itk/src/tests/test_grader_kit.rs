use crate::grader_kit::{GradeRequest, GradeResult, Grader, GraderKit};

/// Exact-match grader: score 1.0 for identical strings, 0.0 otherwise.
struct ExactMatchGrader;

impl Grader for ExactMatchGrader {
    fn name(&self) -> &str {
        "exact-match"
    }

    fn grade(&self, req: &GradeRequest) -> Result<GradeResult, String> {
        let score = if req.reference.trim() == req.candidate.trim() {
            1.0
        } else {
            0.0
        };
        Ok(GradeResult {
            score,
            rationale: None,
        })
    }
}

#[test]
fn grader_kit_passes_for_exact_match_grader() {
    let kit = GraderKit::new();
    let results = kit.run(&ExactMatchGrader);
    for r in &results {
        assert!(r.passed, "Check failed: {} - {}", r.name, r.message);
    }
    assert_eq!(results.len(), 3);
}

#[test]
fn exact_match_grades_correctly() {
    let grader = ExactMatchGrader;

    let perfect = grader
        .grade(&GradeRequest {
            question: "q".into(),
            reference: "42".into(),
            candidate: "42".into(),
        })
        .unwrap();
    assert!((perfect.score - 1.0).abs() < f64::EPSILON);

    let wrong = grader
        .grade(&GradeRequest {
            question: "q".into(),
            reference: "42".into(),
            candidate: "wrong".into(),
        })
        .unwrap();
    assert!((wrong.score - 0.0).abs() < f64::EPSILON);
}
