/// Per-plugin filesystem policy.
///
/// Controls which paths a plugin may read from and write to.  The host
/// intercepts filesystem syscalls and consults this policy before granting
/// access.

/// A filesystem path rule.
#[derive(Debug, Clone)]
pub struct PathRule {
    /// The path prefix this rule applies to.
    pub prefix: String,
    /// Whether the plugin may read files under this prefix.
    pub read: bool,
    /// Whether the plugin may write files under this prefix.
    pub write: bool,
}

impl PathRule {
    pub fn read_only(prefix: impl Into<String>) -> Self {
        Self { prefix: prefix.into(), read: true, write: false }
    }

    pub fn read_write(prefix: impl Into<String>) -> Self {
        Self { prefix: prefix.into(), read: true, write: true }
    }

    pub fn write_only(prefix: impl Into<String>) -> Self {
        Self { prefix: prefix.into(), read: false, write: true }
    }
}

/// Access mode being requested.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessMode {
    Read,
    Write,
}

/// Complete filesystem policy for a plugin.
#[derive(Debug, Clone)]
pub struct FilesystemPolicy {
    /// Ordered list of path rules; first matching rule wins.
    pub rules: Vec<PathRule>,
    /// When `true`, access to paths not matching any rule is denied.
    pub default_deny: bool,
}

impl FilesystemPolicy {
    /// Deny all filesystem access.
    pub fn deny_all() -> Self {
        Self { rules: vec![], default_deny: true }
    }

    /// Allow unrestricted filesystem access (not recommended for plugins).
    pub fn allow_all() -> Self {
        Self { rules: vec![], default_deny: false }
    }

    /// Add a path rule at the end of the rule list.
    pub fn add_rule(&mut self, rule: PathRule) {
        self.rules.push(rule);
    }

    /// Check whether the requested access to `path` is permitted.
    pub fn permits(&self, path: &str, mode: AccessMode) -> bool {
        for rule in &self.rules {
            if path.starts_with(&rule.prefix) {
                return match mode {
                    AccessMode::Read => rule.read,
                    AccessMode::Write => rule.write,
                };
            }
        }
        !self.default_deny
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deny_all_blocks_all_access() {
        let policy = FilesystemPolicy::deny_all();
        assert!(!policy.permits("/etc/passwd", AccessMode::Read));
        assert!(!policy.permits("/tmp/foo", AccessMode::Write));
    }

    #[test]
    fn read_only_rule() {
        let mut policy = FilesystemPolicy::deny_all();
        policy.add_rule(PathRule::read_only("/var/data/plugin-cache"));
        assert!(policy.permits("/var/data/plugin-cache/file.db", AccessMode::Read));
        assert!(!policy.permits("/var/data/plugin-cache/file.db", AccessMode::Write));
        assert!(!policy.permits("/etc/shadow", AccessMode::Read));
    }

    #[test]
    fn read_write_rule() {
        let mut policy = FilesystemPolicy::deny_all();
        policy.add_rule(PathRule::read_write("/tmp/plugin-scratch"));
        assert!(policy.permits("/tmp/plugin-scratch/work.tmp", AccessMode::Read));
        assert!(policy.permits("/tmp/plugin-scratch/work.tmp", AccessMode::Write));
    }
}
