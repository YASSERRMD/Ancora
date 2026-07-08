pub mod alert;
pub mod audit;
pub mod builder;
pub mod feed;
pub mod indicator;
pub mod policy;
pub mod presets;
pub mod query;
pub mod report;
pub mod score;
pub mod stats;
pub mod store;
pub mod summary;

#[cfg(test)]
mod tests {
    mod test_alert;
    mod test_alert_status_display;
    mod test_alert_store;
    mod test_audit;
    mod test_audit_action_display;
    mod test_builder;
    mod test_feed;
    mod test_feed_store;
    mod test_indicator;
    mod test_indicator_kind_display;
    mod test_policy;
    mod test_policy_decision;
    mod test_presets;
    mod test_query;
    mod test_query_min_level;
    mod test_query_tag;
    mod test_report;
    mod test_score;
    mod test_score_levels;
    mod test_scorer_recency;
    mod test_stats;
    mod test_stats_critical_free;
    mod test_store;
    mod test_store_by_kind;
    mod test_store_by_level;
    mod test_store_expired;
    mod test_summary;
    mod test_threat_level_display;
    mod test_threat_level_ordering;
}
