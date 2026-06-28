use crate::tool_kit::{Tool, ToolKit, ToolSchema};
use std::collections::HashMap;

struct UpperCaseTool;

impl Tool for UpperCaseTool {
    fn name(&self) -> &str {
        "uppercase"
    }

    fn description(&self) -> &str {
        "Converts the input text to upper case"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            input_fields: vec!["text".into()],
            output_fields: vec!["result".into()],
        }
    }

    fn call(&self, args: HashMap<String, String>) -> Result<HashMap<String, String>, String> {
        let text = args.get("text").ok_or("missing field: text")?;
        let mut out = HashMap::new();
        out.insert("result".into(), text.to_uppercase());
        Ok(out)
    }
}

#[test]
fn tool_kit_passes_for_uppercase_tool() {
    let kit = ToolKit::new();
    let results = kit.run(&UpperCaseTool);
    for r in &results {
        assert!(r.passed, "Check failed: {} - {}", r.name, r.message);
    }
    assert_eq!(results.len(), 4);
}

#[test]
fn uppercase_tool_produces_correct_output() {
    let tool = UpperCaseTool;
    let mut args = HashMap::new();
    args.insert("text".into(), "hello world".into());
    let result = tool.call(args).unwrap();
    assert_eq!(result["result"], "HELLO WORLD");
}
