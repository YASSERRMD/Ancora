use crate::failover::{ProviderFailover, ProviderStatus};

#[test]
fn first_provider_is_active_initially() {
    let f = ProviderFailover::new(vec!["openai".into(), "anthropic".into()]);
    assert_eq!(f.active_provider(), Some("openai"));
}

#[test]
fn failover_switches_to_next_active() {
    let mut f = ProviderFailover::new(vec!["a".into(), "b".into()]);
    let next = f.failover();
    assert_eq!(next, Some("b"));
}

#[test]
fn mark_failed_removes_from_active() {
    let mut f = ProviderFailover::new(vec!["a".into(), "b".into()]);
    f.mark_failed("a");
    assert_eq!(f.active_provider(), Some("b"));
}

#[test]
fn no_active_when_all_failed() {
    let mut f = ProviderFailover::new(vec!["a".into()]);
    f.mark_failed("a");
    assert_eq!(f.active_provider(), None);
}

#[test]
fn status_returns_correct_enum() {
    let f = ProviderFailover::new(vec!["a".into()]);
    assert_eq!(f.status("a"), Some(&ProviderStatus::Active));
}
