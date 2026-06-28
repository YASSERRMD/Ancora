use crate::posture::{DomainScore, SecurityPosture};

#[test]
fn empty_posture_score_zero() {
    let p = SecurityPosture::new("t1", 1);
    assert_eq!(p.overall_score(), 0);
    assert_eq!(p.domain_count(), 0);
    assert_eq!(p.total_critical_findings(), 0);
}

#[test]
fn add_domains_and_score() {
    let mut p = SecurityPosture::new("t1", 1);
    p.add_domain(DomainScore::new("hsm", 80, 2, 0));
    p.add_domain(DomainScore::new("airgap", 60, 4, 1));
    assert_eq!(p.domain_count(), 2);
    assert_eq!(p.overall_score(), 70);
    assert_eq!(p.total_critical_findings(), 1);
}

#[test]
fn get_domain() {
    let mut p = SecurityPosture::new("t1", 1);
    p.add_domain(DomainScore::new("hsm", 90, 0, 0));
    assert!(p.get_domain("hsm").is_some());
    assert!(p.get_domain("missing").is_none());
}

#[test]
fn domains_iterator() {
    let mut p = SecurityPosture::new("t1", 1);
    p.add_domain(DomainScore::new("a", 50, 0, 0));
    p.add_domain(DomainScore::new("b", 70, 0, 0));
    assert_eq!(p.domains().count(), 2);
}
