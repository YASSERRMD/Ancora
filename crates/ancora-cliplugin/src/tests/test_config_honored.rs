use crate::config::{parse_config_text, ConfigStore, PluginConfig};

#[test]
fn test_config_set_and_get() {
    let mut cfg = PluginConfig::new("my.plugin");
    cfg.set("timeout", "30");
    cfg.set("retries", "3");

    assert_eq!(cfg.get("timeout"), Some("30"));
    assert_eq!(cfg.get("retries"), Some("3"));
    assert_eq!(cfg.get("missing"), None);
}

#[test]
fn test_config_get_or_default() {
    let cfg = PluginConfig::new("my.plugin");
    assert_eq!(cfg.get_or_default("key", "fallback"), "fallback");
}

#[test]
fn test_config_merge_overwrites_existing_key() {
    let mut base = PluginConfig::new("my.plugin");
    base.set("key", "old");
    base.set("other", "unchanged");

    let mut incoming = PluginConfig::new("my.plugin");
    incoming.set("key", "new");

    base.merge(&incoming);

    assert_eq!(base.get("key"), Some("new"));
    assert_eq!(base.get("other"), Some("unchanged"));
}

#[test]
fn test_parse_config_text() {
    let text = r#"
# comment
timeout = "30"
retries = "3"
feature = "enabled"
"#;
    let cfg = parse_config_text("my.plugin", text);

    assert_eq!(cfg.get("timeout"), Some("30"));
    assert_eq!(cfg.get("retries"), Some("3"));
    assert_eq!(cfg.get("feature"), Some("enabled"));
    assert_eq!(cfg.plugin_id, "my.plugin");
}

#[test]
fn test_config_store_insert_and_retrieve() {
    let mut store = ConfigStore::new();
    let mut cfg = PluginConfig::new("plug.a");
    cfg.set("k", "v");
    store.insert(cfg);

    let retrieved = store.get("plug.a").expect("should find config");
    assert_eq!(retrieved.get("k"), Some("v"));
}

#[test]
fn test_config_store_merge() {
    let mut store = ConfigStore::new();
    let mut base = PluginConfig::new("plug.a");
    base.set("a", "1");
    store.insert(base);

    let mut incoming = PluginConfig::new("plug.a");
    incoming.set("b", "2");
    store.merge(incoming);

    let cfg = store.get("plug.a").unwrap();
    assert_eq!(cfg.get("a"), Some("1"));
    assert_eq!(cfg.get("b"), Some("2"));
}

#[test]
fn test_plugin_uses_config_value() {
    // Simulate a plugin reading its configured timeout.
    let mut cfg = PluginConfig::new("plug.timeout-test");
    cfg.set("timeout_ms", "500");

    let timeout: u64 = cfg
        .get("timeout_ms")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1000);

    assert_eq!(timeout, 500, "plugin should honor configured timeout");
}
