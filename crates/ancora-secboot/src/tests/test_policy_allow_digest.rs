use crate::BootPolicy;
#[test]
fn allow_digest_permits_correct_digest() {
    let p = BootPolicy::new("t1").allow_digest("vmlinuz", "abc123");
    assert!(p.is_digest_allowed("vmlinuz", "abc123"));
    assert!(!p.is_digest_allowed("vmlinuz", "bad"));
    assert!(!p.is_digest_allowed("other", "abc123"));
}
