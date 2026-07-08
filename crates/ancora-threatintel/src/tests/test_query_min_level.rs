use crate::indicator::{Indicator, IndicatorKind, ThreatLevel};
use crate::query::IndicatorQuery;

#[test]
fn query_min_level_filters_lower() {
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
        IndicatorKind::IpAddress,
        "y",
        ThreatLevel::High,
        "f",
        1,
    );
    let i3 = Indicator::new(
        "i3",
        "t1",
        IndicatorKind::IpAddress,
        "z",
        ThreatLevel::Critical,
        "f",
        1,
    );
    let all = [i1, i2, i3];
    let r = IndicatorQuery::new()
        .min_level(ThreatLevel::High)
        .run(all.iter());
    assert_eq!(r.len(), 2);
}
