// Policy: model allowlist -- only approved model IDs may be requested.

struct ModelPolicy {
    approved: Vec<&'static str>,
}

impl ModelPolicy {
    fn new(approved: Vec<&'static str>) -> Self {
        Self { approved }
    }
    fn is_approved(&self, model_id: &str) -> bool {
        self.approved.contains(&model_id)
    }
    fn check(&self, model_id: &str) -> Result<(), String> {
        if self.is_approved(model_id) {
            Ok(())
        } else {
            Err(format!("model '{}' not in approved list", model_id))
        }
    }
}

const APPROVED: &[&str] = &["claude-3-5-haiku", "claude-3-5-sonnet", "qwen3-local"];

#[test]
fn test_approved_model_passes() {
    let p = ModelPolicy::new(APPROVED.to_vec());
    assert!(p.check("claude-3-5-haiku").is_ok());
}

#[test]
fn test_unapproved_model_rejected() {
    let p = ModelPolicy::new(APPROVED.to_vec());
    let r = p.check("gpt-4o");
    assert!(r.is_err());
    assert!(r.unwrap_err().contains("gpt-4o"));
}

#[test]
fn test_all_approved_models_pass() {
    let p = ModelPolicy::new(APPROVED.to_vec());
    for m in APPROVED {
        assert!(p.check(m).is_ok());
    }
}

#[test]
fn test_empty_approved_list_rejects_all() {
    let p = ModelPolicy::new(vec![]);
    assert!(p.check("any-model").is_err());
}

#[test]
fn test_partial_match_not_approved() {
    let p = ModelPolicy::new(vec!["claude-3-5-haiku"]);
    assert!(p.check("claude").is_err());
    assert!(p.check("claude-3-5-haiku-extra").is_err());
}

#[test]
fn test_local_model_approved() {
    let p = ModelPolicy::new(APPROVED.to_vec());
    assert!(p.check("qwen3-local").is_ok());
}
