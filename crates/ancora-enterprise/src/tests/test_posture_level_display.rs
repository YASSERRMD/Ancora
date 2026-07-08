use crate::posture::PostureLevel;

#[test]
fn critical() {
    assert_eq!(PostureLevel::Critical.to_string(), "CRITICAL");
}
#[test]
fn poor() {
    assert_eq!(PostureLevel::Poor.to_string(), "POOR");
}
#[test]
fn fair() {
    assert_eq!(PostureLevel::Fair.to_string(), "FAIR");
}
#[test]
fn good() {
    assert_eq!(PostureLevel::Good.to_string(), "GOOD");
}
#[test]
fn excellent() {
    assert_eq!(PostureLevel::Excellent.to_string(), "EXCELLENT");
}
