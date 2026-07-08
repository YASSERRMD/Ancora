pub mod allowlist;
/// ancora-telpriv: Telemetry privacy and redaction for the Ancora agent framework.
///
/// Telemetry is redaction-first, classification-aware, and never leaks
/// sensitive data by default. All PII is scrubbed at the source before any
/// record leaves the process boundary.
pub mod audit;
pub mod classification;
pub mod eval_policy;
pub mod hashing;
pub mod log_policy;
pub mod opt_in;
pub mod pii_scrub;
pub mod span_policy;

#[cfg(test)]
mod tests {
    mod test_allowlist;
    mod test_classified_not_exported;
    mod test_decisions_audited;
    mod test_eval_redacted;
    mod test_hashing_correlation;
    mod test_logs_redacted;
    mod test_pii_scrubbed_spans;
    mod test_prompt_capture_off;
    mod test_redaction_reproducible;
}
