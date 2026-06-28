use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct RevocationStore {
    revoked: HashSet<String>,
}

impl RevocationStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn revoke(&mut self, token_raw: impl Into<String>) {
        self.revoked.insert(token_raw.into());
    }

    pub fn is_revoked(&self, token_raw: &str) -> bool {
        self.revoked.contains(token_raw)
    }

    pub fn count(&self) -> usize {
        self.revoked.len()
    }
}

pub fn revoke_all(store: &mut RevocationStore, tokens: impl IntoIterator<Item = String>) {
    for t in tokens {
        store.revoke(t);
    }
}
