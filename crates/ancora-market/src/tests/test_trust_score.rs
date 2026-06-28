use crate::badge::BadgeSet;
use crate::identity::AuthorIdentity;
use crate::license::LicenseDeclaration;
use crate::residency::{Region, ResidencyDeclaration};
use crate::security_scan::{ScanReport, ScanStatus};
use crate::trust_score::{TrustSignals, compute_trust_score};

fn verified_author() -> AuthorIdentity {
    let mut a = AuthorIdentity::new("trusted-author", "Trusted Author", "PK").unwrap();
    a.mark_verified();
    a
}

#[test]
fn trust_score_computed_with_all_signals() {
    let author = verified_author();
    let scan = ScanReport::new("scanner", "2026-06-01", ScanStatus::Clean);
    let license = LicenseDeclaration::new("Apache-2.0", true).unwrap();
    let residency = ResidencyDeclaration::new(vec![Region::EEA], vec![Region::EEA]);
    let badges = BadgeSet::new();
    let signals = TrustSignals {
        author: &author,
        scan_report: Some(&scan),
        license: Some(&license),
        residency: Some(&residency),
        badges: &badges,
        version_count: 3,
    };
    let score = compute_trust_score(&signals);
    // identity(20) + security(30) + license(15) + residency(15) + badges(0) + history(6)
    assert_eq!(score.score, 86);
    assert!(score.is_high_trust());
}

#[test]
fn trust_score_zero_for_no_signals() {
    let author = AuthorIdentity::new("anon", "Anon", "PK").unwrap();
    let badges = BadgeSet::new();
    let signals = TrustSignals {
        author: &author,
        scan_report: None,
        license: None,
        residency: None,
        badges: &badges,
        version_count: 0,
    };
    let score = compute_trust_score(&signals);
    assert_eq!(score.score, 5); // identity=5 for unverified
    assert!(!score.is_acceptable());
}

#[test]
fn trust_score_breakdown_sums_correctly() {
    let author = verified_author();
    let scan = ScanReport::new("scanner", "2026-06-01", ScanStatus::Clean);
    let badges = BadgeSet::new();
    let signals = TrustSignals {
        author: &author,
        scan_report: Some(&scan),
        license: None,
        residency: None,
        badges: &badges,
        version_count: 0,
    };
    let score = compute_trust_score(&signals);
    let b = &score.breakdown;
    let sum = b.identity + b.security + b.license + b.residency + b.badges + b.history;
    assert_eq!(score.score, sum);
}
