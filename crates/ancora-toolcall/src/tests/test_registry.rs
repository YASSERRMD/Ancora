use crate::registry::ToolRegistry;
use crate::schema::ToolDef;

#[test]
fn register_and_get() {
    let mut r = ToolRegistry::new();
    r.register(ToolDef::new("search", "search"));
    assert!(r.get("search").is_some());
    assert_eq!(r.count(), 1);
}

#[test]
fn validate_call_unknown_tool_errors() {
    let r = ToolRegistry::new();
    assert!(r.validate_call("unknown").is_err());
}

#[test]
fn names_returns_all_tools() {
    let mut r = ToolRegistry::new();
    r.register(ToolDef::new("a", "a"));
    r.register(ToolDef::new("b", "b"));
    let mut names = r.names();
    names.sort();
    assert_eq!(names, vec!["a", "b"]);
}
