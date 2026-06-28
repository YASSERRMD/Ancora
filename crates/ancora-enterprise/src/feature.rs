use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeatureState {
    Enabled,
    Disabled,
    BetaOnly,
    Deprecated,
}

impl fmt::Display for FeatureState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            FeatureState::Enabled => "ENABLED",
            FeatureState::Disabled => "DISABLED",
            FeatureState::BetaOnly => "BETA_ONLY",
            FeatureState::Deprecated => "DEPRECATED",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct FeatureFlag {
    pub name: String,
    pub state: FeatureState,
    pub description: String,
}

impl FeatureFlag {
    pub fn new(name: impl Into<String>, state: FeatureState, description: impl Into<String>) -> Self {
        Self { name: name.into(), state, description: description.into() }
    }

    pub fn is_active(&self) -> bool { self.state == FeatureState::Enabled }
    pub fn is_beta(&self) -> bool { self.state == FeatureState::BetaOnly }
}

pub struct FeatureRegistry {
    flags: HashMap<String, FeatureFlag>,
}

impl FeatureRegistry {
    pub fn new() -> Self { Self { flags: HashMap::new() } }
    pub fn register(&mut self, flag: FeatureFlag) { self.flags.insert(flag.name.clone(), flag); }
    pub fn get(&self, name: &str) -> Option<&FeatureFlag> { self.flags.get(name) }
    pub fn is_enabled(&self, name: &str) -> bool { self.get(name).map(|f| f.is_active()).unwrap_or(false) }
    pub fn enable(&mut self, name: &str) {
        if let Some(f) = self.flags.get_mut(name) { f.state = FeatureState::Enabled; }
    }
    pub fn disable(&mut self, name: &str) {
        if let Some(f) = self.flags.get_mut(name) { f.state = FeatureState::Disabled; }
    }
    pub fn count(&self) -> usize { self.flags.len() }
    pub fn enabled_count(&self) -> usize { self.flags.values().filter(|f| f.is_active()).count() }
    pub fn all(&self) -> impl Iterator<Item = &FeatureFlag> { self.flags.values() }
}
