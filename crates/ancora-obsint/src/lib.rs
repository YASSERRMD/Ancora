pub mod datadog;
pub mod grafana;
pub mod health;
pub mod langfuse;
/// ancora-obsint: Observability integrations for the Ancora agent framework.
///
/// Provides traces and metrics export to common backends with a self-hosted-only,
/// residency-safe option.
pub mod otlp;
pub mod phoenix;
pub mod prometheus;
pub mod selection;
pub mod selfhosted;

#[cfg(test)]
mod tests {
    mod test_datadog_mapping;
    mod test_exporter_selection;
    mod test_grafana_mapping;
    mod test_health;
    mod test_langfuse_mapping;
    mod test_phoenix_mapping;
    mod test_prometheus_scrape;
    mod test_residency;
    mod test_selfhosted_blocks;
}
