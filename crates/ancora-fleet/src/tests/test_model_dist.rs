use crate::model_dist::*;
use crate::registration::DeviceId;

#[test]
fn test_model_distributed_and_verified() {
    let mut svc = ModelDistributionService::new();
    let artifact = ModelArtifact::new("llm-v2", "2.0.0", 1024 * 1024, "abc123def456");

    let id = DeviceId::new("dev-001");
    let record = svc.distribute(&id, &artifact);

    assert_eq!(record.status, DistributionStatus::Verified);
    assert!(svc.is_verified(&id, "llm-v2"));
}

#[test]
fn test_model_dist_missing_checksum_fails() {
    let mut svc = ModelDistributionService::new();
    let artifact = ModelArtifact::new("llm-v2", "2.0.0", 512, "");

    let id = DeviceId::new("dev-002");
    let record = svc.distribute(&id, &artifact);

    assert!(matches!(record.status, DistributionStatus::Failed(_)));
    assert!(!svc.is_verified(&id, "llm-v2"));
}

#[test]
fn test_model_dist_to_fleet() {
    let mut svc = ModelDistributionService::new();
    let artifact = ModelArtifact::new("tiny-llm", "1.0", 256, "valid-checksum");
    let ids: Vec<DeviceId> = (0..6)
        .map(|i| DeviceId::new(format!("dev-{}", i)))
        .collect();

    let records = svc.distribute_to_fleet(&ids, &artifact);
    assert_eq!(records.len(), 6);
    assert_eq!(svc.verified_devices("tiny-llm").len(), 6);
}
