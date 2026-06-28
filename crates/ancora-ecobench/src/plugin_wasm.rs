//! Plugin call overhead for WebAssembly-based plugins.
//!
//! Measures the round-trip cost of invoking a plugin function via a simulated
//! WASM boundary. No real WASM runtime is required; the module models the
//! serialisation and deserialisation steps that dominate the overhead.

use std::time::{Duration, Instant};

/// A simulated WASM module handle.
#[derive(Debug)]
pub struct WasmModule {
    name: String,
    export_count: usize,
}

impl WasmModule {
    /// Create a new simulated module.
    pub fn new(name: &str, export_count: usize) -> Self {
        Self {
            name: name.to_owned(),
            export_count,
        }
    }

    /// The module's declared name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Number of exported functions.
    pub fn export_count(&self) -> usize {
        self.export_count
    }
}

/// Payload passed across the WASM boundary.
#[derive(Debug, Clone)]
pub struct WasmPayload {
    /// Raw bytes representing the serialised input.
    pub bytes: Vec<u8>,
}

impl WasmPayload {
    /// Construct a payload from a byte slice.
    pub fn new(data: &[u8]) -> Self {
        Self {
            bytes: data.to_vec(),
        }
    }
}

/// Result of a single WASM function call.
#[derive(Debug)]
pub struct WasmCallResult {
    /// Return payload from the module.
    pub output: WasmPayload,
    /// Total round-trip duration.
    pub elapsed: Duration,
    /// Time spent serialising the input.
    pub serialize_time: Duration,
    /// Time spent deserialising the output.
    pub deserialize_time: Duration,
    /// Time spent inside the simulated WASM execution.
    pub exec_time: Duration,
}

/// Invoke a WASM export function with the given payload.
///
/// Simulates serialisation, execution, and deserialisation phases.
pub fn call_wasm(module: &WasmModule, fn_name: &str, input: WasmPayload) -> Result<WasmCallResult, String> {
    if module.export_count() == 0 {
        return Err(format!("module '{}' has no exports", module.name()));
    }

    let overall = Instant::now();

    // Serialise: simulate by computing a checksum over the input bytes.
    let t = Instant::now();
    let _checksum: u64 = input.bytes.iter().fold(0u64, |acc, &b| acc.wrapping_add(b as u64));
    let serialize_time = t.elapsed();

    // Execute: simulate by reversing the bytes.
    let t = Instant::now();
    let mut result_bytes = input.bytes.clone();
    result_bytes.reverse();
    // Append fn_name length as a sentinel byte.
    result_bytes.push(fn_name.len() as u8);
    let exec_time = t.elapsed();

    // Deserialise: simulate by building a new payload.
    let t = Instant::now();
    let output = WasmPayload::new(&result_bytes);
    let deserialize_time = t.elapsed();

    Ok(WasmCallResult {
        output,
        elapsed: overall.elapsed(),
        serialize_time,
        deserialize_time,
        exec_time,
    })
}

/// Regression threshold for a single WASM call in microseconds.
pub const WASM_CALL_TARGET_US: u64 = 1_000;

/// Returns `true` if the call completed within the regression threshold.
pub fn within_target(result: &WasmCallResult) -> bool {
    result.elapsed.as_micros() as u64 <= WASM_CALL_TARGET_US
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_succeeds_with_exports() {
        let m = WasmModule::new("test-mod", 3);
        let payload = WasmPayload::new(b"hello");
        let r = call_wasm(&m, "run", payload).unwrap();
        // Output should contain original bytes reversed plus sentinel.
        assert_eq!(r.output.bytes.last(), Some(&3u8)); // len("run") == 3
    }

    #[test]
    fn call_fails_without_exports() {
        let m = WasmModule::new("empty-mod", 0);
        let payload = WasmPayload::new(b"data");
        assert!(call_wasm(&m, "run", payload).is_err());
    }
}
