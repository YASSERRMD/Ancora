use crate::secret::{Secret, SecretKind};
use crate::store::SecretStore;

pub struct SecretQuery {
    kind: Option<SecretKind>,
    min_versions: Option<usize>,
    has_ttl: Option<bool>,
    path_prefix: Option<String>,
}

impl SecretQuery {
    pub fn new() -> Self {
        Self {
            kind: None,
            min_versions: None,
            has_ttl: None,
            path_prefix: None,
        }
    }

    pub fn kind(mut self, k: SecretKind) -> Self {
        self.kind = Some(k);
        self
    }
    pub fn min_versions(mut self, n: usize) -> Self {
        self.min_versions = Some(n);
        self
    }
    pub fn has_ttl(mut self, v: bool) -> Self {
        self.has_ttl = Some(v);
        self
    }
    pub fn path_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.path_prefix = Some(prefix.into());
        self
    }

    pub fn run<'a>(&self, store: &'a SecretStore, tenant_id: &str) -> Vec<&'a Secret> {
        store
            .list_tenant(tenant_id)
            .into_iter()
            .filter(|s| {
                self.kind.as_ref().map_or(true, |k| &s.kind == k)
                    && self.min_versions.map_or(true, |n| s.version_count() >= n)
                    && self.has_ttl.map_or(true, |v| s.ttl_ticks.is_some() == v)
                    && self
                        .path_prefix
                        .as_deref()
                        .map_or(true, |p| s.path.starts_with(p))
            })
            .collect()
    }
}

impl Default for SecretQuery {
    fn default() -> Self {
        Self::new()
    }
}
