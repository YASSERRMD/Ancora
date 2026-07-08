use crate::recipe_e2e::{Recipe, RecipeRunner, RecipeStep, StepResult};

fn build_recipe() -> Recipe {
    let mut recipe = Recipe::new("data-pipeline", "1.0.0");
    recipe.add_step(RecipeStep::new(
        "ingest",
        "ingest-plugin",
        "run",
        vec!["--input", "data.csv"],
    ));
    recipe.add_step(RecipeStep::new(
        "transform",
        "transform-plugin",
        "apply",
        vec!["--schema", "v2"],
    ));
    recipe.add_step(RecipeStep::new(
        "output",
        "output-plugin",
        "emit",
        vec!["--dest", "s3://bucket"],
    ));
    recipe
}

#[test]
fn test_recipe_installs_and_runs() {
    let mut runner = RecipeRunner::new();
    let recipe = build_recipe();
    runner.install(recipe).expect("install must succeed");
    assert!(runner.is_installed("data-pipeline"));
    let results = runner.run("data-pipeline").expect("run must succeed");
    assert_eq!(results.len(), 3);
    for result in &results {
        assert!(matches!(result, StepResult::Ok(_)));
    }
}

#[test]
fn test_run_missing_recipe_fails() {
    let runner = RecipeRunner::new();
    let result = runner.run("nonexistent");
    assert!(result.is_err());
}

#[test]
fn test_recipe_duplicate_install_fails() {
    let mut runner = RecipeRunner::new();
    runner.install(build_recipe()).unwrap();
    let result = runner.install(build_recipe());
    assert!(result.is_err());
}

#[test]
fn test_recipe_uninstall() {
    let mut runner = RecipeRunner::new();
    runner.install(build_recipe()).unwrap();
    assert!(runner.uninstall("data-pipeline"));
    assert!(!runner.is_installed("data-pipeline"));
}

#[test]
fn test_recipe_step_results_contain_plugin_info() {
    let mut runner = RecipeRunner::new();
    runner.install(build_recipe()).unwrap();
    let results = runner.run("data-pipeline").unwrap();
    if let StepResult::Ok(msg) = &results[0] {
        assert!(msg.contains("ingest-plugin"));
    } else {
        panic!("expected Ok result");
    }
}
