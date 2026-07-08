use crate::interface::PluginMeta;
use crate::list::{build_entries, render_list, ListEntry, ListOptions};
use crate::update::{UpdateStatus, Version};

fn make_meta(id: &str) -> PluginMeta {
    PluginMeta::new(id, id, "1.0.0", "test plugin", "test")
}

#[test]
fn test_empty_list_shows_no_plugins_message() {
    let opts = ListOptions::default();
    let rendered = render_list(&[], &opts);
    assert!(rendered.contains("No plugins installed"));
}

#[test]
fn test_installed_plugin_appears_in_list() {
    let entries = build_entries(vec![make_meta("plug.one")]);
    let opts = ListOptions::default();
    let rendered = render_list(&entries, &opts);

    assert!(
        rendered.contains("plug.one"),
        "plugin id should appear in list"
    );
}

#[test]
fn test_multiple_plugins_all_appear() {
    let entries = build_entries(vec![
        make_meta("plug.a"),
        make_meta("plug.b"),
        make_meta("plug.c"),
    ]);
    let opts = ListOptions::default();
    let rendered = render_list(&entries, &opts);

    assert!(rendered.contains("plug.a"));
    assert!(rendered.contains("plug.b"));
    assert!(rendered.contains("plug.c"));
}

#[test]
fn test_disabled_plugin_hidden_by_default() {
    let entries = vec![
        ListEntry::new(make_meta("plug.enabled")),
        ListEntry::new(make_meta("plug.disabled")).disabled(),
    ];
    let opts = ListOptions {
        include_disabled: false,
        ..Default::default()
    };

    let rendered = render_list(&entries, &opts);
    assert!(rendered.contains("plug.enabled"));
    assert!(
        !rendered.contains("plug.disabled"),
        "disabled plugin should be hidden by default"
    );
}

#[test]
fn test_disabled_plugin_shown_when_option_set() {
    let entries = vec![ListEntry::new(make_meta("plug.disabled")).disabled()];
    let opts = ListOptions {
        include_disabled: true,
        ..Default::default()
    };

    let rendered = render_list(&entries, &opts);
    assert!(
        rendered.contains("plug.disabled"),
        "disabled plugin should show when include_disabled=true"
    );
}

#[test]
fn test_update_available_shown_in_list() {
    use crate::update::UpdateAvailable;

    let entry = ListEntry::new(make_meta("plug.old")).with_update_status(
        UpdateStatus::UpdateAvailable(UpdateAvailable {
            plugin_id: "plug.old".to_string(),
            installed: Version::parse("1.0.0").unwrap(),
            available: Version::parse("2.0.0").unwrap(),
            update_url: None,
            notes: None,
        }),
    );

    let opts = ListOptions {
        show_updates: true,
        ..Default::default()
    };

    let rendered = render_list(&[entry], &opts);
    assert!(
        rendered.contains("update available"),
        "update status should appear in list"
    );
}

#[test]
fn test_up_to_date_status_shown() {
    let entry =
        ListEntry::new(make_meta("plug.current")).with_update_status(UpdateStatus::UpToDate {
            plugin_id: "plug.current".to_string(),
            version: Version::parse("1.0.0").unwrap(),
        });

    let opts = ListOptions::default();
    let rendered = render_list(&[entry], &opts);
    assert!(
        rendered.contains("up-to-date"),
        "up-to-date status should appear in list"
    );
}
