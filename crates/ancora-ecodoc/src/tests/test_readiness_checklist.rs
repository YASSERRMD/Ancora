//! Tests for the ecosystem readiness checklist module.

use crate::readiness::{criteria_for, is_ready, Milestone};

#[test]
fn alpha_criteria_are_a_subset_of_stable() {
    let alpha = criteria_for(&Milestone::AlphaRelease);
    let stable = criteria_for(&Milestone::StableRelease);
    for c in &alpha {
        assert!(
            stable.iter().any(|s| s.id == c.id),
            "alpha criterion {} not found in stable criteria",
            c.id
        );
    }
}

#[test]
fn marketplace_has_most_criteria() {
    let market = criteria_for(&Milestone::MarketplaceListing);
    let alpha = criteria_for(&Milestone::AlphaRelease);
    assert!(market.len() >= alpha.len());
}

#[test]
fn satisfying_all_alpha_criteria_passes() {
    let milestone = Milestone::AlphaRelease;
    let ids: Vec<&str> = criteria_for(&milestone).iter().map(|c| c.id).collect();
    assert!(is_ready(&milestone, &ids).is_ok());
}

#[test]
fn empty_satisfied_set_fails_every_milestone() {
    for milestone in [
        Milestone::AlphaRelease,
        Milestone::BetaRelease,
        Milestone::StableRelease,
        Milestone::MarketplaceListing,
    ] {
        assert!(
            is_ready(&milestone, &[]).is_err(),
            "{milestone} should fail with empty satisfied set"
        );
    }
}

#[test]
fn is_ready_returns_only_missing_ids() {
    let milestone = Milestone::MarketplaceListing;
    let all_ids: Vec<&str> = criteria_for(&milestone).iter().map(|c| c.id).collect();

    // Provide all except the last.
    if let Some((&last, rest)) = all_ids.split_last() {
        let result = is_ready(&milestone, rest);
        assert!(result.is_err());
        let missing = result.unwrap_err();
        assert_eq!(missing, vec![last]);
    }
}

#[test]
fn milestone_display() {
    assert_eq!(Milestone::StableRelease.to_string(), "stable-release");
    assert_eq!(
        Milestone::MarketplaceListing.to_string(),
        "marketplace-listing"
    );
}
