// Chaos: network fault injection -- timeout, connection refused, partial read.

#[derive(Debug, PartialEq)]
enum FaultKind {
    Timeout,
    ConnectionRefused,
    PartialRead,
    None,
}

struct FaultInjector {
    faults: Vec<FaultKind>,
    cursor: usize,
}

impl FaultInjector {
    fn new(faults: Vec<FaultKind>) -> Self {
        Self { faults, cursor: 0 }
    }
    fn next_fault(&mut self) -> &FaultKind {
        let idx = self.cursor % self.faults.len();
        self.cursor += 1;
        &self.faults[idx]
    }
}

fn call_with_fault(injector: &mut FaultInjector) -> Result<String, String> {
    match injector.next_fault() {
        FaultKind::None => Ok("ok".to_string()),
        FaultKind::Timeout => Err("timeout".to_string()),
        FaultKind::ConnectionRefused => Err("connection_refused".to_string()),
        FaultKind::PartialRead => Err("partial_read".to_string()),
    }
}

#[test]
fn test_timeout_fault_returns_error() {
    let mut inj = FaultInjector::new(vec![FaultKind::Timeout]);
    let r = call_with_fault(&mut inj);
    assert!(r.is_err());
    assert_eq!(r.unwrap_err(), "timeout");
}

#[test]
fn test_connection_refused_returns_error() {
    let mut inj = FaultInjector::new(vec![FaultKind::ConnectionRefused]);
    let r = call_with_fault(&mut inj);
    assert_eq!(r.unwrap_err(), "connection_refused");
}

#[test]
fn test_partial_read_returns_error() {
    let mut inj = FaultInjector::new(vec![FaultKind::PartialRead]);
    let r = call_with_fault(&mut inj);
    assert_eq!(r.unwrap_err(), "partial_read");
}

#[test]
fn test_no_fault_succeeds() {
    let mut inj = FaultInjector::new(vec![FaultKind::None]);
    let r = call_with_fault(&mut inj);
    assert!(r.is_ok());
}

#[test]
fn test_fault_sequence_cycles() {
    let mut inj = FaultInjector::new(vec![FaultKind::Timeout, FaultKind::None]);
    let r0 = call_with_fault(&mut inj);
    let r1 = call_with_fault(&mut inj);
    let r2 = call_with_fault(&mut inj);
    assert!(r0.is_err());
    assert!(r1.is_ok());
    assert!(r2.is_err());
}

#[test]
fn test_recovery_after_fault() {
    let mut inj = FaultInjector::new(vec![
        FaultKind::Timeout,
        FaultKind::Timeout,
        FaultKind::None,
    ]);
    let _ = call_with_fault(&mut inj);
    let _ = call_with_fault(&mut inj);
    let r = call_with_fault(&mut inj);
    assert!(r.is_ok());
}
