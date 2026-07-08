/// Type of catalog entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatalogKind {
    Metric,
    Eval,
    Dashboard,
    Alert,
    Trace,
}

impl std::fmt::Display for CatalogKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CatalogKind::Metric => "metric",
            CatalogKind::Eval => "eval",
            CatalogKind::Dashboard => "dashboard",
            CatalogKind::Alert => "alert",
            CatalogKind::Trace => "trace",
        };
        write!(f, "{}", s)
    }
}

/// A single entry in the metrics and evals catalog.
#[derive(Debug, Clone)]
pub struct CatalogEntry {
    pub id: String,
    pub kind: CatalogKind,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub stable: bool,
}

impl CatalogEntry {
    pub fn new(
        id: impl Into<String>,
        kind: CatalogKind,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            name: name.into(),
            description: description.into(),
            tags: Vec::new(),
            stable: false,
        }
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn stable(mut self) -> Self {
        self.stable = true;
        self
    }

    pub fn render(&self) -> String {
        format!(
            "[{}] {} ({}) - {}{}\n  Tags: {}",
            self.id,
            self.name,
            self.kind,
            self.description,
            if self.stable { " [stable]" } else { "" },
            self.tags.join(", ")
        )
    }
}

/// Full catalog index of metrics and evals.
#[derive(Debug, Default)]
pub struct CatalogIndex {
    entries: Vec<CatalogEntry>,
}

impl CatalogIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_entry(mut self, entry: CatalogEntry) -> Self {
        self.entries.push(entry);
        self
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn by_kind(&self, kind: &CatalogKind) -> Vec<&CatalogEntry> {
        self.entries.iter().filter(|e| &e.kind == kind).collect()
    }

    pub fn stable_entries(&self) -> Vec<&CatalogEntry> {
        self.entries.iter().filter(|e| e.stable).collect()
    }

    pub fn search_by_tag(&self, tag: &str) -> Vec<&CatalogEntry> {
        self.entries
            .iter()
            .filter(|e| e.tags.iter().any(|t| t == tag))
            .collect()
    }

    pub fn render(&self) -> String {
        let mut out = format!("Catalog Index ({} entries)\n\n", self.len());
        for e in &self.entries {
            out.push_str(&format!("{}\n\n", e.render()));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_add_and_len() {
        let c = CatalogIndex::new()
            .add_entry(CatalogEntry::new(
                "M001",
                CatalogKind::Metric,
                "agent.request.latency",
                "P99 request latency",
            ))
            .add_entry(CatalogEntry::new(
                "E001",
                CatalogKind::Eval,
                "factual-accuracy",
                "LLM factual accuracy score",
            ));
        assert_eq!(c.len(), 2);
    }

    #[test]
    fn filter_by_kind() {
        let c = CatalogIndex::new()
            .add_entry(CatalogEntry::new("M1", CatalogKind::Metric, "m", "m"))
            .add_entry(CatalogEntry::new("E1", CatalogKind::Eval, "e", "e"));
        let metrics = c.by_kind(&CatalogKind::Metric);
        assert_eq!(metrics.len(), 1);
        assert_eq!(metrics[0].id, "M1");
    }

    #[test]
    fn search_by_tag() {
        let c = CatalogIndex::new().add_entry(
            CatalogEntry::new("M2", CatalogKind::Metric, "cpu", "cpu usage")
                .with_tags(vec!["infra".into(), "system".into()]),
        );
        let results = c.search_by_tag("infra");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn stable_entries_filtered() {
        let c = CatalogIndex::new()
            .add_entry(CatalogEntry::new("M3", CatalogKind::Metric, "rps", "req/s").stable())
            .add_entry(CatalogEntry::new(
                "E2",
                CatalogKind::Eval,
                "rouge",
                "rouge score",
            ));
        assert_eq!(c.stable_entries().len(), 1);
    }
}
