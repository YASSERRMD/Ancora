use crate::AccessKind;
#[test]
fn access_kind_variants_are_distinct() {
    assert_ne!(AccessKind::Read, AccessKind::Write);
    assert_ne!(AccessKind::Rotate, AccessKind::Delete);
    assert_ne!(AccessKind::Read, AccessKind::Delete);
}
