use crate::posture::DomainScore;

#[test]
fn basic_fields() {
    let s = DomainScore::new("hsm", 85, 3, 1);
    assert_eq!(s.domain, "hsm");
    assert_eq!(s.score, 85);
    assert_eq!(s.findings, 3);
    assert_eq!(s.critical_findings, 1);
}

#[test]
fn score_capped_at_100() {
    let s = DomainScore::new("test", 150, 0, 0);
    assert_eq!(s.score, 100);
}
