use crate::working_memory::WorkingMemory;

#[test]
fn ring_buffer_evicts_oldest() {
    let mut wm = WorkingMemory::new(3);
    wm.push("a".to_string());
    wm.push("b".to_string());
    wm.push("c".to_string());
    wm.push("d".to_string()); // evicts "a"
    let recent = wm.peek_recent(3);
    assert!(!recent.contains(&"a"));
    assert!(recent.contains(&"d"));
}

#[test]
fn peek_recent_respects_n() {
    let mut wm = WorkingMemory::new(10);
    for i in 0..5 { wm.push(i.to_string()); }
    assert_eq!(wm.peek_recent(2).len(), 2);
}

#[test]
fn clear_empties_buffer() {
    let mut wm = WorkingMemory::new(5);
    wm.push("x".to_string());
    wm.clear();
    assert!(wm.is_empty());
}
