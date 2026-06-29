//! Model recommendation tests.

use crate::model::SmallModel;
use crate::recommend::{DeviceProfile, DeviceRecommender, ModelCandidate};

#[test]
fn test_device_profile_battery_powered() {
    let mobile = DeviceProfile::mobile();
    assert!(mobile.is_battery_powered());
    let server = DeviceProfile::edge_server();
    assert!(!server.is_battery_powered());
}

#[test]
fn test_recommendation_no_candidates() {
    let rec = DeviceRecommender::new();
    let result = rec.recommend(&DeviceProfile::mobile());
    assert!(result.recommended_model_name.is_none());
}

#[test]
fn test_recommendation_candidate_too_large() {
    let mut rec = DeviceRecommender::new();
    // 70B at fp32 -- will not fit in mobile RAM.
    let model = SmallModel::new("llama-70b", 70_000, 32);
    let mem = ModelCandidate::estimate_memory_mib(&model);
    let lat = ModelCandidate::estimate_latency_ms(&model, 20.0);
    rec.add_candidate(ModelCandidate::new(model, mem, lat));
    let result = rec.recommend(&DeviceProfile::mobile());
    // Should have no recommendation since model is too large.
    assert!(result.recommended_model_name.is_none());
}

#[test]
fn test_recommendation_device_name_preserved() {
    let rec = DeviceRecommender::new();
    let device = DeviceProfile::laptop();
    let result = rec.recommend(&device);
    assert_eq!(result.device_name, "laptop");
}
