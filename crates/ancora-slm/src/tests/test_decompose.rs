use crate::decompose::{
    execute_plan, validate_step_output, DecompositionPlan, OutputFormat, Step,
};

fn make_json_step(id: &str) -> Step {
    Step {
        id: id.to_string(),
        description: "Produce JSON".to_string(),
        output_format: OutputFormat::JsonObject,
        optional: false,
    }
}

fn make_text_step(id: &str) -> Step {
    Step {
        id: id.to_string(),
        description: "Summarise".to_string(),
        output_format: OutputFormat::Text,
        optional: false,
    }
}

#[test]
fn test_validate_step_output_text_ok() {
    assert!(validate_step_output("hello world", &OutputFormat::Text).is_ok());
}

#[test]
fn test_validate_step_output_text_empty_fails() {
    assert!(validate_step_output("   ", &OutputFormat::Text).is_err());
}

#[test]
fn test_validate_step_output_json_object_ok() {
    assert!(validate_step_output(r#"{"k": 1}"#, &OutputFormat::JsonObject).is_ok());
}

#[test]
fn test_validate_step_output_json_array_ok() {
    assert!(validate_step_output("[1,2,3]", &OutputFormat::JsonArray).is_ok());
}

#[test]
fn test_validate_step_output_yes_no() {
    assert!(validate_step_output("yes", &OutputFormat::YesNo).is_ok());
    assert!(validate_step_output("No", &OutputFormat::YesNo).is_ok());
    assert!(validate_step_output("maybe", &OutputFormat::YesNo).is_err());
}

#[test]
fn test_execute_plan_all_steps_pass() {
    let plan = DecompositionPlan::new(
        "Test task",
        vec![make_json_step("step1"), make_json_step("step2")],
    );
    let model_fn = |_: &str| r#"{"ok": true}"#.to_string();
    let results = execute_plan(&plan, model_fn);
    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|r| r.valid), "all steps should pass");
}

#[test]
fn test_execute_plan_stops_on_required_failure() {
    let plan = DecompositionPlan::new(
        "Task",
        vec![
            make_json_step("step1"),
            Step {
                id: "fail-step".into(),
                description: "fail".into(),
                output_format: OutputFormat::JsonObject,
                optional: false,
            },
            make_text_step("step3"),
        ],
    );
    let model_fn = |prompt: &str| {
        if prompt.contains("fail-step") || prompt.contains("fail") {
            "plain text not json".to_string()
        } else {
            r#"{"ok": true}"#.to_string()
        }
    };
    let results = execute_plan(&plan, model_fn);
    // Should stop after the failing required step — step3 should not be executed.
    assert!(results.len() < 3, "should stop before step3");
    assert!(!results.last().unwrap().valid, "last result should be invalid");
}

#[test]
fn test_execute_plan_continues_past_optional_failure() {
    let plan = DecompositionPlan::new(
        "Task",
        vec![
            Step {
                id: "opt".into(),
                description: "optional fail".into(),
                output_format: OutputFormat::JsonObject,
                optional: true,
            },
            make_text_step("final"),
        ],
    );
    // Model always returns plain text — first step fails but is optional.
    let model_fn = |_: &str| "plain text".to_string();
    let results = execute_plan(&plan, model_fn);
    assert_eq!(results.len(), 2, "should execute both steps including optional");
    assert!(!results[0].valid);
    assert!(results[1].valid);
}
