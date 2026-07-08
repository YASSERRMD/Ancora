use ancora_core::routing::ModelRouter;

#[test]
fn default_model_used_when_no_bindings_exist() {
    let router = ModelRouter::new("gpt-4o");
    assert_eq!(router.resolve("any-node", None), "gpt-4o");
}

#[test]
fn explicit_binding_overrides_default_and_node_model() {
    let mut router = ModelRouter::new("big");
    router.bind("node-a", "small");
    assert_eq!(
        router.resolve("node-a", Some("node-specific")),
        "small",
        "explicit binding must take priority over node model"
    );
}

#[test]
fn node_model_id_used_when_no_binding_exists() {
    let router = ModelRouter::new("default");
    assert_eq!(router.resolve("node-b", Some("node-model")), "node-model");
}

#[test]
fn empty_node_model_falls_back_to_default() {
    let router = ModelRouter::new("fallback");
    assert_eq!(router.resolve("node-c", Some("")), "fallback");
}

#[test]
fn bind_small_assigns_cheap_model_to_multiple_nodes() {
    let mut router = ModelRouter::new("large");
    router.bind_small(&["cheap-1", "cheap-2", "cheap-3"], "cheap");
    for node in ["cheap-1", "cheap-2", "cheap-3"] {
        assert_eq!(
            router.resolve(node, None),
            "cheap",
            "bind_small must set cheap model for {node}"
        );
    }
}

#[test]
fn bind_small_does_not_affect_other_nodes() {
    let mut router = ModelRouter::new("large");
    router.bind_small(&["simple"], "small");
    assert_eq!(router.resolve("complex", None), "large");
}

#[test]
fn later_bind_overwrites_earlier_bind_for_same_node() {
    let mut router = ModelRouter::new("default");
    router.bind("node", "model-v1");
    router.bind("node", "model-v2");
    assert_eq!(
        router.resolve("node", None),
        "model-v2",
        "later binding must win"
    );
}

#[test]
fn sequential_resolution_returns_correct_model_per_node() {
    let mut router = ModelRouter::new("default");
    router.bind("researcher", "llama3");
    router.bind("writer", "mistral");

    assert_eq!(router.resolve("researcher", None), "llama3");
    assert_eq!(router.resolve("writer", None), "mistral");
    assert_eq!(router.resolve("other", None), "default");
}

#[test]
fn conditional_node_gets_correct_model_from_binding() {
    let mut router = ModelRouter::new("base");
    router.bind("branch-true", "model-a");
    router.bind("branch-false", "model-b");

    assert_eq!(router.resolve("branch-true", None), "model-a");
    assert_eq!(router.resolve("branch-false", None), "model-b");
}

#[test]
fn resolver_stable_across_repeated_calls() {
    let mut router = ModelRouter::new("default");
    router.bind("stable-node", "stable-model");

    for _ in 0..20 {
        assert_eq!(router.resolve("stable-node", None), "stable-model");
    }
}

#[test]
fn large_graph_routing_resolves_all_nodes() {
    let n = 50;
    let mut router = ModelRouter::new("default");
    for i in 0..n {
        router.bind(&format!("node-{}", i), &format!("model-{}", i % 3));
    }
    for i in 0..n {
        let resolved = router.resolve(&format!("node-{}", i), None);
        assert_eq!(resolved, format!("model-{}", i % 3));
    }
}
