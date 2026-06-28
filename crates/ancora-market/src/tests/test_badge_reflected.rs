use crate::badge::{Badge, BadgeKind, BadgeSet};

#[test]
fn awarded_badge_reflected_in_set() {
    let mut set = BadgeSet::new();
    set.award(Badge {
        kind: BadgeKind::SecurityVerified,
        awarded_on: "2026-06-01".to_string(),
        issuer: "ancora-registry".to_string(),
    });
    assert!(set.has(&BadgeKind::SecurityVerified));
    assert_eq!(set.count(), 1);
}

#[test]
fn multiple_distinct_badges_all_reflected() {
    let mut set = BadgeSet::new();
    set.award(Badge {
        kind: BadgeKind::SecurityVerified,
        awarded_on: "2026-06-01".to_string(),
        issuer: "ancora-registry".to_string(),
    });
    set.award(Badge {
        kind: BadgeKind::OpenSourceLicense,
        awarded_on: "2026-06-01".to_string(),
        issuer: "ancora-registry".to_string(),
    });
    set.award(Badge {
        kind: BadgeKind::ResidencyDeclared,
        awarded_on: "2026-06-01".to_string(),
        issuer: "ancora-registry".to_string(),
    });
    assert!(set.has(&BadgeKind::SecurityVerified));
    assert!(set.has(&BadgeKind::OpenSourceLicense));
    assert!(set.has(&BadgeKind::ResidencyDeclared));
    assert_eq!(set.count(), 3);
}

#[test]
fn absent_badge_not_reflected() {
    let set = BadgeSet::new();
    assert!(!set.has(&BadgeKind::OfficiallyReviewed));
}

#[test]
fn all_returns_awarded_badges() {
    let mut set = BadgeSet::new();
    set.award(Badge {
        kind: BadgeKind::HighTrust,
        awarded_on: "2026-06-01".to_string(),
        issuer: "ancora-registry".to_string(),
    });
    assert_eq!(set.all().len(), 1);
    assert_eq!(set.all()[0].kind, BadgeKind::HighTrust);
}
