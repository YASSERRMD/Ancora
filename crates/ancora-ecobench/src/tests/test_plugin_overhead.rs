//! Tests: plugin overhead within target thresholds.

use crate::plugin_load::{load_plugin, within_target as load_within_target, LOAD_TARGET_US};
use crate::plugin_wasm::{call_wasm, WasmModule, WasmPayload, within_target as wasm_within_target};
use crate::plugin_subprocess::{
    call_subprocess, SubprocessConfig, SubprocessHandle,
    within_target as subprocess_within_target,
};

#[test]
fn plugin_load_within_target() {
    let r = load_plugin("bench-plugin", "1.0.0", &["call"]);
    // The simulated load must be faster than the documented threshold.
    assert!(
        load_within_target(&r),
        "load took {:?} which exceeds {} us",
        r.elapsed,
        LOAD_TARGET_US
    );
}

#[test]
fn wasm_call_within_target() {
    let m = WasmModule::new("bench-wasm", 2);
    let payload = WasmPayload::new(b"benchmark-input");
    let r = call_wasm(&m, "execute", payload).expect("wasm call should succeed");
    assert!(
        wasm_within_target(&r),
        "wasm call took {:?}",
        r.elapsed
    );
}

#[test]
fn subprocess_call_within_target() {
    let cfg = SubprocessConfig::new("/usr/bin/bench-plugin");
    let mut handle = SubprocessHandle::spawn(cfg);
    let r = call_subprocess(&mut handle, "run", "data").expect("subprocess call should succeed");
    assert!(
        subprocess_within_target(&r),
        "subprocess call took {:?}",
        r.elapsed
    );
}
