// Policy: data retention -- journals older than TTL are eligble for deletion.

struct JournalRecord {
    run_id: String,
    created_at_ns: u64,
}

fn retention_eligible(record: &JournalRecord, now_ns: u64, ttl_ns: u64) -> bool {
    now_ns.saturating_sub(record.created_at_ns) > ttl_ns
}

fn apply_retention(records: &[JournalRecord], now_ns: u64, ttl_ns: u64) -> Vec<&str> {
    records
        .iter()
        .filter(|r| retention_eligible(r, now_ns, ttl_ns))
        .map(|r| r.run_id.as_str())
        .collect()
}

const ONE_DAY_NS: u64 = 86_400_000_000_000;
const NOW_NS: u64 = 100 * ONE_DAY_NS;
const TTL_NS: u64 = 30 * ONE_DAY_NS;

fn record(id: &str, days_ago: u64) -> JournalRecord {
    JournalRecord {
        run_id: id.to_string(),
        created_at_ns: NOW_NS - days_ago * ONE_DAY_NS,
    }
}

#[test]
fn test_old_record_eligible_for_deletion() {
    let r = record("old", 31);
    assert!(retention_eligible(&r, NOW_NS, TTL_NS));
}

#[test]
fn test_recent_record_not_eligible() {
    let r = record("new", 5);
    assert!(!retention_eligible(&r, NOW_NS, TTL_NS));
}

#[test]
fn test_exactly_ttl_not_eligible() {
    let r = record("boundary", 30);
    assert!(!retention_eligible(&r, NOW_NS, TTL_NS));
}

#[test]
fn test_apply_retention_returns_eligible_ids() {
    let records = vec![record("a", 60), record("b", 10), record("c", 45)];
    let eligible = apply_retention(&records, NOW_NS, TTL_NS);
    assert!(eligible.contains(&"a"));
    assert!(eligible.contains(&"c"));
    assert!(!eligible.contains(&"b"));
}

#[test]
fn test_no_records_eligible_when_all_recent() {
    let records = vec![record("x", 1), record("y", 2)];
    let eligible = apply_retention(&records, NOW_NS, TTL_NS);
    assert!(eligible.is_empty());
}

#[test]
fn test_all_records_eligible_when_all_old() {
    let records = vec![record("p", 90), record("q", 100)];
    let eligible = apply_retention(&records, NOW_NS, TTL_NS);
    assert_eq!(eligible.len(), 2);
}
