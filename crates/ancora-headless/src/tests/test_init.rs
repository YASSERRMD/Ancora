use crate::init::{
    notify_ready, PidFile, RestartPolicy, ServiceLifecycle, ServiceState, ServiceUnit,
};

#[test]
fn test_service_unit_default_name() {
    let unit = ServiceUnit::default();
    assert_eq!(unit.name, "ancora-agent");
}

#[test]
fn test_service_unit_render_contains_exec() {
    let unit = ServiceUnit::default();
    let rendered = unit.render();
    assert!(rendered.contains("ExecStart=/usr/local/bin/ancora-headless"));
    assert!(rendered.contains("Type=notify"));
    assert!(rendered.contains("WantedBy=multi-user.target"));
}

#[test]
fn test_service_unit_auto_restarts_on_failure() {
    let unit = ServiceUnit {
        restart_policy: RestartPolicy::OnFailure,
        ..ServiceUnit::default()
    };
    assert!(unit.auto_restarts());
}

#[test]
fn test_service_unit_no_restart_policy() {
    let unit = ServiceUnit {
        restart_policy: RestartPolicy::No,
        ..ServiceUnit::default()
    };
    assert!(!unit.auto_restarts());
}

#[test]
fn test_pid_file_content() {
    let pf = PidFile::new("/run/ancora/agent.pid", 12345);
    assert_eq!(pf.content(), "12345\n");
}

#[test]
fn test_notify_ready_string() {
    let s = notify_ready(&ServiceState::Ready);
    assert!(s.contains("READY=1"));
}

#[test]
fn test_notify_starting_string() {
    let s = notify_ready(&ServiceState::Starting);
    assert!(s.contains("starting"));
}

#[test]
fn test_lifecycle_transitions() {
    let mut lc = ServiceLifecycle::new();
    assert_eq!(*lc.current(), ServiceState::Idle);
    lc.transition(ServiceState::Starting);
    lc.transition(ServiceState::Ready);
    assert!(lc.is_ready());
    assert_eq!(lc.history().len(), 3); // Idle, Starting, Ready
}

#[test]
fn test_service_state_display() {
    assert_eq!(ServiceState::Ready.to_string(), "ready");
    assert_eq!(
        ServiceState::Failed("oops".to_string()).to_string(),
        "failed: oops"
    );
}
