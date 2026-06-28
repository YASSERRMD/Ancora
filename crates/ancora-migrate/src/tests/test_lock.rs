use crate::lock::MigrationLock;

#[test]
fn lock_free_initially() {
    let lock = MigrationLock::new(60);
    assert!(lock.is_free(0));
}

#[test]
fn acquire_succeeds_when_free() {
    let mut lock = MigrationLock::new(60);
    assert!(lock.acquire("node-1", 0));
    assert_eq!(lock.holder(), Some("node-1"));
}

#[test]
fn second_acquire_fails() {
    let mut lock = MigrationLock::new(60);
    lock.acquire("node-1", 0);
    assert!(!lock.acquire("node-2", 1));
}

#[test]
fn release_by_holder_frees_lock() {
    let mut lock = MigrationLock::new(60);
    lock.acquire("node-1", 0);
    assert!(lock.release("node-1"));
    assert!(lock.is_free(1));
}

#[test]
fn ttl_expiry_makes_lock_free() {
    let mut lock = MigrationLock::new(30);
    lock.acquire("node-1", 0);
    assert!(!lock.is_free(29));
    assert!(lock.is_free(30));
}

#[test]
fn non_holder_cannot_release() {
    let mut lock = MigrationLock::new(60);
    lock.acquire("node-1", 0);
    assert!(!lock.release("node-2"));
}
