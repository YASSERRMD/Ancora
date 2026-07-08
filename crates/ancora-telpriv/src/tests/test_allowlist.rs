use crate::allowlist::Allowlist;

#[test]
fn only_allowlisted_attrs_exported() {
    let al = Allowlist::default_safe();
    let attrs = vec![
        ("span.name".to_string(), "agent_invoke".to_string()),
        ("user.email".to_string(), "carol@example.com".to_string()),
        ("http.status_code".to_string(), "200".to_string()),
        ("prompt.text".to_string(), "raw prompt".to_string()),
    ];
    let allowed = al.filter(&attrs);
    let keys: Vec<&str> = allowed.iter().map(|(k, _)| k.as_str()).collect();

    assert!(keys.contains(&"span.name"));
    assert!(keys.contains(&"http.status_code"));
    assert!(!keys.contains(&"user.email"), "email must be filtered out");
    assert!(
        !keys.contains(&"prompt.text"),
        "prompt must be filtered out"
    );
}

#[test]
fn metric_prefix_allowed() {
    let al = Allowlist::default_safe();
    assert!(al.is_allowed("metric.latency_p99"));
    assert!(al.is_allowed("metric.throughput"));
}

#[test]
fn unknown_attr_blocked() {
    let al = Allowlist::default_safe();
    assert!(!al.is_allowed("custom.mystery_field"));
}

#[test]
fn custom_exact_added() {
    let mut al = Allowlist::empty();
    al.add_exact("app.tenant_id");
    let attrs = vec![
        ("app.tenant_id".to_string(), "t-123".to_string()),
        ("app.user_token".to_string(), "tok-abc".to_string()),
    ];
    let allowed = al.filter(&attrs);
    assert_eq!(allowed.len(), 1);
    assert_eq!(allowed[0].0, "app.tenant_id");
}

#[test]
fn empty_allowlist_blocks_all() {
    let al = Allowlist::empty();
    let attrs = vec![("span.name".to_string(), "test".to_string())];
    let allowed = al.filter(&attrs);
    assert!(allowed.is_empty());
}
