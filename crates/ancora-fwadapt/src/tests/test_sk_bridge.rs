use crate::semantic_kernel::{
    export_to_sk_descriptor, import_sk_plugin, AncoraSkToolSpec, SKBridgeError, SKFunctionDef,
    SKFunctionParam, SKPluginDef,
};
use std::collections::HashMap;

#[test]
fn semantic_kernel_bridge_works() {
    let plugin = SKPluginDef {
        plugin_name: "MathPlugin".into(),
        functions: vec![
            SKFunctionDef {
                name: "add".into(),
                description: "Add two numbers".into(),
                params: vec![
                    SKFunctionParam {
                        name: "a".into(),
                        description: "First operand".into(),
                        default_value: Some("0".into()),
                    },
                    SKFunctionParam {
                        name: "b".into(),
                        description: "Second operand".into(),
                        default_value: Some("0".into()),
                    },
                ],
            },
            SKFunctionDef {
                name: "multiply".into(),
                description: "Multiply two numbers".into(),
                params: vec![],
            },
        ],
    };
    let specs = import_sk_plugin(plugin).unwrap();
    assert_eq!(specs.len(), 2);
    let names: Vec<&str> = specs.iter().map(|s| s.qualified_name.as_str()).collect();
    assert!(names.contains(&"MathPlugin.add"));
    assert!(names.contains(&"MathPlugin.multiply"));
    let add_spec = specs
        .iter()
        .find(|s| s.qualified_name == "MathPlugin.add")
        .unwrap();
    assert_eq!(add_spec.param_defaults.get("a").unwrap(), "0");
}

#[test]
fn sk_empty_plugin_returns_error() {
    let plugin = SKPluginDef {
        plugin_name: "Empty".into(),
        functions: vec![],
    };
    assert!(matches!(
        import_sk_plugin(plugin),
        Err(SKBridgeError::EmptyPlugin)
    ));
}

#[test]
fn sk_duplicate_function_returns_error() {
    let plugin = SKPluginDef {
        plugin_name: "Dup".into(),
        functions: vec![
            SKFunctionDef {
                name: "fn1".into(),
                description: "first".into(),
                params: vec![],
            },
            SKFunctionDef {
                name: "fn1".into(),
                description: "duplicate".into(),
                params: vec![],
            },
        ],
    };
    assert!(matches!(
        import_sk_plugin(plugin),
        Err(SKBridgeError::DuplicateFunction(_))
    ));
}

#[test]
fn sk_descriptor_export_format() {
    let spec = AncoraSkToolSpec {
        qualified_name: "P.func".into(),
        description: "does something".into(),
        param_defaults: {
            let mut m = HashMap::new();
            m.insert("key".into(), "val".into());
            m
        },
    };
    let desc = export_to_sk_descriptor(&spec);
    assert!(desc.contains("P.func"));
    assert!(desc.contains("does something"));
    assert!(desc.contains("key=val"));
}
