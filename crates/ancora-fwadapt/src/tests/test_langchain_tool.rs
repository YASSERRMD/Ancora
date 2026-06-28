use crate::langchain_tools::{
    import_langchain_tools, AncoraToolAdapter, LangchainToolDef, ToolAdapterError,
};

#[test]
fn langchain_tool_runs_in_ancora() {
    let def = LangchainToolDef {
        name: "calculator".into(),
        description: "Performs arithmetic".into(),
    };
    let adapter = AncoraToolAdapter::from_langchain(def, |input| {
        // Simulate echo evaluation.
        Ok(format!("result({})", input))
    });
    let output = adapter.run("2+2").unwrap();
    assert_eq!(output, "result(2+2)");
}

#[test]
fn langchain_tool_propagates_error() {
    let def = LangchainToolDef {
        name: "broken-tool".into(),
        description: "Always fails".into(),
    };
    let adapter = AncoraToolAdapter::from_langchain(def, |_| {
        Err(ToolAdapterError { message: "intentional failure".into() })
    });
    let err = adapter.run("anything").unwrap_err();
    assert!(err.message.contains("intentional failure"));
}

#[test]
fn import_multiple_langchain_tools() {
    let defs = vec![
        LangchainToolDef { name: "search".into(), description: "Searches".into() },
        LangchainToolDef { name: "translate".into(), description: "Translates".into() },
        LangchainToolDef { name: "summarise".into(), description: "Summarises".into() },
    ];
    let adapters = import_langchain_tools(defs);
    assert_eq!(adapters.len(), 3);
    let names: Vec<&str> = adapters.iter().map(|a| a.tool_name.as_str()).collect();
    assert!(names.contains(&"search"));
    assert!(names.contains(&"summarise"));
}

#[test]
fn imported_tool_echo_handler() {
    let defs = vec![LangchainToolDef {
        name: "echo".into(),
        description: "echo".into(),
    }];
    let adapters = import_langchain_tools(defs);
    let out = adapters[0].run("hello world").unwrap();
    assert!(out.contains("hello world"));
}
