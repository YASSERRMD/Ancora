use crate::identity::IdentityKind;

#[test]
fn kind_display() {
    assert_eq!(format!("{}", IdentityKind::Human), "HUMAN");
    assert_eq!(format!("{}", IdentityKind::Service), "SERVICE");
    assert_eq!(format!("{}", IdentityKind::Device), "DEVICE");
    assert_eq!(format!("{}", IdentityKind::Workload), "WORKLOAD");
}
