/// Determinism with plugins on replay: same inputs produce same outputs.

use crate::plugin_e2e::{Plugin, PluginState, PluginTemplate};
use crate::recipe_e2e::{Recipe, RecipeRunner, RecipeStep, StepResult};
use crate::builder_e2e::{Node, PluginGraph};

fn run_recipe_once() -> Vec<StepResult> {
    let mut recipe = Recipe::new("det-recipe", "1.0.0");
    recipe.add_step(RecipeStep::new("step-a", "plugin-a", "run", vec!["--seed", "42"]));
    recipe.add_step(RecipeStep::new("step-b", "plugin-b", "process", vec!["--mode", "strict"]));
    let mut runner = RecipeRunner::new();
    runner.install(recipe).unwrap();
    runner.run("det-recipe").unwrap()
}

fn build_graph_order() -> Vec<u64> {
    let mut graph = PluginGraph::new();
    graph.add_node(Node::new(10, "A", "pa")).unwrap();
    graph.add_node(Node::new(20, "B", "pb")).unwrap();
    graph.add_node(Node::new(30, "C", "pc")).unwrap();
    graph.add_edge(10, 20).unwrap();
    graph.add_edge(20, 30).unwrap();
    graph.topological_order().unwrap()
}

#[test]
fn test_recipe_is_deterministic_across_runs() {
    let r1 = run_recipe_once();
    let r2 = run_recipe_once();
    assert_eq!(r1.len(), r2.len());
    for (a, b) in r1.iter().zip(r2.iter()) {
        match (a, b) {
            (StepResult::Ok(ma), StepResult::Ok(mb)) => assert_eq!(ma, mb),
            _ => panic!("result mismatch between runs"),
        }
    }
}

#[test]
fn test_graph_topological_order_deterministic() {
    let order1 = build_graph_order();
    let order2 = build_graph_order();
    assert_eq!(order1, order2);
    assert_eq!(order1, vec![10, 20, 30]);
}

#[test]
fn test_plugin_lifecycle_deterministic() {
    fn lifecycle(id: u64) -> PluginState {
        let template = PluginTemplate::new("det-plugin", "1.0.0", "d", "main.rs");
        let mut plugin = Plugin::from_template(template, id).unwrap();
        plugin.compile().unwrap();
        plugin.install().unwrap();
        plugin.start().unwrap();
        plugin.state.clone()
    }
    let s1 = lifecycle(1);
    let s2 = lifecycle(2);
    assert_eq!(s1, s2);
    assert_eq!(s1, PluginState::Running);
}
