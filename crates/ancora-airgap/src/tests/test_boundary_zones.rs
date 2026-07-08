use crate::boundary::{AirGapBoundary, AirGapZone, ZoneClassification};

#[test]
fn public_zone_not_restricted() {
    let z = AirGapZone::new("z1", "Z1", ZoneClassification::Public, "t1");
    assert!(!z.is_restricted());
}

#[test]
fn top_secret_is_restricted() {
    let z = AirGapZone::new("z1", "Z1", ZoneClassification::TopSecret, "t1");
    assert!(z.is_restricted());
}

#[test]
fn zone_with_metadata() {
    let z = AirGapZone::new("z1", "Z1", ZoneClassification::Internal, "t1")
        .with_metadata("owner", "ops");
    assert_eq!(z.metadata.get("owner").map(|s| s.as_str()), Some("ops"));
}

#[test]
fn boundary_count() {
    let mut b = AirGapBoundary::new();
    b.add_zone(AirGapZone::new(
        "z1",
        "Z1",
        ZoneClassification::Public,
        "t1",
    ));
    b.add_zone(AirGapZone::new(
        "z2",
        "Z2",
        ZoneClassification::TopSecret,
        "t1",
    ));
    assert_eq!(b.count(), 2);
    assert_eq!(b.restricted_zones().len(), 1);
}
