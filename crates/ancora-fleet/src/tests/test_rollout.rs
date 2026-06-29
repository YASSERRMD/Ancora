use crate::rollout::*;
use crate::registration::DeviceId;

#[test]
fn test_staged_rollout_proceeds() {
    let mut engine = RolloutEngine::new();
    let mut plan = RolloutPlan::new("rollout-1", "firmware-v3");

    let mut phase1 = RolloutPhase::new("canary", 10);
    phase1.add_device(DeviceId::new("dev-0"));
    phase1.add_device(DeviceId::new("dev-1"));

    let mut phase2 = RolloutPhase::new("prod", 90);
    for i in 2..10 {
        phase2.add_device(DeviceId::new(format!("dev-{}", i)));
    }

    plan.add_phase(phase1);
    plan.add_phase(phase2);

    assert_eq!(plan.total_devices(), 10);

    let total = engine.execute_all(&plan);
    assert_eq!(total, 10);

    let completed = engine.completed_devices("rollout-1");
    assert_eq!(completed.len(), 10);
}

#[test]
fn test_rollout_rollback() {
    let mut engine = RolloutEngine::new();
    let mut plan = RolloutPlan::new("rollout-2", "model-v2");

    let mut phase = RolloutPhase::new("all", 100);
    let ids: Vec<DeviceId> = (0..3).map(|i| DeviceId::new(format!("dev-{}", i))).collect();
    for id in &ids {
        phase.add_device(id.clone());
    }
    plan.add_phase(phase);
    engine.execute_all(&plan);

    engine.rollback("rollout-2", &ids[0]);
    let status = engine.device_status("rollout-2", &ids[0]);
    assert_eq!(status, Some(&RolloutStatus::RolledBack));
}

#[test]
fn test_rollout_single_phase() {
    let mut engine = RolloutEngine::new();
    let mut plan = RolloutPlan::new("r3", "cfg-v1");
    let mut phase = RolloutPhase::new("wave1", 50);
    phase.add_device(DeviceId::new("x1"));
    phase.add_device(DeviceId::new("x2"));
    plan.add_phase(phase);

    let count = engine.execute_phase(&plan, 0);
    assert_eq!(count, 2);
    assert_eq!(engine.completed_devices("r3").len(), 2);
}
