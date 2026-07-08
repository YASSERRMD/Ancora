use ancora_skills::{
    Crew, JitLoader, SkillDescriptor, SkillJournal, SkillRegistry, SkillScope, SubAgentNode,
};
use serde_json::json;

pub fn run_skills_subagent_example() {
    let mut registry = SkillRegistry::default();
    let mut loader = JitLoader::new();

    let search = SkillDescriptor::new(
        "search",
        1,
        "keyword search",
        vec!["retrieval"],
        SkillScope::ReadOnly,
    );
    let summarize = SkillDescriptor::new(
        "summarize",
        1,
        "text summarization",
        vec!["nlp"],
        SkillScope::ReadOnly,
    );

    loader.load_on_demand(&mut registry, search).unwrap();
    loader.load_on_demand(&mut registry, summarize).unwrap();

    assert!(registry.find("search").is_some());
    assert_eq!(registry.by_tag("retrieval").len(), 1);

    let node = SubAgentNode::new("n1", "agent-search", json!({"query": "rust async"}));
    let result = node.invoke(&SkillScope::ReadOnly).unwrap();
    assert_eq!(result.output["status"], "ok");

    let crew = Crew::new("research", vec!["search", "summarize"]);
    let resolved = crew.resolve(&registry).unwrap();
    assert_eq!(resolved.len(), 2);

    let mut journal = SkillJournal::default();
    journal.record(1, "search", 1, "n1");
    journal.record(2, "summarize", 1, "n2");
    let replayed = journal.replay();
    assert_eq!(replayed, vec![("search", 1), ("summarize", 1)]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skills_subagent_example_runs() {
        run_skills_subagent_example();
    }
}
