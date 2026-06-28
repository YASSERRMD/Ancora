use crate::scenario::{RedTeamScenario, ScenarioKind};

#[test]
fn multiple_metadata_entries() {
    let s = RedTeamScenario::new("sc-1", "t1", "N", ScenarioKind::DataExfiltration, 1)
        .with_metadata("k1", "v1")
        .with_metadata("k2", "v2");
    assert_eq!(s.metadata.len(), 2);
    assert_eq!(s.metadata.get("k1").map(|v| v.as_str()), Some("v1"));
    assert_eq!(s.metadata.get("k2").map(|v| v.as_str()), Some("v2"));
}

#[test]
fn metadata_override() {
    let s = RedTeamScenario::new("sc-1", "t1", "N", ScenarioKind::DataExfiltration, 1)
        .with_metadata("k", "old")
        .with_metadata("k", "new");
    assert_eq!(s.metadata.get("k").map(|v| v.as_str()), Some("new"));
}
