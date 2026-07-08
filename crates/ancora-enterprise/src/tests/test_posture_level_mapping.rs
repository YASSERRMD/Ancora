use crate::posture::{DomainScore, PostureLevel, SecurityPosture};

fn posture_with_score(score: u8) -> SecurityPosture {
    let mut p = SecurityPosture::new("t1", 1);
    p.add_domain(DomainScore::new("domain", score, 0, 0));
    p
}

#[test]
fn score_0_is_critical() {
    assert_eq!(
        posture_with_score(0).posture_level(),
        PostureLevel::Critical
    );
}

#[test]
fn score_29_is_critical() {
    assert_eq!(
        posture_with_score(29).posture_level(),
        PostureLevel::Critical
    );
}

#[test]
fn score_30_is_poor() {
    assert_eq!(posture_with_score(30).posture_level(), PostureLevel::Poor);
}

#[test]
fn score_50_is_fair() {
    assert_eq!(posture_with_score(50).posture_level(), PostureLevel::Fair);
}

#[test]
fn score_70_is_good() {
    assert_eq!(posture_with_score(70).posture_level(), PostureLevel::Good);
}

#[test]
fn score_85_is_excellent() {
    assert_eq!(
        posture_with_score(85).posture_level(),
        PostureLevel::Excellent
    );
}
