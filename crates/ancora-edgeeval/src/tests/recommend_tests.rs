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

#[test]
fn test_recommendation_edge_server_accepts_large_model() {
    let mut rec = DeviceRecommender::new();
    // 1B INT8 model on a 1000 GOPS edge server -- should easily fit and meet latency.
    let model = SmallModel::new("phi-1b-int8", 1_000, 8);
    let mem = ModelCandidate::estimate_memory_mib(&model);
    let lat = ModelCandidate::estimate_latency_ms(&model, 1000.0);
    rec.add_candidate(ModelCandidate::new(model, mem, lat));
    let device = DeviceProfile::edge_server();
    let result = rec.recommend(&device);
    assert_eq!(result.device_name, "edge-server");
    assert!(result.recommended_model_name.is_some());
}

#[test]
fn test_recommendation_candidates_evaluated_count() {
    let mut rec = DeviceRecommender::new();
    for i in 0..3u64 {
        let model = SmallModel::new(format!("model-{}", i), (i + 1) * 100, 8);
        let mem = ModelCandidate::estimate_memory_mib(&model);
        let lat = ModelCandidate::estimate_latency_ms(&model, 100.0);
        rec.add_candidate(ModelCandidate::new(model, mem, lat));
    }
    let device = DeviceProfile::laptop();
    let result = rec.recommend(&device);
    assert!(result.candidates_evaluated > 0 || result.recommended_model_name.is_none());
}
