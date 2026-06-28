/// templates - Load graph templates from presets.

use crate::import::{GraphSpec, SpecEdge, SpecNode};
use std::collections::HashMap;

/// A named template backed by a preset graph spec.
#[derive(Debug, Clone)]
pub struct Template {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: TemplateCategory,
    pub spec: GraphSpec,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateCategory {
    Basic,
    RagPipeline,
    MultiAgent,
    Verification,
    Custom(String),
}

impl std::fmt::Display for TemplateCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateCategory::Basic => write!(f, "basic"),
            TemplateCategory::RagPipeline => write!(f, "rag_pipeline"),
            TemplateCategory::MultiAgent => write!(f, "multi_agent"),
            TemplateCategory::Verification => write!(f, "verification"),
            TemplateCategory::Custom(s) => write!(f, "custom:{}", s),
        }
    }
}

/// Registry of all available templates.
#[derive(Debug, Default, Clone)]
pub struct TemplateRegistry {
    templates: Vec<Template>,
}

impl TemplateRegistry {
    pub fn new() -> Self {
        TemplateRegistry::default()
    }

    /// Create a registry pre-loaded with built-in templates.
    pub fn default_registry() -> Self {
        let mut r = TemplateRegistry::new();

        r.register(build_single_agent_template());
        r.register(build_rag_pipeline_template());
        r.register(build_agent_verifier_template());
        r.register(build_multi_agent_template());
        r.register(build_loop_template());

        r
    }

    pub fn register(&mut self, template: Template) {
        self.templates.push(template);
    }

    pub fn get(&self, id: &str) -> Option<&Template> {
        self.templates.iter().find(|t| t.id == id)
    }

    pub fn by_category(&self, cat: &TemplateCategory) -> Vec<&Template> {
        self.templates.iter().filter(|t| &t.category == cat).collect()
    }

    pub fn search(&self, query: &str) -> Vec<&Template> {
        let q = query.to_lowercase();
        self.templates
            .iter()
            .filter(|t| {
                t.name.to_lowercase().contains(&q)
                    || t.description.to_lowercase().contains(&q)
                    || t.tags.iter().any(|tag| tag.to_lowercase().contains(&q))
            })
            .collect()
    }

    pub fn all(&self) -> &[Template] {
        &self.templates
    }

    pub fn len(&self) -> usize {
        self.templates.len()
    }

    pub fn is_empty(&self) -> bool {
        self.templates.is_empty()
    }

    /// Clone a template's spec so the user can customise it.
    pub fn instantiate(&self, id: &str, new_name: impl Into<String>) -> Option<GraphSpec> {
        self.get(id).map(|t| {
            let mut spec = t.spec.clone();
            spec.name = new_name.into();
            spec
        })
    }
}

// ---- Built-in template builders ----

fn node(id: &str, kind: &str, label: &str, x: f64, y: f64) -> SpecNode {
    SpecNode {
        id: id.into(),
        kind: kind.into(),
        label: label.into(),
        x,
        y,
        config: HashMap::new(),
    }
}

fn edge(id: &str, src: &str, tgt: &str, etype: &str) -> SpecEdge {
    SpecEdge {
        id: id.into(),
        source: src.into(),
        target: tgt.into(),
        edge_type: etype.into(),
        label: None,
    }
}

fn build_single_agent_template() -> Template {
    let mut spec = GraphSpec::new("single_agent");
    spec.nodes.push(node("in", "control.router", "Input", 0.0, 0.0));
    spec.nodes.push(node("llm", "agent.llm", "LLM Agent", 200.0, 0.0));
    spec.nodes.push(node("out", "control.merge", "Output", 400.0, 0.0));
    spec.edges.push(edge("e1", "in", "llm", "data_flow"));
    spec.edges.push(edge("e2", "llm", "out", "data_flow"));

    Template {
        id: "single_agent".into(),
        name: "Single LLM Agent".into(),
        description: "A minimal graph with one LLM agent and input/output nodes.".into(),
        category: TemplateCategory::Basic,
        spec,
        tags: vec!["simple".into(), "llm".into(), "starter".into()],
    }
}

fn build_rag_pipeline_template() -> Template {
    let mut spec = GraphSpec::new("rag_pipeline");
    spec.nodes.push(node("query", "control.router", "Query", 0.0, 0.0));
    spec.nodes.push(node("retrieval", "agent.retrieval", "Retrieval", 200.0, 0.0));
    spec.nodes.push(node("llm", "agent.llm", "LLM Agent", 400.0, 0.0));
    spec.nodes.push(node("verify", "verifier.hallucination", "Hallucination Detector", 400.0, 120.0));
    spec.nodes.push(node("out", "control.merge", "Output", 600.0, 0.0));
    spec.edges.push(edge("e1", "query", "retrieval", "data_flow"));
    spec.edges.push(edge("e2", "retrieval", "llm", "data_flow"));
    spec.edges.push(edge("e3", "llm", "verify", "verification"));
    spec.edges.push(edge("e4", "verify", "out", "data_flow"));

    Template {
        id: "rag_pipeline".into(),
        name: "RAG Pipeline".into(),
        description: "Retrieval-augmented generation with a hallucination verifier.".into(),
        category: TemplateCategory::RagPipeline,
        spec,
        tags: vec!["rag".into(), "retrieval".into(), "hallucination".into()],
    }
}

