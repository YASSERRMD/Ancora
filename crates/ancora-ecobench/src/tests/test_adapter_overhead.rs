//! Tests: adapter overhead within target thresholds.

use crate::adapter_overhead::{
    adapt_tools, within_target, AncoraToolDef, ADAPT_PER_TOOL_TARGET_US,
};

#[test]
fn adapter_overhead_within_target_single_tool() {
    let defs = vec![AncoraToolDef::new("search", "Search documents")
        .with_param("query", "string")
        .with_param("limit", "integer")];
    let r = adapt_tools(&defs);
    assert!(
        within_target(&r, 1),
        "adapter overhead {} us/tool exceeds {} us threshold",
        r.elapsed.as_micros(),
        ADAPT_PER_TOOL_TARGET_US
    );
}

#[test]
fn adapter_overhead_within_target_many_tools() {
    let defs: Vec<AncoraToolDef> = (0..20)
        .map(|i| {
            AncoraToolDef::new(&format!("tool-{}", i), "Benchmark tool")
                .with_param("input", "string")
        })
        .collect();
    let n = defs.len();
    let r = adapt_tools(&defs);
    assert_eq!(r.tools.len(), 20);
    assert!(
        within_target(&r, n),
        "adapter overhead {} us for {} tools",
        r.elapsed.as_micros(),
        n
    );
}

#[test]
fn adapter_preserves_tool_names() {
    let defs = vec![
        AncoraToolDef::new("alpha", "First tool"),
        AncoraToolDef::new("beta", "Second tool"),
    ];
    let r = adapt_tools(&defs);
    let names: Vec<&str> = r.tools.iter().map(|t| t.name.as_str()).collect();
    assert!(names.contains(&"alpha"));
    assert!(names.contains(&"beta"));
}
