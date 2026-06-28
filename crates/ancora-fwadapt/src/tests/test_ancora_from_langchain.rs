use crate::ancora_to_langchain::{
    expose_as_langchain_tool, render_langchain_python_snippet, AncoraAgentSpec,
    LangchainToolDescriptor,
};

#[test]
fn ancora_agent_callable_from_langchain() {
    let spec = AncoraAgentSpec {
        id: "qa-agent".into(),
        display_name: "QA Agent".into(),
        capability_summary: "Runs quality assurance checks".into(),
        endpoint: "http://localhost:7070/run".into(),
    };
    let desc = expose_as_langchain_tool(&spec);
    // Dashes replaced with underscores for Python compatibility.
    assert_eq!(desc.name, "qa_agent");
    assert!(desc.description.contains("QA Agent"));
    assert_eq!(desc.return_type, "str");
}

#[test]
fn langchain_descriptor_endpoint_preserved() {
    let spec = AncoraAgentSpec {
        id: "worker".into(),
        display_name: "Worker".into(),
        capability_summary: "Does work".into(),
        endpoint: "http://10.0.0.1:8080/invoke".into(),
    };
    let desc = expose_as_langchain_tool(&spec);
    assert_eq!(desc.endpoint, "http://10.0.0.1:8080/invoke");
}

#[test]
fn python_snippet_contains_all_fields() {
    let desc = LangchainToolDescriptor {
        name: "my_agent".into(),
        description: "Does something useful".into(),
        return_type: "str".into(),
        endpoint: "http://host:1234/run".into(),
    };
    let snippet = render_langchain_python_snippet(&desc);
    assert!(snippet.contains("my_agent"));
    assert!(snippet.contains("http://host:1234/run"));
    assert!(snippet.contains("Tool("));
    assert!(snippet.contains("func=_my_agent_invoke"));
}

#[test]
fn id_with_multiple_dashes_normalised() {
    let spec = AncoraAgentSpec {
        id: "multi-part-id-agent".into(),
        display_name: "Multi Part".into(),
        capability_summary: "Complex".into(),
        endpoint: "http://localhost/run".into(),
    };
    let desc = expose_as_langchain_tool(&spec);
    assert_eq!(desc.name, "multi_part_id_agent");
}
