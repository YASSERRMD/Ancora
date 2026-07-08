use crate::mcp_native::{McpParamDef, McpParamType, McpRegistryError, McpToolDef, McpToolRegistry};

#[test]
fn mcp_tools_run_register_and_validate() {
    let mut reg = McpToolRegistry::new();
    reg.register(McpToolDef {
        name: "write_file".into(),
        description: "Write contents to a file".into(),
        params: vec![
            McpParamDef {
                name: "path".into(),
                param_type: McpParamType::String,
                required: true,
                description: "Target path".into(),
            },
            McpParamDef {
                name: "content".into(),
                param_type: McpParamType::String,
                required: true,
                description: "File content".into(),
            },
        ],
    })
    .unwrap();

    // Validate a correct call.
    reg.validate_call("write_file", &[("path", "/tmp/x"), ("content", "hello")])
        .unwrap();

    // Validate an incomplete call.
    let err = reg
        .validate_call("write_file", &[("path", "/tmp/x")])
        .unwrap_err();
    assert!(matches!(err, McpRegistryError::ToolNotFound(_)));
}

#[test]
fn mcp_tool_not_found_returns_error() {
    let reg = McpToolRegistry::new();
    assert!(matches!(
        reg.get("unknown"),
        Err(McpRegistryError::ToolNotFound(_))
    ));
}

#[test]
fn mcp_duplicate_tool_rejected() {
    let mut reg = McpToolRegistry::new();
    reg.register(McpToolDef {
        name: "ping".into(),
        description: "Ping".into(),
        params: vec![],
    })
    .unwrap();
    assert!(matches!(
        reg.register(McpToolDef {
            name: "ping".into(),
            description: "duplicate ping".into(),
            params: vec![],
        }),
        Err(McpRegistryError::DuplicateTool(_))
    ));
}

#[test]
fn mcp_tool_names_lists_all() {
    let mut reg = McpToolRegistry::new();
    for name in ["a", "b", "c"] {
        reg.register(McpToolDef {
            name: name.into(),
            description: "desc".into(),
            params: vec![],
        })
        .unwrap();
    }
    let names = reg.tool_names();
    assert_eq!(names.len(), 3);
    assert!(names.contains(&"b"));
}

#[test]
fn mcp_param_type_display() {
    assert_eq!(format!("{}", McpParamType::String), "string");
    assert_eq!(format!("{}", McpParamType::Number), "number");
    assert_eq!(format!("{}", McpParamType::Boolean), "boolean");
}
