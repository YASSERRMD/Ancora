use crate::indicator::{Indicator, IndicatorKind, ThreatLevel};
use crate::query::IndicatorQuery;

#[test]
fn query_by_kind() {
    let i1 = Indicator::new(
        "i1",
        "t1",
        IndicatorKind::IpAddress,
        "x",
        ThreatLevel::Low,
        "f",
        1,
    );
    let i2 = Indicator::new(
        "i2",
        "t1",
        IndicatorKind::Domain,
        "y",
        ThreatLevel::Low,
        "f",
        1,
    );
    let all = [i1, i2];
    let r = IndicatorQuery::new()
        .kind(IndicatorKind::IpAddress)
        .run(all.iter());
    assert_eq!(r.len(), 1);
}

#[test]
fn query_by_source() {
    let i1 = Indicator::new(
        "i1",
        "t1",
        IndicatorKind::Domain,
        "x",
        ThreatLevel::Low,
        "feed-a",
        1,
    );
    let i2 = Indicator::new(
        "i2",
        "t1",
        IndicatorKind::Domain,
        "y",
        ThreatLevel::Low,
        "feed-b",
        1,
    );
    let all = [i1, i2];
    let r = IndicatorQuery::new().source("feed-a").run(all.iter());
    assert_eq!(r.len(), 1);
}

#[test]
fn query_all() {
    let i1 = Indicator::new(
        "i1",
        "t1",
        IndicatorKind::Url,
        "x",
        ThreatLevel::Low,
        "f",
        1,
    );
    let i2 = Indicator::new(
        "i2",
        "t1",
        IndicatorKind::Email,
        "y",
        ThreatLevel::High,
        "f",
        1,
    );
    let all = [i1, i2];
    let r = IndicatorQuery::new().run(all.iter());
    assert_eq!(r.len(), 2);
}
