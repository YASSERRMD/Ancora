use crate::attack::AttackVector;

#[test]
fn network() {
    assert_eq!(AttackVector::Network.to_string(), "NETWORK");
}
#[test]
fn local() {
    assert_eq!(AttackVector::Local.to_string(), "LOCAL");
}
#[test]
fn physical() {
    assert_eq!(AttackVector::Physical.to_string(), "PHYSICAL");
}
#[test]
fn adjacent() {
    assert_eq!(AttackVector::Adjacent.to_string(), "ADJACENT");
}
