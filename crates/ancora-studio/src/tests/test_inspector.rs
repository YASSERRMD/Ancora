use crate::inspector::{InspectorData, ToolCallDetail};

#[test]
fn test_inspector_shows_data() {
    let data = InspectorData::new("r1", 2)
        .with_prompt("What is 2+2?")
        .with_response("4");
    assert_eq!(data.visible_prompt(), Some("What is 2+2?"));
    assert_eq!(data.visible_response(), Some("4"));
    assert_eq!(data.step_index, 2);
}

#[test]
fn test_inspector_redaction_hides_prompt() {
    let mut data = InspectorData::new("r1", 0).with_prompt("secret prompt");
    data.redacted_fields.push("prompt".into());
    assert_eq!(data.visible_prompt(), None);
    // response should still show
    let data2 = data.with_response("visible");
    assert_eq!(data2.visible_response(), Some("visible"));
}

#[test]
fn test_inspector_tool_calls_shown() {
    let mut data = InspectorData::new("r1", 0);
    data.add_tool_call(ToolCallDetail {
        name: "calculator".into(),
        input_json: r#"{"expr":"2+2"}"#.into(),
        output_json: Some(r#"{"result":4}"#.into()),
        error: None,
    });
    data.add_tool_call(ToolCallDetail {
        name: "failed_tool".into(),
        input_json: "{}".into(),
        output_json: None,
        error: Some("timeout".into()),
    });
    assert_eq!(data.tool_calls.len(), 2);
    assert!(data.tool_calls[1].error.is_some());
}

#[test]
fn test_inspector_metadata() {
    let mut data = InspectorData::new("r1", 0);
    data.set_metadata("model", "gpt-4");
    assert_eq!(data.metadata.get("model").unwrap(), "gpt-4");
}