fn build_agent_verifier_template() -> Template {
    let mut spec = GraphSpec::new("agent_verifier");
    spec.nodes.push(node("in", "control.router", "Input", 0.0, 0.0));
    spec.nodes.push(node("llm", "agent.llm", "LLM Agent", 200.0, 0.0));
    spec.nodes.push(node("schema_check", "verifier.json_schema", "Schema Verifier", 400.0, 0.0));
    spec.nodes.push(node("toxicity", "verifier.toxicity", "Toxicity Filter", 400.0, 100.0));
    spec.nodes.push(node("out", "control.merge", "Output", 600.0, 0.0));
    spec.edges.push(edge("e1", "in", "llm", "data_flow"));
    spec.edges.push(edge("e2", "llm", "schema_check", "verification"));
    spec.edges.push(edge("e3", "llm", "toxicity", "verification"));
    spec.edges.push(edge("e4", "schema_check", "out", "data_flow"));
    spec.edges.push(edge("e5", "toxicity", "out", "control_dep"));

    Template {
        id: "agent_verifier".into(),
        name: "Agent with Verifiers".into(),
        description: "An LLM agent whose output is checked by schema and toxicity verifiers.".into(),
        category: TemplateCategory::Verification,
        spec,
        tags: vec!["verifier".into(), "safety".into(), "schema".into()],
    }
}

fn build_multi_agent_template() -> Template {
    let mut spec = GraphSpec::new("multi_agent");
    spec.nodes.push(node("router", "control.router", "Router", 0.0, 0.0));
    spec.nodes.push(node("classifier", "agent.classifier", "Classifier", 200.0, -80.0));
    spec.nodes.push(node("llm_a", "agent.llm", "LLM Agent A", 200.0, 40.0));
    spec.nodes.push(node("llm_b", "agent.llm", "LLM Agent B", 200.0, 160.0));
    spec.nodes.push(node("merge", "control.merge", "Merge", 400.0, 40.0));
    spec.edges.push(edge("e1", "router", "classifier", "data_flow"));
    spec.edges.push(edge("e2", "router", "llm_a", "data_flow"));
    spec.edges.push(edge("e3", "router", "llm_b", "data_flow"));
    spec.edges.push(edge("e4", "classifier", "merge", "control_dep"));
    spec.edges.push(edge("e5", "llm_a", "merge", "data_flow"));
    spec.edges.push(edge("e6", "llm_b", "merge", "data_flow"));

    Template {
        id: "multi_agent".into(),
        name: "Multi-Agent Fan-Out".into(),
        description: "A router fans out to multiple agents whose results are merged.".into(),
        category: TemplateCategory::MultiAgent,
        spec,
        tags: vec!["multi-agent".into(), "fan-out".into(), "parallel".into()],
    }
}

fn build_loop_template() -> Template {
    let mut spec = GraphSpec::new("loop_template");
    spec.nodes.push(node("in", "control.router", "Input", 0.0, 0.0));
    spec.nodes.push(node("llm", "agent.llm", "LLM Agent", 200.0, 0.0));
    spec.nodes.push(node("check", "verifier.hallucination", "Quality Check", 400.0, 0.0));
    spec.nodes.push(node("loop_ctrl", "control.loop", "Loop Controller", 300.0, 120.0));
    spec.nodes.push(node("out", "control.merge", "Output", 600.0, 0.0));
    spec.edges.push(edge("e1", "in", "llm", "data_flow"));
    spec.edges.push(edge("e2", "llm", "check", "data_flow"));
    spec.edges.push(edge("e3", "check", "loop_ctrl", "control_dep"));
    spec.edges.push(edge("e4", "loop_ctrl", "llm", "loop_back"));
    spec.edges.push(edge("e5", "check", "out", "data_flow"));

    Template {
        id: "loop_template".into(),
        name: "Self-Refining Loop".into(),
        description: "An agent loops until a quality verifier is satisfied.".into(),
        category: TemplateCategory::Basic,
        spec,
        tags: vec!["loop".into(), "refinement".into(), "self-improving".into()],
    }
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn default_registry_non_empty() {
        let r = TemplateRegistry::default_registry();
        assert!(!r.is_empty());
    }

    #[test]
    fn get_template_by_id() {
        let r = TemplateRegistry::default_registry();
        let t = r.get("single_agent").expect("single_agent template missing");
        assert_eq!(t.category, TemplateCategory::Basic);
    }

    #[test]
    fn instantiate_clones_spec() {
        let r = TemplateRegistry::default_registry();
        let spec = r.instantiate("rag_pipeline", "my_rag").unwrap();
        assert_eq!(spec.name, "my_rag");
        assert!(!spec.nodes.is_empty());
    }

    #[test]
    fn search_finds_by_tag() {
        let r = TemplateRegistry::default_registry();
        let results = r.search("retrieval");
        assert!(!results.is_empty());
    }

    #[test]
    fn by_category_works() {
        let r = TemplateRegistry::default_registry();
        let basic = r.by_category(&TemplateCategory::Basic);
        assert!(!basic.is_empty());
    }
}
