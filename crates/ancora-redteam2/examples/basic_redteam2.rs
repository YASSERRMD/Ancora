use ancora_redteam2::attack::{AttackLog, AttackOutcome, AttackStep, AttackVector};
use ancora_redteam2::audit::{RedTeamAction, RedTeamAuditEntry, RedTeamAuditLog};
use ancora_redteam2::detection::{DetectionEvent, DetectionLog, DetectionSource};
use ancora_redteam2::presets::{lateral_movement_scenario, standard_objectives};
use ancora_redteam2::report::RedTeamReport;
use ancora_redteam2::stats::RedTeamStats;
use ancora_redteam2::store::ScenarioStore;

fn main() {
    let mut store = ScenarioStore::new();
    let mut audit = RedTeamAuditLog::new();

    let mut sc = lateral_movement_scenario("sc-1", "tenant-acme", 1);
    sc.start();
    store.insert(sc);
    audit.record(RedTeamAuditEntry::new(
        1,
        "tenant-acme",
        "sc-1",
        RedTeamAction::ScenarioStarted,
        "redteam-op",
        "scenario started",
    ));

    let mut attacks = AttackLog::new();
    attacks.record(AttackStep::new(
        "a1",
        "sc-1",
        "SMB Relay",
        AttackVector::Network,
        AttackOutcome::Success,
        "T1557",
        "Captured NTLM hash",
        2,
    ));
    attacks.record(AttackStep::new(
        "a2",
        "sc-1",
        "Pass the Hash",
        AttackVector::Network,
        AttackOutcome::Detected,
        "T1550",
        "EDR alert raised",
        3,
    ));
    audit.record(RedTeamAuditEntry::new(
        2,
        "tenant-acme",
        "sc-1",
        RedTeamAction::AttackStepExecuted,
        "redteam-op",
        "step a1 executed",
    ));

    let mut detections = DetectionLog::new();
    detections.record(DetectionEvent::new(
        "d1",
        "sc-1",
        DetectionSource::Edr,
        "Credential access detected",
        3,
        true,
    ));

    let mut objectives = standard_objectives("sc-1");
    objectives.get_mut("obj-1").unwrap().achieve(2);
    audit.record(RedTeamAuditEntry::new(
        4,
        "tenant-acme",
        "sc-1",
        RedTeamAction::ObjectiveAchieved,
        "redteam-op",
        "obj-1 achieved",
    ));

    store.get_mut("sc-1").unwrap().complete(10);
    audit.record(RedTeamAuditEntry::new(
        10,
        "tenant-acme",
        "sc-1",
        RedTeamAction::ScenarioCompleted,
        "redteam-op",
        "scenario complete",
    ));

    let sc_refs: Vec<_> = store.for_tenant("tenant-acme");
    let stats = RedTeamStats::compute(&sc_refs, &attacks, &detections);
    let report = RedTeamReport::generate(&store, &attacks, &detections, &objectives, &audit, 10);

    println!("=== Red Team Exercise Report ===");
    println!("Total scenarios:   {}", report.total_scenarios);
    println!("Active scenarios:  {}", report.active_scenarios);
    println!("Attack steps:      {}", report.total_attack_steps);
    println!("Successful steps:  {}", report.successful_steps);
    println!("Detections:        {}", report.total_detections);
    println!("True positives:    {}", report.true_positives);
    println!(
        "Objectives:        {}/{}",
        report.achieved_objectives, report.total_objectives
    );
    println!("Audit entries:     {}", report.total_audit_entries);
    println!("Success rate:      {:.0}%", stats.success_rate * 100.0);
    println!("Evasion rate:      {:.0}%", stats.evasion_rate() * 100.0);
    println!("Detection rate:    {:.0}%", stats.detection_rate * 100.0);
}
