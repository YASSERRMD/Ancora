/// Tests for engine health and readiness checking.
use crate::health::{wait_ready, HealthChecker, HealthState, HealthStatus, MockHealthChecker};
use crate::model::EngineKind;
use std::time::Duration;

#[test]
fn healthy_status_is_ready() {
    let status = HealthStatus::healthy(EngineKind::Ollama, 5);
    assert!(status.is_ready());
    assert_eq!(status.state, HealthState::Healthy);
    assert_eq!(status.latency_ms, Some(5));
}

#[test]
fn unhealthy_status_not_ready() {
    let status = HealthStatus::unhealthy(EngineKind::Vllm, "connection refused");
    assert!(!status.is_ready());
    assert_eq!(status.state, HealthState::Unhealthy);
    assert!(status
        .message
        .as_deref()
        .unwrap()
        .contains("connection refused"));
}

#[test]
fn starting_status_not_ready() {
    let status = HealthStatus::starting(EngineKind::Tgi);
    assert!(!status.is_ready());
    assert_eq!(status.state, HealthState::Starting);
}

#[test]
fn unknown_status_not_ready() {
    let status = HealthStatus::unknown(EngineKind::OnnxRuntime);
    assert!(!status.is_ready());
    assert_eq!(status.state, HealthState::Unknown);
}

#[test]
fn mock_checker_healthy() {
    let checker = MockHealthChecker::healthy(EngineKind::Ollama);
    let status = checker.check();
    assert!(status.is_ready());
    assert_eq!(status.engine, EngineKind::Ollama);
}

#[test]
fn mock_checker_unhealthy() {
    let checker = MockHealthChecker::unhealthy(EngineKind::Vllm);
    let status = checker.check();
    assert!(!status.is_ready());
    assert_eq!(status.engine, EngineKind::Vllm);
}

#[test]
fn wait_ready_returns_immediately_when_healthy() {
    let checker = MockHealthChecker::healthy(EngineKind::LlamaCppServer);
    let status = wait_ready(&checker, Duration::from_secs(5), Duration::ZERO);
    assert!(status.is_ready());
}

#[test]
fn wait_ready_returns_unhealthy_on_timeout() {
    let checker = MockHealthChecker::unhealthy(EngineKind::Sglang);
    let status = wait_ready(&checker, Duration::ZERO, Duration::ZERO);
    assert!(!status.is_ready());
}

#[test]
fn health_state_display() {
    assert_eq!(HealthState::Healthy.to_string(), "healthy");
    assert_eq!(HealthState::Unhealthy.to_string(), "unhealthy");
    assert_eq!(HealthState::Starting.to_string(), "starting");
    assert_eq!(HealthState::Unknown.to_string(), "unknown");
}
