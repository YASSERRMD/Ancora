// Policy: consent gate -- human-in-loop consent required before destructive actions.

#[derive(Debug, PartialEq, Clone)]
enum ActionKind {
    ReadOnly,
    Write,
    Destructive,
}

#[derive(Debug, PartialEq, Clone)]
enum ConsentState {
    NotRequested,
    Pending,
    Granted,
    Denied,
}

struct ConsentGate {
    state: ConsentState,
    required_for: Vec<ActionKind>,
}

impl ConsentGate {
    fn new(required_for: Vec<ActionKind>) -> Self {
        Self { state: ConsentState::NotRequested, required_for }
    }

    fn request(&mut self) { self.state = ConsentState::Pending; }
    fn grant(&mut self) { self.state = ConsentState::Granted; }
    fn deny(&mut self) { self.state = ConsentState::Denied; }

    fn check(&self, action: &ActionKind) -> Result<(), String> {
        if self.required_for.contains(action) {
            match self.state {
                ConsentState::Granted => Ok(()),
                ConsentState::Denied => Err("consent denied".to_string()),
                ConsentState::Pending => Err("consent pending".to_string()),
                ConsentState::NotRequested => Err("consent not requested".to_string()),
            }
        } else {
            Ok(())
        }
    }
}

#[test]
fn test_read_allowed_without_consent() {
    let g = ConsentGate::new(vec![ActionKind::Destructive]);
    assert!(g.check(&ActionKind::ReadOnly).is_ok());
}

#[test]
fn test_destructive_blocked_without_consent() {
    let g = ConsentGate::new(vec![ActionKind::Destructive]);
    assert!(g.check(&ActionKind::Destructive).is_err());
}

#[test]
fn test_destructive_allowed_after_consent_granted() {
    let mut g = ConsentGate::new(vec![ActionKind::Destructive]);
    g.request();
    g.grant();
    assert!(g.check(&ActionKind::Destructive).is_ok());
}

#[test]
fn test_destructive_blocked_after_consent_denied() {
    let mut g = ConsentGate::new(vec![ActionKind::Destructive]);
    g.request();
    g.deny();
    let r = g.check(&ActionKind::Destructive);
    assert_eq!(r.unwrap_err(), "consent denied");
}

#[test]
fn test_pending_consent_blocks_action() {
    let mut g = ConsentGate::new(vec![ActionKind::Write]);
    g.request();
    let r = g.check(&ActionKind::Write);
    assert_eq!(r.unwrap_err(), "consent pending");
}

#[test]
fn test_write_requires_consent_when_configured() {
    let g = ConsentGate::new(vec![ActionKind::Write]);
    assert!(g.check(&ActionKind::Write).is_err());
    assert!(g.check(&ActionKind::ReadOnly).is_ok());
}
