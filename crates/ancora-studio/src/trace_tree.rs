/// Trace tree - nested span view of an agent run (parent/child spans).

#[derive(Debug, Clone, PartialEq)]
pub enum SpanKind {
    Agent,
    LlmCall,
    ToolCall,
    Retrieval,
    Checkpoint,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct Span {
    pub id: String,
    pub parent_id: Option<String>,
    pub kind: SpanKind,
    pub name: String,
    pub start_ms: u64,
    pub end_ms: u64,
    pub attributes: std::collections::HashMap<String, String>,
    pub error: Option<String>,
}

impl Span {
    pub fn duration_ms(&self) -> u64 {
        self.end_ms.saturating_sub(self.start_ms)
    }

    pub fn is_error(&self) -> bool {
        self.error.is_some()
    }
}

pub struct TraceTree {
    spans: Vec<Span>,
}

#[derive(Debug)]
pub struct SpanNode<'a> {
    pub span: &'a Span,
    pub children: Vec<SpanNode<'a>>,
}

impl TraceTree {
    pub fn new(spans: Vec<Span>) -> Self {
        Self { spans }
    }

    pub fn root_spans(&self) -> Vec<&Span> {
        self.spans.iter().filter(|s| s.parent_id.is_none()).collect()
    }

    pub fn children_of(&self, parent_id: &str) -> Vec<&Span> {
        self.spans
            .iter()
            .filter(|s| s.parent_id.as_deref() == Some(parent_id))
            .collect()
    }

    pub fn build_tree(&self) -> Vec<SpanNode<'_>> {
        self.root_spans()
            .into_iter()
            .map(|s| self.build_node(s))
            .collect()
    }

    fn build_node<'a>(&'a self, span: &'a Span) -> SpanNode<'a> {
        let children = self
            .children_of(&span.id)
            .into_iter()
            .map(|c| self.build_node(c))
            .collect();
        SpanNode { span, children }
    }

    pub fn find_span(&self, id: &str) -> Option<&Span> {
        self.spans.iter().find(|s| s.id == id)
    }

    pub fn error_count(&self) -> usize {
        self.spans.iter().filter(|s| s.is_error()).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_span(id: &str, parent: Option<&str>, kind: SpanKind) -> Span {
        Span {
            id: id.into(),
            parent_id: parent.map(|s| s.into()),
            kind,
            name: format!("span-{}", id),
            start_ms: 0,
            end_ms: 10,
            attributes: Default::default(),
            error: None,
        }
    }

    #[test]
    fn test_tree_structure() {
        let tree = TraceTree::new(vec![
            make_span("root", None, SpanKind::Agent),
            make_span("child1", Some("root"), SpanKind::LlmCall),
            make_span("child2", Some("root"), SpanKind::ToolCall),
        ]);
        let roots = tree.root_spans();
        assert_eq!(roots.len(), 1);
        let children = tree.children_of("root");
        assert_eq!(children.len(), 2);
    }

    #[test]
    fn test_build_tree() {
        let tree = TraceTree::new(vec![
            make_span("root", None, SpanKind::Agent),
            make_span("child", Some("root"), SpanKind::LlmCall),
        ]);
        let nodes = tree.build_tree();
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].children.len(), 1);
    }
}
