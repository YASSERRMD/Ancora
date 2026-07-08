use crate::index::entry_count;
use crate::readiness::{readiness_checklist, readiness_percent};
use crate::trust_summary::{governance_score, trust_dimensions};

#[test]
fn test_readiness_is_100_percent() {
    let checklist = readiness_checklist();
    let pct = readiness_percent(&checklist);
    assert_eq!(pct, 100, "readiness should be 100%");
}

#[test]
fn test_governance_score_is_100() {
    let dims = trust_dimensions();
    let score = governance_score(&dims);
    assert_eq!(score, 100, "governance score should be 100");
}

#[test]
fn test_ecosystem_index_has_all_modules() {
    let count = entry_count();
    assert!(
        count >= 13,
        "ecosystem index should have at least 13 entries, got {}",
        count
    );
}
