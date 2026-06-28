use std::collections::HashSet;

/// The outcome of an access-control check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessResult {
    /// The caller is allowed to proceed.
    Allowed,
    /// The caller is denied; the string explains why.
    Denied(String),
}

impl AccessResult {
    pub fn is_allowed(&self) -> bool {
        matches!(self, Self::Allowed)
    }
}

/// Policy controlling which publishers are permitted to publish to this registry.
#[derive(Debug, Clone)]
pub enum AccessPolicy {
    /// Any publisher is accepted.
    Open,
    /// Only publishers whose identity appears in the allow-list are accepted.
    AllowList(HashSet<String>),
    /// All publishers are denied (read-only registry).
    DenyAll,
}

impl AccessPolicy {
    /// Build an AllowList policy from an iterator of publisher identities.
    pub fn allow_list<I: IntoIterator<Item = S>, S: Into<String>>(ids: I) -> Self {
        Self::AllowList(ids.into_iter().map(Into::into).collect())
    }

    /// Evaluate the policy against a publisher identity.
    pub fn check(&self, publisher: &str) -> AccessResult {
        match self {
            Self::Open => AccessResult::Allowed,
            Self::AllowList(set) => {
                if set.contains(publisher) {
                    AccessResult::Allowed
                } else {
                    AccessResult::Denied(format!("publisher '{publisher}' is not on the allow-list"))
                }
            }
            Self::DenyAll => AccessResult::Denied("registry is read-only".to_string()),
        }
    }

    /// Add a publisher to an AllowList policy (no-op for other variants).
    pub fn add_publisher(&mut self, publisher: impl Into<String>) {
        if let Self::AllowList(set) = self {
            set.insert(publisher.into());
        }
    }

    /// Remove a publisher from an AllowList policy (no-op for other variants).
    pub fn remove_publisher(&mut self, publisher: &str) {
        if let Self::AllowList(set) = self {
            set.remove(publisher);
        }
    }
}

impl Default for AccessPolicy {
    fn default() -> Self {
        Self::Open
    }
}
