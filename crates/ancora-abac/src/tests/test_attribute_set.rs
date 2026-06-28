use crate::AttributeSet;
#[test]
fn set_and_get_string() {
    let mut a = AttributeSet::new(); a.set("role", "admin");
    assert_eq!(a.get_str("role"), Some("admin"));
}
#[test]
fn missing_key_returns_none() {
    let a = AttributeSet::new();
    assert!(a.get("missing").is_none());
}
#[test]
fn set_int_and_get() {
    let mut a = AttributeSet::new(); a.set("age", 42i64);
    assert_eq!(a.get_int("age"), Some(42));
}
