use crate::boundary::{AirGapBoundary, AirGapZone, ZoneClassification};

#[test]
fn boundary_add_and_get() {
    let mut b = AirGapBoundary::new();
    b.add_zone(AirGapZone::new(
        "z1",
        "Zone 1",
        ZoneClassification::Internal,
        "t1",
    ));
    assert!(b.get("z1").is_some());
    assert_eq!(b.count(), 1);
}

#[test]
fn boundary_restricted_zones() {
    let mut b = AirGapBoundary::new();
    b.add_zone(AirGapZone::new(
        "z1",
        "Z1",
        ZoneClassification::Restricted,
        "t1",
    ));
    b.add_zone(AirGapZone::new(
        "z2",
        "Z2",
        ZoneClassification::Public,
        "t1",
    ));
    assert_eq!(b.restricted_zones().len(), 1);
}

#[test]
fn boundary_for_tenant() {
    let mut b = AirGapBoundary::new();
    b.add_zone(AirGapZone::new(
        "z1",
        "Z1",
        ZoneClassification::Internal,
        "t1",
    ));
    b.add_zone(AirGapZone::new(
        "z2",
        "Z2",
        ZoneClassification::Internal,
        "t2",
    ));
    assert_eq!(b.for_tenant("t1").len(), 1);
}
