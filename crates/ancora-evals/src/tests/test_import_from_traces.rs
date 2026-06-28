use crate::dataset::{Dataset, Example};

/// Simulate importing examples from a trace log.
fn import_from_trace_log(trace_lines: &str, dataset_name: &str, version: &str) -> Dataset {
    let mut dataset = Dataset::new(dataset_name, version);
    for line in trace_lines.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // Trace format: "TRACE|id|input|expected"
        let parts: Vec<&str> = line.splitn(4, '|').collect();
        if parts.len() == 4 && parts[0] == "TRACE" {
            dataset.add(Example::new(parts[1], parts[2], parts[3]));
        }
    }
    dataset
}

#[test]
fn test_import_builds_dataset() {
    let traces = "TRACE|t1|What is Rust?|A systems programming language\nTRACE|t2|What is Cargo?|The Rust package manager";
    let dataset = import_from_trace_log(traces, "from-traces", "1.0.0");
    assert_eq!(dataset.len(), 2);
    assert_eq!(dataset.examples[0].id, "t1");
    assert_eq!(dataset.examples[1].expected, "The Rust package manager");
}

#[test]
fn test_import_skips_non_trace_lines() {
    let traces = "INFO|something\nTRACE|t1|in|out\nDEBUG|other";
    let dataset = import_from_trace_log(traces, "d", "1.0.0");
    assert_eq!(dataset.len(), 1);
}

#[test]
fn test_import_empty_trace() {
    let dataset = import_from_trace_log("", "d", "1.0.0");
    assert!(dataset.is_empty());
}
