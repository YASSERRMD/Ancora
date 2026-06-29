//! Tests: residency-aware sync blocks disallowed transfers.

use crate::journal::Journal;
use crate::model::ResidencyZone;
use crate::residency::{ResidencyFilter, ResidencyTag};

#[test]
fn test_residency_aware_sync_blocks_disallowed_transfer() {
    let mut journal = Journal::new("device-eu");
    let seq_eu = journal.append("eu-data", b"private".to_vec());
    let seq_gl = journal.append("global-data", b"public".to_vec());

    let tags = vec![
        ResidencyTag { seq: seq_eu, zone: ResidencyZone::Region("EU".into()) },
        ResidencyTag { seq: seq_gl, zone: ResidencyZone::Global },
    ];

    // Hub is in the US -- should not receive EU-restricted data.
    let filter = ResidencyFilter::new(ResidencyZone::Region("US".into()), tags);
    let allowed = filter.filter_allowed(journal.all_entries());

    // Only the global entry should be allowed.
    assert_eq!(allowed.len(), 1);
    assert_eq!(allowed[0].key, "global-data");
}

#[test]
fn test_residency_global_allowed_everywhere() {
    let mut journal = Journal::new("device-x");
    let seq = journal.append("global-key", b"data".to_vec());

    let tags = vec![ResidencyTag { seq, zone: ResidencyZone::Global }];
    let filter = ResidencyFilter::new(ResidencyZone::Region("APAC".into()), tags);
    let allowed = filter.filter_allowed(journal.all_entries());
    assert_eq!(allowed.len(), 1);
}

#[test]
fn test_residency_local_never_leaves_device() {
    let mut journal = Journal::new("device-y");
    let seq = journal.append("local-key", b"secret".to_vec());

    let tags = vec![ResidencyTag { seq, zone: ResidencyZone::Local }];
    // Even if hub is in the same region, local data must never leave.
    let filter = ResidencyFilter::new(ResidencyZone::Region("EU".into()), tags);
    let allowed = filter.filter_allowed(journal.all_entries());
    assert_eq!(allowed.len(), 0);
}

#[test]
fn test_residency_build_request_filters() {
    let mut journal = Journal::new("device-z");
    let s1 = journal.append("k1", b"a".to_vec());
    let s2 = journal.append("k2", b"b".to_vec());

    let tags = vec![
        ResidencyTag { seq: s1, zone: ResidencyZone::Local },
        ResidencyTag { seq: s2, zone: ResidencyZone::Global },
    ];
    let filter = ResidencyFilter::new(ResidencyZone::Global, tags);
    let req = filter.build_request("device-z", journal.all_entries());
    assert_eq!(req.entries.len(), 1);
    assert_eq!(req.entries[0].key, "k2");
}
