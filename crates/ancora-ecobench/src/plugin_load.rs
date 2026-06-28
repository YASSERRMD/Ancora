//! Plugin load time measurement.
//!
//! Tracks the time required to discover, parse, and initialise a plugin
//! from a local path. No I/O is performed in library code; the timing
//! primitives wrap `std::time::Instant`.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Describes a plugin discovered on disk.
#[derive(Debug, Clone)]
pub struct PluginDescriptor {
    /// Human-readable name of the plugin.
    pub name: String,
    /// Reported version string.
    pub version: String,
    /// Arbitrary capability flags exposed by the plugin.
    pub capabilities: Vec<String>,
}

/// State machine for a single plugin load sequence.
#[derive(Debug)]
pub enum LoadState {
    /// Not yet started.
    Pending,
    /// Discovery phase complete; metadata available.
    Discovered(PluginDescriptor),
    /// Fully initialised and ready to call.
    Ready(PluginDescriptor),
    /// Terminal failure with a human-readable message.
    Failed(String),
}

/// Simulated result of loading a plugin, including timing.
#[derive(Debug)]
pub struct LoadResult {
    /// Final state after attempting load.
    pub state: LoadState,
    /// Wall-clock time consumed by the load sequence.
    pub elapsed: Duration,
    /// Breakdown of time per phase.
    pub phase_times: HashMap<String, Duration>,
}

/// Load a plugin from the supplied descriptor data.
///
/// This function simulates the three phases (discover, parse, init) without
/// performing real I/O, which makes it suitable for unit testing and
/// deterministic benchmarking.
pub fn load_plugin(name: &str, version: &str, caps: &[&str]) -> LoadResult {
    let overall_start = Instant::now();
    let mut phase_times = HashMap::new();

    // Phase 1: discover
    let t = Instant::now();
    let descriptor = PluginDescriptor {
        name: name.to_owned(),
        version: version.to_owned(),
        capabilities: caps.iter().map(|s| s.to_string()).collect(),
    };
    phase_times.insert("discover".to_owned(), t.elapsed());

    // Phase 2: parse
    let t = Instant::now();
    let _ = descriptor.capabilities.len(); // simulate work
    phase_times.insert("parse".to_owned(), t.elapsed());

    // Phase 3: init
    let t = Instant::now();
    let _ = descriptor.name.to_uppercase(); // simulate work
    phase_times.insert("init".to_owned(), t.elapsed());

    LoadResult {
        state: LoadState::Ready(descriptor),
        elapsed: overall_start.elapsed(),
        phase_times,
    }
}

/// Target threshold for a plugin load (used in regression checks).
pub const LOAD_TARGET_US: u64 = 5_000;

/// Returns `true` if the load completed within the regression threshold.
pub fn within_target(result: &LoadResult) -> bool {
    result.elapsed.as_micros() as u64 <= LOAD_TARGET_US
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_returns_ready() {
        let r = load_plugin("my-plugin", "1.0.0", &["call", "stream"]);
        assert!(matches!(r.state, LoadState::Ready(_)));
    }

    #[test]
    fn phase_times_recorded() {
        let r = load_plugin("p", "0.1", &[]);
        assert!(r.phase_times.contains_key("discover"));
        assert!(r.phase_times.contains_key("parse"));
        assert!(r.phase_times.contains_key("init"));
    }
}
