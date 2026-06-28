use crate::{Secret, SecretKind};
#[test]
fn active_value_returns_initial_value() {
    let s = Secret::new("db/pass", "t1", SecretKind::Opaque, "initial", 1);
    assert_eq!(s.active_value(), Some("initial"));
}
