use crate::indicator::{Indicator, IndicatorKind, ThreatLevel};
use crate::query::IndicatorQuery;

#[test]
fn query_by_tag() {
    let i1 = Indicator::new(
        "i1",
        "t1",
        IndicatorKind::Domain,
        "x",
        ThreatLevel::Low,
        "f",
        1,
    )
    .with_tag("apt29");
    let i2 = Indicator::new(
        "i2",
        "t1",
        IndicatorKind::Domain,
        "y",
        ThreatLevel::Low,
        "f",
        1,
    )
    .with_tag("ransomware");
    let all = vec![i1, i2];
    let r = IndicatorQuery::new().tag("apt29").run(all.iter());
    assert_eq!(r.len(), 1);
    assert_eq!(r[0].id, "i1");
}

#[test]
fn query_active_only() {
    let i1 = Indicator::new(
        "i1",
        "t1",
        IndicatorKind::IpAddress,
        "x",
        ThreatLevel::Low,
        "f",
        1,
    );
    let mut i2 = Indicator::new(
        "i2",
        "t1",
        IndicatorKind::IpAddress,
        "y",
        ThreatLevel::Low,
        "f",
        1,
    );
    i2.deactivate();
    let all = vec![i1, i2];
    let r = IndicatorQuery::new().active_only().run(all.iter());
    assert_eq!(r.len(), 1);
}
