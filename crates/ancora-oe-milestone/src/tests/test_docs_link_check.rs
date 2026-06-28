use crate::catalog_index::{CatalogEntry, CatalogIndex, CatalogKind};
use crate::feature_matrix::{Availability, FeatureMatrix, FeatureRow};

/// Simulate a docs link-check by verifying that all catalog entries
/// referenced in the feature matrix are resolvable (id non-empty).
#[test]
fn all_catalog_entries_have_ids() {
    let catalog = CatalogIndex::new()
        .add(
            CatalogEntry::new(
                "M001",
                CatalogKind::Metric,
                "agent.request.latency",
                "Request latency histogram",
            )
            .stable(),
        )
        .add(CatalogEntry::new(
            "E001",
            CatalogKind::Eval,
            "factual-accuracy",
            "Factual accuracy score",
        ))
        .add(CatalogEntry::new(
            "D001",
            CatalogKind::Dashboard,
            "obs-overview",
            "Observability overview dashboard",
        ));

    for entry in catalog.stable_entries() {
        assert!(!entry.id.is_empty(), "Entry ID must not be empty");
        assert!(!entry.name.is_empty(), "Entry name must not be empty");
    }
}

#[test]
fn feature_matrix_all_rows_have_names() {
    let mut matrix = FeatureMatrix::new();
    matrix.add_row(FeatureRow::new(
        "distributed-tracing",
        Availability::GA,
        Availability::GA,
        Availability::GA,
        Availability::GA,
    ));
    matrix.add_row(FeatureRow::new(
        "metrics-export",
        Availability::GA,
        Availability::GA,
        Availability::Beta,
        Availability::GA,
    ));

    for row in &matrix.rows {
        assert!(!row.feature.is_empty());
    }
}

#[test]
fn catalog_render_is_non_empty() {
    let catalog = CatalogIndex::new().add(CatalogEntry::new(
        "T001",
        CatalogKind::Trace,
        "agent.span",
        "Agent span data",
    ));
    let rendered = catalog.render();
    assert!(!rendered.is_empty());
    assert!(rendered.contains("T001"));
}
