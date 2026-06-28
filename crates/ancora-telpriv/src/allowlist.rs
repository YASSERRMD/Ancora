/// Allowlist of safe telemetry attribute names.
///
/// Only attributes on the allowlist may be exported without redaction.
/// Any attribute not on the list is subject to the classification policy.

use std::collections::HashSet;

/// A set of attribute name prefixes and exact names that are safe to export.
#[derive(Debug, Clone)]
pub struct Allowlist {
    /// Exact attribute names that are always safe.
    exact: HashSet<String>,
    /// Prefixes where any attribute name starting with the prefix is safe.
    prefixes: Vec<String>,
}

impl Allowlist {
    /// Construct an empty allowlist.
    pub fn empty() -> Self {
        Allowlist {
            exact: HashSet::new(),
            prefixes: Vec::new(),
        }
    }

    /// Construct the default safe-attribute allowlist for Ancora telemetry.
    pub fn default_safe() -> Self {
        let mut al = Self::empty();
        // Standard OTel semantic conventions that are safe.
        for name in &[
            "span.name",
            "span.kind",
            "span.status",
            "http.method",
            "http.status_code",
            "http.url",
            "http.target",
            "http.host",
            "net.peer.port",
            "rpc.method",
            "rpc.service",
            "db.system",
            "db.operation",
            "agent.id",
            "agent.version",
            "task.id",
            "task.kind",
            "model.name",
            "model.provider",
            "tool.name",
            "error.kind",
            "duration_ms",
            "token.input_count",
            "token.output_count",
        ] {
            al.exact.insert(name.to_string());
        }
        // Safe prefixes.
        for prefix in &["metric.", "trace.", "resource."] {
            al.prefixes.push(prefix.to_string());
        }
        al
    }

    /// Add an exact name to the allowlist.
    pub fn add_exact(&mut self, name: impl Into<String>) {
        self.exact.insert(name.into());
    }

    /// Add a prefix to the allowlist.
    pub fn add_prefix(&mut self, prefix: impl Into<String>) {
        self.prefixes.push(prefix.into());
    }

    /// Returns true if the attribute name is on the allowlist.
    pub fn is_allowed(&self, name: &str) -> bool {
        if self.exact.contains(name) {
            return true;
        }
        self.prefixes.iter().any(|p| name.starts_with(p.as_str()))
    }

    /// Filter a list of attributes to those on the allowlist.
    pub fn filter<'a>(&self, attrs: &'a [(String, String)]) -> Vec<&'a (String, String)> {
        attrs.iter().filter(|(k, _)| self.is_allowed(k)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_allows_span_name() {
        let al = Allowlist::default_safe();
        assert!(al.is_allowed("span.name"));
    }

    #[test]
    fn default_blocks_email() {
        let al = Allowlist::default_safe();
        assert!(!al.is_allowed("user.email"));
    }

    #[test]
    fn prefix_match() {
        let al = Allowlist::default_safe();
        assert!(al.is_allowed("metric.latency"));
    }

    #[test]
    fn add_exact_works() {
        let mut al = Allowlist::empty();
        al.add_exact("custom.field");
        assert!(al.is_allowed("custom.field"));
        assert!(!al.is_allowed("custom.other"));
    }
}
