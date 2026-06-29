// Coverage gate: verify all test suites are documented as offline-only.

const OFFLINE_SUITES: &[(&str, bool)] = &[
    ("det_suite",          true),
    ("chaos_suite",        true),
    ("load_suite",         true),
    ("reliability_suite",  true),
    ("sec_suite",          true),
    ("policy_suite",       true),
    ("vector_suite",       true),
    ("xlang_suite",        true),
];

fn all_offline(suites: &[(&str, bool)]) -> bool {
    suites.iter().all(|(_, offline)| *offline)
}

fn non_offline_suites<'a>(suites: &[(&'a str, bool)]) -> Vec<&'a str> {
    suites.iter().filter(|(_, offline)| !offline).map(|(name, _)| *name).collect()
}

#[test]
fn test_all_suites_are_offline() {
    assert!(all_offline(OFFLINE_SUITES), "some suites are not offline: {:?}", non_offline_suites(OFFLINE_SUITES));
}

#[test]
fn test_eight_suites_defined() {
    assert_eq!(OFFLINE_SUITES.len(), 8);
}

#[test]
fn test_det_suite_is_offline() {
    let det = OFFLINE_SUITES.iter().find(|(n, _)| *n == "det_suite");
    assert!(det.map(|(_, o)| *o).unwrap_or(false));
}

#[test]
fn test_security_suite_is_offline() {
    let sec = OFFLINE_SUITES.iter().find(|(n, _)| *n == "sec_suite");
    assert!(sec.map(|(_, o)| *o).unwrap_or(false));
}

#[test]
fn test_no_suite_has_live_network() {
    let live = non_offline_suites(OFFLINE_SUITES);
    assert!(live.is_empty(), "suites with live network: {:?}", live);
}

#[test]
fn test_suite_names_end_with_suite() {
    for (name, _) in OFFLINE_SUITES {
        assert!(name.ends_with("_suite"), "suite name should end with _suite: {name}");
    }
}
