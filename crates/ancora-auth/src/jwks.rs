use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct JwkKey {
    pub kid: String,
    pub kty: String,
    pub alg: String,
    pub n: String,
    pub e: String,
    pub valid_from_tick: u64,
    pub valid_until_tick: u64,
}

impl JwkKey {
    pub fn new(
        kid: impl Into<String>,
        n: impl Into<String>,
        e: impl Into<String>,
        valid_from_tick: u64,
        valid_until_tick: u64,
    ) -> Self {
        Self {
            kid: kid.into(),
            kty: "RSA".into(),
            alg: "RS256".into(),
            n: n.into(),
            e: e.into(),
            valid_from_tick,
            valid_until_tick,
        }
    }

    pub fn is_active(&self, tick: u64) -> bool {
        tick >= self.valid_from_tick && tick < self.valid_until_tick
    }
}

#[derive(Debug, Default)]
pub struct JwksStore {
    keys: HashMap<String, JwkKey>,
}

impl JwksStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_key(&mut self, key: JwkKey) {
        self.keys.insert(key.kid.clone(), key);
    }

    pub fn get_key(&self, kid: &str) -> Option<&JwkKey> {
        self.keys.get(kid)
    }

    pub fn active_keys(&self, tick: u64) -> Vec<&JwkKey> {
        self.keys.values().filter(|k| k.is_active(tick)).collect()
    }

    pub fn rotate(&mut self, old_kid: &str, new_key: JwkKey) {
        self.keys.remove(old_kid);
        self.keys.insert(new_key.kid.clone(), new_key);
    }

    pub fn key_count(&self) -> usize {
        self.keys.len()
    }
}
