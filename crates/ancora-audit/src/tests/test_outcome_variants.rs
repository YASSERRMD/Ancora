use crate::Outcome;
#[test]
fn outcome_variants_are_distinct() {
    assert_ne!(Outcome::Success, Outcome::Failure);
    assert_ne!(Outcome::Failure, Outcome::Blocked);
    assert_ne!(Outcome::Success, Outcome::Blocked);
}
#[test]
fn outcome_clone_equals_original() {
    let o = Outcome::Blocked;
    assert_eq!(o.clone(), Outcome::Blocked);
}
