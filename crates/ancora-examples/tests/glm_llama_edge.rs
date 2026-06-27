use ancora_core::run::Run;
use ancora_proto::ancora::AgentSpec;
use std::collections::HashSet;

const GLM_MODELS: &[&str] = &["glm-4", "glm-4-flash", "glm-4-air", "glm-3-turbo"];

fn make_spec(model: &str) -> AgentSpec {
    AgentSpec {
        name: format!("glm-agent-{model}"),
        model_id: model.to_string(),
        instructions: "Respond briefly.".to_string(),
        output_schema_json: String::new(),
        tools: vec![],
        max_steps: 5,
        model_retry: None,
        model_params_json: String::new(),
    }
}

#[test]
fn glm_model_list_has_four_variants() {
    assert_eq!(4, GLM_MODELS.len());
}

#[test]
fn glm_model_names_are_distinct() {
    let unique: HashSet<&&str> = GLM_MODELS.iter().collect();
    assert_eq!(GLM_MODELS.len(), unique.len());
}

#[test]
fn glm_model_names_start_with_glm() {
    assert!(GLM_MODELS.iter().all(|m| m.starts_with("glm-")));
}

#[test]
fn each_glm_model_produces_distinct_spec_name() {
    let names: Vec<String> = GLM_MODELS.iter().map(|m| make_spec(m).name).collect();
    let unique: HashSet<&String> = names.iter().collect();
    assert_eq!(names.len(), unique.len());
}

#[test]
fn four_glm_run_ids_are_distinct() {
    let runs: Vec<Run> = (0..GLM_MODELS.len()).map(|_| Run::generate()).collect();
    let unique: HashSet<&String> = runs.iter().map(|r| &r.id).collect();
    assert_eq!(runs.len(), unique.len());
}
