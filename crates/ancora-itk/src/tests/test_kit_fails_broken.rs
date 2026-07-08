/// Tests that the kits correctly detect broken/non-conforming adapters.
use crate::provider_kit::{Provider, ProviderKit};
use crate::tool_kit::{Tool, ToolKit, ToolSchema};
use crate::vectorstore_kit::{VecDoc, VectorStore, VectorStoreKit};
use std::collections::HashMap;

// Broken provider: name is empty, models is empty, complete always errors.
struct BrokenProvider;

impl Provider for BrokenProvider {
    fn name(&self) -> &str {
        ""
    }

    fn models(&self) -> Vec<String> {
        vec![]
    }

    fn complete(&self, _prompt: &str) -> Result<String, String> {
        Err("not implemented".into())
    }
}

#[test]
fn provider_kit_fails_broken_provider() {
    let kit = ProviderKit::new();
    let results = kit.run(&BrokenProvider);
    let failures: Vec<_> = results.iter().filter(|r| !r.passed).collect();
    assert!(
        !failures.is_empty(),
        "Expected at least one failure for broken provider"
    );
}

// Broken vector store: search always returns empty.
struct BrokenVectorStore;

impl VectorStore for BrokenVectorStore {
    fn name(&self) -> &str {
        "broken-store"
    }

    fn upsert(&mut self, _doc: VecDoc) -> Result<(), String> {
        Ok(()) // silently drops the doc
    }

    fn search(&self, _query: &[f32], _top_k: usize) -> Result<Vec<VecDoc>, String> {
        Ok(vec![]) // always empty - should fail the upsert/search check
    }
}

#[test]
fn vectorstore_kit_fails_broken_store() {
    let kit = VectorStoreKit::new();
    let mut store = BrokenVectorStore;
    let results = kit.run(&mut store);
    let failures: Vec<_> = results.iter().filter(|r| !r.passed).collect();
    assert!(
        !failures.is_empty(),
        "Expected at least one failure for broken vector store"
    );
}

// Broken tool: no input fields and call always errors.
struct BrokenTool;

impl Tool for BrokenTool {
    fn name(&self) -> &str {
        ""
    }

    fn description(&self) -> &str {
        ""
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            input_fields: vec![],
            output_fields: vec![],
        }
    }

    fn call(&self, _args: HashMap<String, String>) -> Result<HashMap<String, String>, String> {
        Err("broken".into())
    }
}

#[test]
fn tool_kit_fails_broken_tool() {
    let kit = ToolKit::new();
    let results = kit.run(&BrokenTool);
    let failures: Vec<_> = results.iter().filter(|r| !r.passed).collect();
    assert!(
        !failures.is_empty(),
        "Expected at least one failure for broken tool"
    );
}
