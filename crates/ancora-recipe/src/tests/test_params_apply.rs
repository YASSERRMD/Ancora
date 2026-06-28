use crate::params::{apply_override, ParamSet};
use crate::{code_review, debate, rag_citations, research_report};

#[test]
fn params_override_top_k() {
    let mut ps = ParamSet::new();
    apply_override(&mut ps, "top_k=7").unwrap();
    assert_eq!(ps.get("top_k"), Some("7"));
    // Recipe sees the override
    let r = rag_citations::build(&ps);
    assert!(r.validate().is_ok());
}

#[test]
fn params_override_sections() {
    let mut ps = ParamSet::new();
    apply_override(&mut ps, "sections=6").unwrap();
    let r = research_report::build(&ps);
    assert_eq!(r.step_count(), 4); // steps count doesn't change - sections affect description
}

#[test]
fn params_override_strict_mode() {
    let mut ps = ParamSet::new();
    apply_override(&mut ps, "strict=true").unwrap();
    apply_override(&mut ps, "language=python").unwrap();
    let r = code_review::build(&ps);
    assert!(r.validate().is_ok());
    assert!(r.description.contains("strict"));
}

#[test]
fn params_override_debate_rounds() {
    let mut ps = ParamSet::new();
    apply_override(&mut ps, "rounds=4").unwrap();
    let r = debate::build(&ps);
    // setup + 4 rounds + verdict = 6
    assert_eq!(r.step_count(), 6);
}

#[test]
fn params_merge_layers() {
    // merge(other) overwrites with other's values; use the right layering order.
    // Build a base with defaults, then merge user overrides on top.
    let mut base = ParamSet::from_pairs([("top_k", "5"), ("corpus", "default")]);
    let user_overrides = ParamSet::from_pairs([("top_k", "10")]);
    base.merge(&user_overrides);
    // user override wins for top_k; corpus comes from defaults
    assert_eq!(base.get("top_k"), Some("10"));
    assert_eq!(base.get("corpus"), Some("default"));
}

#[test]
fn malformed_override_returns_error() {
    let mut ps = ParamSet::new();
    assert!(apply_override(&mut ps, "no-equals").is_err());
    assert!(apply_override(&mut ps, "").is_err());
}
