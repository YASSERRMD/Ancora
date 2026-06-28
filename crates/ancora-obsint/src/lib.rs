/// ancora-obsint: Observability integrations for the Ancora agent framework.
///
/// Provides traces and metrics export to common backends with a self-hosted-only,
/// residency-safe option.

pub mod otlp;
pub mod langfuse;
pub mod phoenix;
pub mod grafana;
pub mod datadog;
pub mod prometheus;
pub mod selfhosted;
pub mod selection;
pub mod health;

#[cfg(test)]
mod tests {
    mod test_langfuse_mapping;
    mod test_phoenix_mapping;
    mod test_grafana_mapping;
    mod test_datadog_mapping;
    mod test_prometheus_scrape;
    mod test_selfhosted_blocks;
    mod test_exporter_selection;
    mod test_residency;
    mod test_health;
}
