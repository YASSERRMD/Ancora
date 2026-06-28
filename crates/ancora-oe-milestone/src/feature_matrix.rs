/// Availability tier for a feature across runtimes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Availability {
    GA,
    Beta,
    Preview,
    Planned,
    NotApplicable,
}

impl std::fmt::Display for Availability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Availability::GA => "GA",
            Availability::Beta => "Beta",
            Availability::Preview => "Preview",
            Availability::Planned => "Planned",
            Availability::NotApplicable => "N/A",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub struct FeatureRow {
    pub feature: String,
    pub rust: Availability,
    pub python: Availability,
    pub go: Availability,
    pub typescript: Availability,
}

impl FeatureRow {
    pub fn new(
        feature: impl Into<String>,
        rust: Availability,
        python: Availability,
        go: Availability,
        typescript: Availability,
    ) -> Self {
        Self {
            feature: feature.into(),
            rust,
            python,
            go,
            typescript,
        }
    }

    pub fn is_fully_ga(&self) -> bool {
        self.rust == Availability::GA
            && self.python == Availability::GA
            && self.go == Availability::GA
            && self.typescript == Availability::GA
    }

    pub fn render_row(&self) -> String {
        format!(
            "| {} | {} | {} | {} | {} |",
            self.feature, self.rust, self.python, self.go, self.typescript
        )
    }
}

#[derive(Debug, Default)]
pub struct FeatureMatrix {
    pub rows: Vec<FeatureRow>,
}

impl FeatureMatrix {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_row(&mut self, row: FeatureRow) {
        self.rows.push(row);
    }

    pub fn render_table(&self) -> String {
        let header = "| Feature | Rust | Python | Go | TypeScript |";
        let sep = "| --- | --- | --- | --- | --- |";
        let rows: Vec<String> = self.rows.iter().map(|r| r.render_row()).collect();
        std::iter::once(header)
            .chain(std::iter::once(sep))
            .chain(rows.iter().map(|s| s.as_str()))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn ga_count(&self) -> usize {
        self.rows.iter().filter(|r| r.is_fully_ga()).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_ga_detection() {
        let row = FeatureRow::new(
            "metrics",
            Availability::GA,
            Availability::GA,
            Availability::GA,
            Availability::GA,
        );
        assert!(row.is_fully_ga());
    }

    #[test]
    fn matrix_render_not_empty() {
        let mut m = FeatureMatrix::new();
        m.add_row(FeatureRow::new(
            "tracing",
            Availability::GA,
            Availability::GA,
            Availability::Beta,
            Availability::GA,
        ));
        let table = m.render_table();
        assert!(table.contains("tracing"));
        assert!(table.contains("Beta"));
    }

    #[test]
    fn ga_count_correct() {
        let mut m = FeatureMatrix::new();
        m.add_row(FeatureRow::new(
            "a",
            Availability::GA,
            Availability::GA,
            Availability::GA,
            Availability::GA,
        ));
        m.add_row(FeatureRow::new(
            "b",
            Availability::GA,
            Availability::GA,
            Availability::Beta,
            Availability::GA,
        ));
        assert_eq!(m.ga_count(), 1);
    }
}
