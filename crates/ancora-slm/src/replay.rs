//! Deterministic replay support for SLM orchestration.
//!
//! All orchestration patterns in this crate are designed to be replayable:
//! given the same inputs and the same "model function", the output is
//! identical.  This module provides:
//!
//! - A [`ReplayStore`] that records request/response pairs.
//! - A [`ReplayModelFn`] that returns recorded responses instead of calling a
//!   live model.
//! - Serialisation helpers for persisting replay stores to JSON.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single recorded interaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayEntry {
    /// The prompt that was sent to the model.
    pub prompt: String,
    /// The response the model produced.
    pub response: String,
}

/// A store of recorded model interactions for deterministic replay.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReplayStore {
    /// Maps prompt → response.  When the same prompt is seen, the stored
    /// response is returned immediately without invoking any real model.
    entries: HashMap<String, String>,
    /// Ordered list of all recorded entries (for serialisation / inspection).
    log: Vec<ReplayEntry>,
}

impl ReplayStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a new prompt/response pair.
    pub fn record(&mut self, prompt: impl Into<String>, response: impl Into<String>) {
        let p = prompt.into();
        let r = response.into();
        self.entries.insert(p.clone(), r.clone());
        self.log.push(ReplayEntry { prompt: p, response: r });
    }

    /// Look up the recorded response for a given prompt.
    /// Returns `None` if no matching entry exists.
    pub fn lookup(&self, prompt: &str) -> Option<&str> {
        self.entries.get(prompt).map(String::as_str)
    }

    /// Number of recorded entries.
    pub fn len(&self) -> usize {
        self.log.len()
    }

    pub fn is_empty(&self) -> bool {
        self.log.is_empty()
    }

    /// Serialise the store to a JSON string.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialise a store from a JSON string.
    pub fn from_json(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }

    /// Export just the ordered log.
    pub fn log(&self) -> &[ReplayEntry] {
        &self.log
    }
}

/// A model function backed by a [`ReplayStore`].
///
/// When a prompt is found in the store, the stored response is returned.
/// When not found, `default_response` is returned (useful for tests that only
/// need to stub specific prompts).
pub struct ReplayModelFn {
    store: ReplayStore,
    default_response: String,
}

impl ReplayModelFn {
    pub fn new(store: ReplayStore, default_response: impl Into<String>) -> Self {
        Self { store, default_response: default_response.into() }
    }

    pub fn call(&self, prompt: &str) -> String {
        self.store
            .lookup(prompt)
            .unwrap_or(&self.default_response)
            .to_string()
    }
}

/// Build a simple deterministic model function from a static list of
/// (prompt, response) pairs.  Useful for unit tests.
pub fn make_replay_fn(
    pairs: Vec<(String, String)>,
    default_response: impl Into<String>,
) -> impl Fn(&str) -> String {
    let store_map: HashMap<String, String> = pairs.into_iter().collect();
    let default = default_response.into();
    move |prompt: &str| -> String {
        store_map.get(prompt).cloned().unwrap_or_else(|| default.clone())
    }
}
