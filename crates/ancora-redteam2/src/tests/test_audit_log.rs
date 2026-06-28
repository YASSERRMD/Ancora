use crate::audit::{RedTeamAction, RedTeamAuditEntry, RedTeamAuditLog};

fn entry(tick: u64, tenant_id: &str, scenario_id: &str, action: RedTeamAction) -> RedTeamAuditEntry {
    RedTeamAuditEntry::new(tick, tenant_id, scenario_id, action, "operator", "detail")
}

#[test]
fn empty_log() {
    let log = RedTeamAuditLog::new();
    assert_eq!(log.count(), 0);
}

#[test]
fn record_and_count() {
    let mut log = RedTeamAuditLog::new();
    log.record(entry(1, "t1", "sc1", RedTeamAction::ScenarioCreated));
    log.record(entry(2, "t1", "sc1", RedTeamAction::ScenarioStarted));
    assert_eq!(log.count(), 2);
}

#[test]
fn for_tenant() {
    let mut log = RedTeamAuditLog::new();
    log.record(entry(1, "t1", "sc1", RedTeamAction::ScenarioCreated));
    log.record(entry(2, "t2", "sc2", RedTeamAction::ScenarioCreated));
    assert_eq!(log.for_tenant("t1").len(), 1);
    assert_eq!(log.for_tenant("t2").len(), 1);
    assert_eq!(log.for_tenant("none").len(), 0);
}

#[test]
fn for_scenario() {
    let mut log = RedTeamAuditLog::new();
    log.record(entry(1, "t1", "sc1", RedTeamAction::ScenarioCreated));
    log.record(entry(2, "t1", "sc1", RedTeamAction::ScenarioStarted));
    log.record(entry(3, "t1", "sc2", RedTeamAction::ScenarioCreated));
    assert_eq!(log.for_scenario("sc1").len(), 2);
}

#[test]
fn by_action() {
    let mut log = RedTeamAuditLog::new();
    log.record(entry(1, "t1", "sc1", RedTeamAction::AttackStepExecuted));
    log.record(entry(2, "t1", "sc1", RedTeamAction::AttackStepExecuted));
    log.record(entry(3, "t1", "sc1", RedTeamAction::ObjectiveAchieved));
    assert_eq!(log.by_action(&RedTeamAction::AttackStepExecuted).len(), 2);
    assert_eq!(log.by_action(&RedTeamAction::ObjectiveAchieved).len(), 1);
}

#[test]
fn all_iterator() {
    let mut log = RedTeamAuditLog::new();
    log.record(entry(1, "t1", "sc1", RedTeamAction::ScenarioCreated));
    assert_eq!(log.all().count(), 1);
}
