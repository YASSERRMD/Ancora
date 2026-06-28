pub mod builder;
pub mod entry;
pub mod log;
pub mod stats;

pub use builder::AuditEntryBuilder;
pub use entry::{AuditEntry, Outcome, Severity};
pub use log::ImmutableAuditLog;
pub use stats::AuditStats;

#[cfg(test)]
mod tests {
    mod test_entry_checksum;
    mod test_entry_tamper;
    mod test_log_append;
    mod test_log_verify_all;
    mod test_log_filter_tenant;
    mod test_log_filter_subject;
    mod test_log_filter_operation;
    mod test_log_filter_tick_range;
    mod test_log_max_size;
    mod test_builder;
    mod test_stats_basic;
    mod test_stats_failure_rate;
    mod test_stats_severity;
    mod test_entry_details;
    mod test_log_get_by_id;
    mod test_log_count;
    mod test_outcome_variants;
    mod test_severity_variants;
    mod test_log_immutable_ids;
    mod test_log_empty;
}
