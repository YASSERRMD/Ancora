use crate::adapter_e2e::{FrameworkAdapter, ToolSpec};

#[test]
fn test_framework_adapter_imports_a_tool() {
    let mut adapter = FrameworkAdapter::new("langchain");
    let spec = ToolSpec::new("search", r#"{"query": "string"}"#, r#"{"results": "array"}"#, "search-plugin");
    adapter.import_tool(spec).expect("import must succeed");
    assert_eq!(adapter.tool_count(), 1);
    let found = adapter.get_tool("search").expect("tool must be found");
    assert_eq!(found.source_plugin, "search-plugin");
}

#[test]
fn test_adapter_duplicate_import_fails() {
    let mut adapter = FrameworkAdapter::new("llamaindex");
    adapter.import_tool(ToolSpec::new("t", "{}", "{}", "p")).unwrap();
    let result = adapter.import_tool(ToolSpec::new("t", "{}", "{}", "p2"));
    assert!(result.is_err());
}

#[test]
fn test_adapter_invalid_spec_fails() {
    let mut adapter = FrameworkAdapter::new("haystack");
    let invalid = ToolSpec::new("", "{}", "{}", "p");
    let result = adapter.import_tool(invalid);
    assert!(result.is_err());
}

#[test]
fn test_adapter_remove_tool() {
    let mut adapter = FrameworkAdapter::new("custom");
    adapter.import_tool(ToolSpec::new("t", "{}", "{}", "p")).unwrap();
    assert!(adapter.remove_tool("t"));
    assert_eq!(adapter.tool_count(), 0);
}

#[test]
fn test_adapter_list_sorted() {
    let mut adapter = FrameworkAdapter::new("multi");
    adapter.import_tool(ToolSpec::new("z-tool", "{}", "{}", "p1")).unwrap();
    adapter.import_tool(ToolSpec::new("a-tool", "{}", "{}", "p2")).unwrap();
    let tools = adapter.list_tools();
    assert_eq!(tools[0].name, "a-tool");
    assert_eq!(tools[1].name, "z-tool");
}
