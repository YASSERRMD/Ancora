use crate::exposure::{Exposure, ExposureLog};

#[test]
fn exposure_is_recorded_and_retrievable() {
    let mut log = ExposureLog::new();
    let exp = Exposure::with_timestamp("exp-1", "user-42", "control", 1_000_000);
    log.record(exp);

    let entries: Vec<_> = log.for_experiment("exp-1").collect();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].subject_key, "user-42");
    assert_eq!(entries[0].variant_name, "control");
}

#[test]
fn multiple_exposures_tracked_per_variant() {
    let mut log = ExposureLog::new();
    for i in 0..5 {
        log.record(Exposure::with_timestamp(
            "exp-2",
            format!("u-{i}"),
            "treatment",
            0,
        ));
    }
    for i in 5..8 {
        log.record(Exposure::with_timestamp(
            "exp-2",
            format!("u-{i}"),
            "control",
            0,
        ));
    }

    let treatment_count = log.for_variant("exp-2", "treatment").count();
    let control_count = log.for_variant("exp-2", "control").count();

    assert_eq!(treatment_count, 5);
    assert_eq!(control_count, 3);
}

#[test]
fn unique_subjects_counted_correctly() {
    let mut log = ExposureLog::new();
    // user-1 exposed twice (duplicate)
    log.record(Exposure::with_timestamp("exp-3", "user-1", "a", 0));
    log.record(Exposure::with_timestamp("exp-3", "user-1", "a", 1));
    log.record(Exposure::with_timestamp("exp-3", "user-2", "a", 0));

    let unique = log.unique_subjects_for_variant("exp-3", "a");
    assert_eq!(unique, 2, "only 2 unique subjects despite 3 records");
}

#[test]
fn exposure_now_has_nonzero_timestamp() {
    let exp = Exposure::now("exp-ts", "u-1", "control");
    assert!(exp.timestamp_secs > 0, "timestamp should be > 0");
}
