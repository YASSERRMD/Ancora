use crate::badge::BadgeSet;
use crate::policy::{evaluate_policy, InstallPolicy, PolicyMode, PolicyVerdict};
use crate::trust_score::{TrustBreakdown, TrustScore};

fn low_trust() -> TrustScore {
    TrustScore {
        score: 15,
        breakdown: TrustBreakdown {
            identity: 5,
            security: 0,
            license: 0,
            residency: 0,
            badges: 0,
            history: 0,
        },
    }
}

fn medium_trust() -> TrustScore {
    TrustScore {
        score: 55,
        breakdown: TrustBreakdown {
            identity: 20,
            security: 20,
            license: 15,
            residency: 0,
            badges: 0,
            history: 0,
        },
    }
}

fn high_trust() -> TrustScore {
    TrustScore {
        score: 90,
        breakdown: TrustBreakdown {
            identity: 20,
            security: 30,
            license: 15,
            residency: 15,
            badges: 10,
            history: 0,
        },
    }
}

#[test]
fn strict_mode_blocks_low_trust() {
    let policy = InstallPolicy {
        mode: PolicyMode::Strict,
        min_trust_score: 50,
        required_badges: Vec::new(),
        allowed_regions: Vec::new(),
    };
    let badges = BadgeSet::new();
    let verdict = evaluate_policy(&policy, &low_trust(), &badges, None);
    assert_eq!(verdict.is_blocked(), true);
    if let PolicyVerdict::Block(reasons) = verdict {
        assert!(!reasons.is_empty());
    }
}

#[test]
fn warn_mode_warns_on_low_trust_but_allows() {
    let policy = InstallPolicy {
        mode: PolicyMode::Warn,
        min_trust_score: 50,
        required_badges: Vec::new(),
        allowed_regions: Vec::new(),
    };
    let badges = BadgeSet::new();
    let verdict = evaluate_policy(&policy, &low_trust(), &badges, None);
    assert!(verdict.is_allowed());
    assert!(matches!(verdict, PolicyVerdict::Warn(_)));
}

#[test]
fn strict_mode_allows_high_trust() {
    let policy = InstallPolicy {
        mode: PolicyMode::Strict,
        min_trust_score: 50,
        required_badges: Vec::new(),
        allowed_regions: Vec::new(),
    };
    let badges = BadgeSet::new();
    let verdict = evaluate_policy(&policy, &high_trust(), &badges, None);
    assert_eq!(verdict, PolicyVerdict::Allow);
}

#[test]
fn permissive_mode_allows_everything() {
    let policy = InstallPolicy {
        mode: PolicyMode::Permissive,
        min_trust_score: 80,
        required_badges: Vec::new(),
        allowed_regions: Vec::new(),
    };
    let badges = BadgeSet::new();
    let verdict = evaluate_policy(&policy, &low_trust(), &badges, None);
    assert_eq!(verdict, PolicyVerdict::Allow);
}

#[test]
fn strict_medium_trust_above_threshold_allowed() {
    let policy = InstallPolicy {
        mode: PolicyMode::Strict,
        min_trust_score: 50,
        required_badges: Vec::new(),
        allowed_regions: Vec::new(),
    };
    let badges = BadgeSet::new();
    let verdict = evaluate_policy(&policy, &medium_trust(), &badges, None);
    assert!(verdict.is_allowed());
}
