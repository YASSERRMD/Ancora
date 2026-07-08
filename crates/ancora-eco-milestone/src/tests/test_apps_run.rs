use crate::apps_status::{all_apps_ok, sample_app_statuses};

#[test]
fn test_all_sample_apps_run_ok() {
    let statuses = sample_app_statuses();
    assert!(!statuses.is_empty(), "should have app statuses");
    assert!(
        all_apps_ok(&statuses),
        "all sample apps should run without error"
    );
}

#[test]
fn test_app_versions_nonempty() {
    let statuses = sample_app_statuses();
    for s in &statuses {
        assert!(
            !s.version.is_empty(),
            "app {} should have a version",
            s.app_name
        );
        assert!(!s.app_name.is_empty(), "app name should not be empty");
    }
}
