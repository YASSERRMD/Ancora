// Policy: data sovereignty -- local-only mode prevents any remote API calls.

#[derive(Debug, PartialEq, Clone)]
enum CallTarget {
    Local,
    Remote(String),
}

struct SovereigntyPolicy {
    local_only: bool,
}

impl SovereigntyPolicy {
    fn new(local_only: bool) -> Self {
        Self { local_only }
    }

    fn check_call(&self, target: &CallTarget) -> Result<(), String> {
        if self.local_only {
            match target {
                CallTarget::Local => Ok(()),
                CallTarget::Remote(host) => Err(format!(
                    "remote call to '{}' blocked in local-only mode",
                    host
                )),
            }
        } else {
            Ok(())
        }
    }
}

#[test]
fn test_local_call_allowed_in_local_only_mode() {
    let p = SovereigntyPolicy::new(true);
    assert!(p.check_call(&CallTarget::Local).is_ok());
}

#[test]
fn test_remote_call_blocked_in_local_only_mode() {
    let p = SovereigntyPolicy::new(true);
    let r = p.check_call(&CallTarget::Remote("api.openai.com".to_string()));
    assert!(r.is_err());
    assert!(r.unwrap_err().contains("api.openai.com"));
}

#[test]
fn test_remote_call_allowed_in_non_local_only_mode() {
    let p = SovereigntyPolicy::new(false);
    assert!(p
        .check_call(&CallTarget::Remote("api.anthropic.com".to_string()))
        .is_ok());
}

#[test]
fn test_local_call_always_allowed() {
    for local_only in [true, false] {
        let p = SovereigntyPolicy::new(local_only);
        assert!(p.check_call(&CallTarget::Local).is_ok());
    }
}

#[test]
fn test_error_message_contains_host() {
    let p = SovereigntyPolicy::new(true);
    let err = p
        .check_call(&CallTarget::Remote("llm.china.example".to_string()))
        .unwrap_err();
    assert!(err.contains("llm.china.example"));
}

#[test]
fn test_error_message_mentions_local_only_mode() {
    let p = SovereigntyPolicy::new(true);
    let err = p
        .check_call(&CallTarget::Remote("anywhere".to_string()))
        .unwrap_err();
    assert!(err.contains("local-only"));
}
