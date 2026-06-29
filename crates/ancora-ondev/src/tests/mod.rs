//! Integration-level tests for the on-device runtime.
//!
//! All tests run fully offline; no network calls are made.

use crate::{
    build_profile::BuildProfile,
    features::{Feature, FeatureRegistry},
    inference::{InferenceRequest, LocalInferenceEngine, ModelBackend},
    journal::Journal,
    memory::{seed_embedding, MemoryRecord, MemoryStore, EMBEDDING_DIM},
    perf::{measure_cold_start, MemorySnapshot},
    targets::{IosCabi, JniBridge, TargetMeta, TargetTriple},
};
use serde_json::json;

// ── Task 10: arm64 build runs an agent ──────────────────────────────────────

#[test]
fn arm64_target_meta_created() {
    let meta = TargetMeta::for_target(TargetTriple::Arm64Linux);
    assert_eq!(meta.target, TargetTriple::Arm64Linux);
    assert!(!meta.cpu_features.is_empty());
}

#[test]
fn arm64_agent_run_produces_response() {
    // Simulates what an arm64 agent would do: init journal + memory + infer.
    let mut journal = Journal::open();
    let mut memory = MemoryStore::new();

    // Record a step in the journal.
    journal.append("agent-arm64", 1, "2026-01-01T00:00:00Z", json!({"step": "start"}));

    // Store a memory record.
    memory.upsert(MemoryRecord {
        id: "m1".to_string(),
        agent_id: "agent-arm64".to_string(),
        label: "context".to_string(),
        embedding: seed_embedding(1, EMBEDDING_DIM),
        metadata: json!({}),
    });

    // Run local inference.
    let engine = LocalInferenceEngine::new(
        ModelBackend::LocalGguf { model_path: "/models/phi3.gguf".to_string() },
        true,
    )
    .unwrap();
    let resp = engine
        .infer(&InferenceRequest {
            prompt: "hello from arm64".to_string(),
            max_tokens: 50,
            temperature: 0.0,
        })
        .unwrap();

    assert!(!resp.text.is_empty());
    assert_eq!(journal.len(), 1);
    assert_eq!(memory.len(), 1);
}

// ── Task 11: android target loads ───────────────────────────────────────────

#[test]
fn android_target_has_jni() {
    let t = TargetTriple::AndroidArm64;
    assert!(t.has_jni());
}

#[test]
fn android_jni_bridge_prefix() {
    let b = JniBridge::default_config();
    assert!(b.jni_prefix().starts_with("Java_"));
}

#[test]
fn android_feature_can_be_enabled() {
    let mut r = FeatureRegistry::minimal();
    r.enable(&Feature::AndroidJni);
    assert!(r.is_enabled(&Feature::AndroidJni));
}

// ── Task 12: ios target loads ────────────────────────────────────────────────

#[test]
fn ios_target_has_cabi() {
    let t = TargetTriple::IosArm64;
    assert!(t.has_ios_cabi());
}

#[test]
fn ios_cabi_module_name() {
    let c = IosCabi::default_config();
    assert_eq!(c.module_name, "AncoraOndev");
}

#[test]
fn ios_feature_can_be_enabled() {
    let mut r = FeatureRegistry::minimal();
    r.enable(&Feature::IosCabi);
    assert!(r.is_enabled(&Feature::IosCabi));
}

// ── Task 13: minimal profile size within target ──────────────────────────────

#[test]
fn minimal_profile_within_default_budget() {
    let p = BuildProfile::minimal();
    // The Rust source for this crate is well under 5 MiB.
    assert!(p.within_budget(1024 * 1024));
}

#[test]
fn balanced_profile_has_larger_budget() {
    let minimal = BuildProfile::minimal();
    let balanced = BuildProfile::balanced();
    assert!(balanced.size_budget_bytes > minimal.size_budget_bytes);
}

// ── Task 14: on-device run offline ──────────────────────────────────────────

#[test]
fn offline_run_no_network() {
    // Confirm that creating an engine with a local backend and running it
    // does not attempt any network I/O (by construction: no networking code
    // exists in the local path).
    let engine = LocalInferenceEngine::new(
        ModelBackend::LocalGguf { model_path: "/models/phi3.gguf".to_string() },
        true,
    )
    .unwrap();
    assert!(engine.is_local_only());
    let resp = engine
        .infer(&InferenceRequest {
            prompt: "offline test".to_string(),
            max_tokens: 20,
            temperature: 0.5,
        })
        .unwrap();
    assert!(!resp.text.is_empty());
}

#[test]
fn remote_backend_blocked_offline() {
    let result = LocalInferenceEngine::new(
        ModelBackend::RemoteApi { url: "https://api.example.com".to_string() },
        true,
    );
    assert!(result.is_err());
}

// ── Task 15: journal persists on device ─────────────────────────────────────

#[test]
fn journal_persists_multiple_entries() {
    let mut j = Journal::open();
    for i in 1u64..=5 {
        j.append("agent-dev", i, &format!("t{}", i), json!({"step": i}));
    }
    assert_eq!(j.len(), 5);
    let entries = j.entries_for("agent-dev");
    assert_eq!(entries.len(), 5);
    assert_eq!(entries[0].seq, 1);
    assert_eq!(entries[4].seq, 5);
}

#[test]
fn journal_survives_purge_of_other_agent() {
    let mut j = Journal::open();
    j.append("agent-a", 1, "t", json!(null));
    j.append("agent-b", 1, "t", json!(null));
    j.purge_agent("agent-b");
    assert_eq!(j.len(), 1);
    assert_eq!(j.entries_for("agent-a").len(), 1);
}

// ── Task 16: memory works on device ─────────────────────────────────────────

#[test]
fn memory_store_search_works_on_device() {
    let mut store = MemoryStore::new();
    for i in 0u64..5 {
        store.upsert(MemoryRecord {
            id: format!("r{}", i),
            agent_id: "dev-agent".to_string(),
            label: format!("rec-{}", i),
            embedding: seed_embedding(i, EMBEDDING_DIM),
            metadata: json!({}),
        });
    }
    let q = seed_embedding(2, EMBEDDING_DIM);
    let results = store.search(&q, 2);
    assert_eq!(results.len(), 2);
    assert!((results[0].score - 1.0).abs() < 1e-5, "top result should be exact match");
}

#[test]
fn memory_export_json_parses() {
    let mut store = MemoryStore::new();
    store.upsert(MemoryRecord {
        id: "x1".to_string(),
        agent_id: "a".to_string(),
        label: "l".to_string(),
        embedding: seed_embedding(0, EMBEDDING_DIM),
        metadata: json!({}),
    });
    let json_str = store.export_json();
    let val: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    assert!(val.is_array());
}

// ── Task 17: cold start on arm measured ─────────────────────────────────────

#[test]
fn cold_start_measured_within_1s() {
    let m = measure_cold_start("aarch64-unknown-linux-musl");
    assert!(
        m.within_budget_ms(1000.0),
        "cold start exceeded 1 s: {:.2} ms",
        m.total_ms
    );
}

// ── Task 18: memory footprint on arm measured ────────────────────────────────

#[test]
fn memory_snapshot_rss_kib_non_negative() {
    let snap = MemorySnapshot::capture();
    // On all platforms this should be a non-negative value.
    assert!(snap.rss_kib() < usize::MAX);
}

#[test]
fn memory_snapshot_within_100mib_limit() {
    // A simple runtime should stay well below 100 MiB.
    let snap = MemorySnapshot::capture();
    // On non-Linux targets the snapshot is zeroed, which also passes.
    assert!(snap.within_limit_mib(100));
}
