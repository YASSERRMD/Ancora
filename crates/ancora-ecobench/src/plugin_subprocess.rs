//! Plugin call overhead for subprocess-based plugins.
//!
//! Models the round-trip cost of invoking a plugin function via a child
//! process. The subprocess lifecycle (spawn, write stdin, read stdout, wait)
//! is simulated in-process so that benchmarks are deterministic and do not
//! require any external binary.

use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Configuration for a simulated subprocess plugin.
#[derive(Debug, Clone)]
pub struct SubprocessConfig {
    /// Simulated executable path.
    pub executable: String,
    /// Extra arguments forwarded to the child.
    pub args: Vec<String>,
    /// Whether to reuse a long-lived process across calls.
    pub persistent: bool,
}

impl SubprocessConfig {
    /// Create a default config for the given executable.
    pub fn new(executable: &str) -> Self {
        Self {
            executable: executable.to_owned(),
            args: Vec::new(),
            persistent: false,
        }
    }
}

/// A handle representing an active (simulated) subprocess.
pub struct SubprocessHandle {
    config: SubprocessConfig,
    /// Pending output lines queued by the simulation.
    output_queue: VecDeque<String>,
    /// Number of calls made through this handle.
    call_count: u64,
}

impl SubprocessHandle {
    /// Spawn (simulate) the subprocess described by `config`.
    pub fn spawn(config: SubprocessConfig) -> Self {
        Self {
            config,
            output_queue: VecDeque::new(),
            call_count: 0,
        }
    }

    /// The executable path this handle was created from.
    pub fn executable(&self) -> &str {
        &self.config.executable
    }

    /// Total number of calls made through this handle.
    pub fn call_count(&self) -> u64 {
        self.call_count
    }
}

/// Result of a single subprocess plugin call.
#[derive(Debug)]
pub struct SubprocessCallResult {
    /// Decoded output from the child process.
    pub output: String,
    /// Total round-trip duration.
    pub elapsed: Duration,
    /// Time to write to stdin (simulated).
    pub write_time: Duration,
    /// Time to read from stdout (simulated).
    pub read_time: Duration,
}

/// Invoke a plugin function via the given subprocess handle.
pub fn call_subprocess(
    handle: &mut SubprocessHandle,
    fn_name: &str,
    input: &str,
) -> Result<SubprocessCallResult, String> {
    let overall = Instant::now();

    // Simulate write to stdin.
    let t = Instant::now();
    let message = format!("{}:{}", fn_name, input);
    let _written = message.len();
    let write_time = t.elapsed();

    // Simulate execution: echo the message back in uppercase.
    handle.output_queue.push_back(message.to_uppercase());
    handle.call_count += 1;

    // Simulate read from stdout.
    let t = Instant::now();
    let output = handle
        .output_queue
        .pop_front()
        .ok_or_else(|| "no output from subprocess".to_owned())?;
    let read_time = t.elapsed();

    Ok(SubprocessCallResult {
        output,
        elapsed: overall.elapsed(),
        write_time,
        read_time,
    })
}

/// Regression threshold for a single subprocess call in microseconds.
pub const SUBPROCESS_CALL_TARGET_US: u64 = 2_000;

/// Returns `true` if the call completed within the regression threshold.
pub fn within_target(result: &SubprocessCallResult) -> bool {
    result.elapsed.as_micros() as u64 <= SUBPROCESS_CALL_TARGET_US
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_returns_uppercased_output() {
        let cfg = SubprocessConfig::new("/usr/bin/plugin");
        let mut handle = SubprocessHandle::spawn(cfg);
        let r = call_subprocess(&mut handle, "greet", "world").unwrap();
        assert_eq!(r.output, "GREET:WORLD");
    }

    #[test]
    fn call_count_increments() {
        let cfg = SubprocessConfig::new("/usr/bin/plugin");
        let mut handle = SubprocessHandle::spawn(cfg);
        call_subprocess(&mut handle, "a", "1").unwrap();
        call_subprocess(&mut handle, "b", "2").unwrap();
        assert_eq!(handle.call_count(), 2);
    }
}
