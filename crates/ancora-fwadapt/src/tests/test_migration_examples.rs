use crate::examples::{
    example_a2a_migration, example_crewai_migration, example_expose_to_langchain,
    example_langchain_migration, example_langgraph_migration, example_mcp_migration,
    example_openai_agents_migration, example_sk_migration,
};

#[test]
fn migration_examples_run_langchain() {
    let r = example_langchain_migration();
    assert_eq!(r.framework, "langchain");
    assert_eq!(r.items_migrated, 2);
}

#[test]
fn migration_examples_run_langgraph() {
    let r = example_langgraph_migration();
    assert_eq!(r.framework, "langgraph");
    assert_eq!(r.items_migrated, 2);
}

#[test]
fn migration_examples_run_crewai() {
    let r = example_crewai_migration();
    assert_eq!(r.framework, "crewai");
    assert_eq!(r.items_migrated, 2);
    assert!(r.notes.contains("ops-crew"));
}

#[test]
fn migration_examples_run_mcp() {
    let r = example_mcp_migration();
    assert_eq!(r.framework, "mcp");
    assert_eq!(r.items_migrated, 2);
}

#[test]
fn migration_examples_run_expose_langchain() {
    let r = example_expose_to_langchain();
    assert_eq!(r.framework, "langchain-expose");
    assert_eq!(r.items_migrated, 1);
    assert!(r.notes.contains("report_gen"));
}

#[test]
fn migration_examples_run_a2a() {
    let r = example_a2a_migration();
    assert_eq!(r.framework, "a2a");
    assert_eq!(r.items_migrated, 1);
    assert!(r.notes.contains("processed"));
}

#[test]
fn migration_examples_run_openai_agents() {
    let r = example_openai_agents_migration();
    assert_eq!(r.framework, "openai-agents");
    assert_eq!(r.items_migrated, 1);
    assert!(r.notes.contains("classified"));
}

#[test]
fn migration_examples_run_sk() {
    let r = example_sk_migration();
    assert_eq!(r.framework, "semantic-kernel");
    assert_eq!(r.items_migrated, 2);
    assert!(r.notes.contains("WritingPlugin"));
}
