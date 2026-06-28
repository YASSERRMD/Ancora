//! Tests: a sample grader plugin loads and grades correctly.

use crate::grader_ext::{GradeRequest, GraderError, GraderPlugin, LengthRatioGrader};
use crate::manifest::{ManifestBuilder, PluginKind, SemVer};

fn build_grader_manifest() -> crate::manifest::PluginManifest {
    ManifestBuilder::new()
        .id("length-ratio-grader")
        .name("Length Ratio Grader")
        .version(SemVer::new(1, 0, 0))
        .sdk_range(SemVer::new(1, 0, 0), SemVer::new(1, 99, 0))
        .kind(PluginKind::Grader)
        .scope("grader:run")
        .build()
        .unwrap()
}

fn make_req(response: &str, reference: &str) -> GradeRequest {
    GradeRequest {
        prompt: "What is 2+2?".into(),
        response: response.into(),
        reference: Some(reference.into()),
        metadata: Default::default(),
    }
}

#[test]
fn perfect_length_scores_one() {
    let g = LengthRatioGrader::new("length-ratio-grader", 0.7);
    let grade = g.grade(make_req("hello", "hello")).unwrap();
    assert!((grade.score - 1.0).abs() < 1e-6);
    assert!(grade.pass);
}

#[test]
fn shorter_response_scores_correctly() {
    let g = LengthRatioGrader::new("length-ratio-grader", 0.5);
    // response length 5, reference length 10 -> ratio 0.5
    let grade = g.grade(make_req("12345", "1234567890")).unwrap();
    assert!((grade.score - 0.5).abs() < 1e-6);
    // With threshold 0.5, exactly 0.5 should pass.
    assert!(grade.pass);
}

#[test]
fn missing_reference_returns_error() {
    let g = LengthRatioGrader::new("length-ratio-grader", 0.7);
    let req = GradeRequest {
        prompt: "test".into(),
        response: "abc".into(),
        reference: None,
        metadata: Default::default(),
    };
    let err = g.grade(req).unwrap_err();
    assert!(matches!(err, GraderError::InvalidInput(_)));
}

#[test]
fn grader_manifest_has_correct_kind() {
    let m = build_grader_manifest();
    assert_eq!(m.kind, PluginKind::Grader);
}

#[test]
fn grader_pass_threshold_respected() {
    let g = LengthRatioGrader::new("length-ratio-grader", 0.8);
    // 5 / 10 = 0.5, threshold 0.8 -> fail
    let grade = g.grade(make_req("12345", "1234567890")).unwrap();
    assert!(!grade.pass);
}
