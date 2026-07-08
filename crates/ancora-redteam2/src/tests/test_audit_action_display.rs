use crate::audit::RedTeamAction;

#[test]
fn created() {
    assert_eq!(
        RedTeamAction::ScenarioCreated.to_string(),
        "SCENARIO_CREATED"
    );
}
#[test]
fn started() {
    assert_eq!(
        RedTeamAction::ScenarioStarted.to_string(),
        "SCENARIO_STARTED"
    );
}
#[test]
fn completed() {
    assert_eq!(
        RedTeamAction::ScenarioCompleted.to_string(),
        "SCENARIO_COMPLETED"
    );
}
#[test]
fn aborted() {
    assert_eq!(
        RedTeamAction::ScenarioAborted.to_string(),
        "SCENARIO_ABORTED"
    );
}
#[test]
fn step_executed() {
    assert_eq!(
        RedTeamAction::AttackStepExecuted.to_string(),
        "ATTACK_STEP_EXECUTED"
    );
}
#[test]
fn objective_achieved() {
    assert_eq!(
        RedTeamAction::ObjectiveAchieved.to_string(),
        "OBJECTIVE_ACHIEVED"
    );
}
#[test]
fn detection_logged() {
    assert_eq!(
        RedTeamAction::DetectionLogged.to_string(),
        "DETECTION_LOGGED"
    );
}
#[test]
fn report_generated() {
    assert_eq!(
        RedTeamAction::ReportGenerated.to_string(),
        "REPORT_GENERATED"
    );
}
