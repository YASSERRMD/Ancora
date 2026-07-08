use crate::cluster::{cluster_failures, Failure};

#[test]
fn cluster_groups_similar_failures() {
    let failures = vec![
        Failure {
            case_id: "c1".into(),
            reason: "expected foo got bar".into(),
        },
        Failure {
            case_id: "c2".into(),
            reason: "expected foo got baz".into(),
        },
        Failure {
            case_id: "c3".into(),
            reason: "timeout after 5s".into(),
        },
        Failure {
            case_id: "c4".into(),
            reason: "timeout after 10s".into(),
        },
        Failure {
            case_id: "c5".into(),
            reason: "expected foo got qux".into(),
        },
    ];

    // min_shared_tokens=2: "expected foo" should cluster c1, c2, c5 together.
    let clusters = cluster_failures(&failures, 2);
    assert!(!clusters.is_empty(), "should produce at least one cluster");

    // Find the largest cluster.
    let largest = clusters.iter().max_by_key(|c| c.count).unwrap();
    assert!(
        largest.count >= 2,
        "largest cluster should have at least 2 members"
    );
}

#[test]
fn cluster_empty_failures() {
    let clusters = cluster_failures(&[], 1);
    assert!(clusters.is_empty(), "empty input -> empty clusters");
}

#[test]
fn cluster_no_sharing_all_singletons() {
    let failures = vec![
        Failure {
            case_id: "c1".into(),
            reason: "alpha".into(),
        },
        Failure {
            case_id: "c2".into(),
            reason: "beta".into(),
        },
        Failure {
            case_id: "c3".into(),
            reason: "gamma".into(),
        },
    ];
    // min_shared_tokens=2 means single-word messages never share enough.
    let clusters = cluster_failures(&failures, 2);
    assert_eq!(
        clusters.len(),
        3,
        "all singletons expected, got {} clusters",
        clusters.len()
    );
}

#[test]
fn cluster_sorted_by_count_desc() {
    let failures = vec![
        Failure {
            case_id: "c1".into(),
            reason: "network error occurred".into(),
        },
        Failure {
            case_id: "c2".into(),
            reason: "network error timeout".into(),
        },
        Failure {
            case_id: "c3".into(),
            reason: "network error failed".into(),
        },
        Failure {
            case_id: "c4".into(),
            reason: "parse failure".into(),
        },
    ];
    let clusters = cluster_failures(&failures, 2);
    let counts: Vec<usize> = clusters.iter().map(|c| c.count).collect();
    for i in 1..counts.len() {
        assert!(
            counts[i - 1] >= counts[i],
            "clusters not sorted descending: {:?}",
            counts
        );
    }
}
