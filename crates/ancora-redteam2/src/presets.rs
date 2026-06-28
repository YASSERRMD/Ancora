use crate::attack::{AttackLog, AttackOutcome, AttackStep, AttackVector};
use crate::detection::{DetectionEvent, DetectionLog, DetectionSource};
use crate::objective::{ObjectiveTracker, RedTeamObjective};
use crate::scenario::{RedTeamScenario, ScenarioKind};

pub fn priv_esc_scenario(id: impl Into<String>, tenant_id: impl Into<String>, tick: u64) -> RedTeamScenario {
    RedTeamScenario::new(id, tenant_id, "Privilege Escalation via SUID Binary", ScenarioKind::PrivilegeEscalation, tick)
        .with_mitre("TA0004")
}

pub fn lateral_movement_scenario(id: impl Into<String>, tenant_id: impl Into<String>, tick: u64) -> RedTeamScenario {
    RedTeamScenario::new(id, tenant_id, "Lateral Movement via Pass-the-Hash", ScenarioKind::LateralMovement, tick)
        .with_mitre("TA0008")
}

pub fn exfil_scenario(id: impl Into<String>, tenant_id: impl Into<String>, tick: u64) -> RedTeamScenario {
    RedTeamScenario::new(id, tenant_id, "Data Exfiltration via DNS Tunneling", ScenarioKind::DataExfiltration, tick)
        .with_mitre("TA0010")
}

pub fn standard_objectives(scenario_id: impl Into<String> + Clone) -> ObjectiveTracker {
    let mut tracker = ObjectiveTracker::new();
    tracker.add(RedTeamObjective::new("obj-1", scenario_id.clone(), "Gain initial foothold"));
    tracker.add(RedTeamObjective::new("obj-2", scenario_id.clone(), "Escalate privileges"));
    tracker.add(RedTeamObjective::new("obj-3", scenario_id.clone(), "Access target data"));
    tracker.add(RedTeamObjective::new("obj-4", scenario_id, "Exfiltrate without detection"));
    tracker
}

pub fn network_attack_steps(scenario_id: impl Into<String> + Clone) -> AttackLog {
    let mut log = AttackLog::new();
    log.record(AttackStep::new("step-1", scenario_id.clone(), "Port Scan", AttackVector::Network, AttackOutcome::Success, "T1046", "Discovered open ports", 1));
    log.record(AttackStep::new("step-2", scenario_id.clone(), "Exploit Service", AttackVector::Network, AttackOutcome::Success, "T1190", "Exploited vulnerable service", 2));
    log.record(AttackStep::new("step-3", scenario_id.clone(), "Credential Dump", AttackVector::Local, AttackOutcome::Detected, "T1003", "Credential dump triggered EDR", 3));
    log.record(AttackStep::new("step-4", scenario_id, "Data Staging", AttackVector::Local, AttackOutcome::Success, "T1074", "Staged sensitive data", 4));
    log
}

pub fn siem_detections(scenario_id: impl Into<String> + Clone) -> DetectionLog {
    let mut log = DetectionLog::new();
    log.record(DetectionEvent::new("det-1", scenario_id.clone(), DetectionSource::Edr, "Credential dumping detected", 3, true));
    log.record(DetectionEvent::new("det-2", scenario_id, DetectionSource::Siem, "Unusual outbound traffic", 5, false));
    log
}
