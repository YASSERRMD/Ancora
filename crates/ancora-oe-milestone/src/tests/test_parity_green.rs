use crate::parity_status::{all_parity_green, parity_gaps, ParityEntry, ParityState};

#[test]
fn parity_green_for_all_obs_features() {
    let entries = vec![
        ParityEntry::new("metrics-export", ParityState::Full),
        ParityEntry::new("trace-propagation", ParityState::Full),
        ParityEntry::new("log-correlation", ParityState::Full),
        ParityEntry::new("eval-runner-api", ParityState::Full),
        ParityEntry::new("eval-result-schema", ParityState::Full),
    ];
    assert!(
        all_parity_green(&entries),
        "All features must have full parity at this milestone"
    );
    assert!(parity_gaps(&entries).is_empty());
}

#[test]
fn individual_parity_entry_full() {
    let e = ParityEntry::new("histogram", ParityState::Full);
    assert!(e.is_full_parity());
    assert!(e.describe().contains("full parity"));
}
