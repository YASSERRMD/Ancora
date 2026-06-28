use crate::schema::ToolDef;

/// Selects the most appropriate tools from a registry given a query.
/// In production this would embed the query; here we rank by name substring match.
pub struct ToolSelector {
    pub max_results: usize,
}

impl ToolSelector {
    pub fn new(max_results: usize) -> Self {
        Self { max_results }
    }

    pub fn select<'a>(&self, tools: &[&'a ToolDef], query: &str) -> Vec<&'a ToolDef> {
        let q = query.to_lowercase();
        let mut matched: Vec<&ToolDef> = tools
            .iter()
            .filter(|t| {
                t.name.to_lowercase().contains(&q)
                    || t.description.to_lowercase().contains(&q)
            })
            .copied()
            .collect();
        matched.truncate(self.max_results);
        matched
    }

    pub fn select_exact<'a>(&self, tools: &[&'a ToolDef], name: &str) -> Option<&'a ToolDef> {
        tools.iter().find(|t| t.name == name).copied()
    }
}
