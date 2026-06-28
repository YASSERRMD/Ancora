use std::collections::HashMap;

/// A set of named string parameters for a recipe.
#[derive(Debug, Clone, Default)]
pub struct ParamSet {
    values: HashMap<String, String>,
}

impl ParamSet {
    /// Create an empty parameter set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a parameter value.
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.values.insert(key.into(), value.into());
    }

    /// Get a parameter value by key.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(|s| s.as_str())
    }

    /// Check whether a parameter is present.
    pub fn contains(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }

    /// Remove a parameter.
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.values.remove(key)
    }

    /// Return the number of parameters.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Return true if no parameters are set.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Merge another ParamSet into this one; values from `other` take precedence.
    pub fn merge(&mut self, other: &ParamSet) {
        for (k, v) in &other.values {
            self.values.insert(k.clone(), v.clone());
        }
    }

    /// Build a ParamSet from an iterator of (key, value) pairs.
    pub fn from_pairs<I, K, V>(iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        let mut ps = Self::new();
        for (k, v) in iter {
            ps.set(k, v);
        }
        ps
    }
}

/// Apply a parameter override string of the form "key=value" to a ParamSet.
/// Returns Err if the string is malformed.
pub fn apply_override(ps: &mut ParamSet, s: &str) -> Result<(), String> {
    let mut parts = s.splitn(2, '=');
    let key = parts.next().ok_or_else(|| "empty override".to_string())?;
    let val = parts
        .next()
        .ok_or_else(|| format!("no '=' found in override '{}'", s))?;
    if key.is_empty() {
        return Err("key must not be empty".to_string());
    }
    ps.set(key, val);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_get() {
        let mut ps = ParamSet::new();
        ps.set("top_k", "10");
        assert_eq!(ps.get("top_k"), Some("10"));
        assert_eq!(ps.get("missing"), None);
    }

    #[test]
    fn merge_precedence() {
        let mut base = ParamSet::from_pairs([("a", "1"), ("b", "2")]);
        let override_ps = ParamSet::from_pairs([("b", "99"), ("c", "3")]);
        base.merge(&override_ps);
        assert_eq!(base.get("a"), Some("1"));
        assert_eq!(base.get("b"), Some("99"));
        assert_eq!(base.get("c"), Some("3"));
    }

    #[test]
    fn apply_override_ok() {
        let mut ps = ParamSet::new();
        apply_override(&mut ps, "lang=rust").unwrap();
        assert_eq!(ps.get("lang"), Some("rust"));
    }

    #[test]
    fn apply_override_err_no_eq() {
        let mut ps = ParamSet::new();
        assert!(apply_override(&mut ps, "noequalssign").is_err());
    }
}
