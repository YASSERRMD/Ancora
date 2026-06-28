// Security: tool allowlist -- only permitted tools may be invoked.

struct ToolPolicy {
    allowed: Vec<&'static str>,
}

impl ToolPolicy {
    fn new(allowed: Vec<&'static str>) -> Self { Self { allowed } }

    fn is_allowed(&self, tool_name: &str) -> bool {
        self.allowed.contains(&tool_name)
    }

    fn check(&self, tool_name: &str) -> Result<(), String> {
        if self.is_allowed(tool_name) {
            Ok(())
        } else {
            Err(format!("tool '{}' is not in the allowlist", tool_name))
        }
    }
}

#[test]
fn test_allowed_tool_passes() {
    let p = ToolPolicy::new(vec!["web_search", "calculator"]);
    assert!(p.check("web_search").is_ok());
}

#[test]
fn test_unlisted_tool_rejected() {
    let p = ToolPolicy::new(vec!["web_search"]);
    let r = p.check("shell_exec");
    assert!(r.is_err());
    assert!(r.unwrap_err().contains("shell_exec"));
}

#[test]
fn test_empty_allowlist_rejects_all() {
    let p = ToolPolicy::new(vec![]);
    assert!(p.check("any_tool").is_err());
}

#[test]
fn test_exact_match_required() {
    let p = ToolPolicy::new(vec!["web_search"]);
    assert!(p.check("web").is_err());
    assert!(p.check("web_search_extra").is_err());
}

#[test]
fn test_multiple_allowed_tools() {
    let p = ToolPolicy::new(vec!["a", "b", "c"]);
    assert!(p.check("a").is_ok());
    assert!(p.check("b").is_ok());
    assert!(p.check("c").is_ok());
    assert!(p.check("d").is_err());
}

#[test]
fn test_error_message_names_tool() {
    let p = ToolPolicy::new(vec!["safe_tool"]);
    let err = p.check("dangerous_tool").unwrap_err();
    assert!(err.contains("dangerous_tool"));
}
