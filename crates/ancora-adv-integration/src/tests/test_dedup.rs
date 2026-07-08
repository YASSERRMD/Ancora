use std::collections::HashSet;

use ancora_lh::BackgroundRun;
use ancora_memcon::Deduplicator;

#[test]
fn combined_zero_duplicate_effects() {
    // BackgroundRun.apply_effect is idempotent: same effect applied twice = no duplicate
    let mut run = BackgroundRun::new("dedup-run", 1);
    run.start();
    run.apply_effect("write-summary");
    run.apply_effect("write-summary");
    run.apply_effect("send-notification");
    run.apply_effect("send-notification");

    let unique: HashSet<&str> = run.effects_applied.iter().map(|s| s.as_str()).collect();
    assert_eq!(
        run.effects_applied.len(),
        unique.len(),
        "duplicate effects found"
    );
    assert_eq!(run.effects_applied.len(), 2);
}

#[test]
fn deduplicator_removes_duplicate_keys() {
    let items = vec![
        "key-a".to_string(),
        "key-b".to_string(),
        "key-a".to_string(),
    ];
    let unique = Deduplicator::dedup(items);
    assert_eq!(unique.len(), 2);

    let mut set: HashSet<&str> = HashSet::new();
    for k in &unique {
        assert!(set.insert(k.as_str()), "duplicate found: {}", k);
    }
}
