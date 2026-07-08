use crate::dataset::{Dataset, Example};

#[test]
fn test_dataset_loads_examples_from_csv() {
    let csv = "q1,What is 2+2?,4\nq2,Capital of France?,Paris";
    let dataset = Dataset::from_csv("math", "1.0.0", csv).expect("parse should succeed");
    assert_eq!(dataset.len(), 2);
    assert_eq!(dataset.examples[0].id, "q1");
    assert_eq!(dataset.examples[0].expected, "4");
    assert_eq!(dataset.examples[1].id, "q2");
    assert_eq!(dataset.examples[1].expected, "Paris");
}

#[test]
fn test_empty_dataset_is_empty() {
    let dataset = Dataset::new("empty", "0.1.0");
    assert!(dataset.is_empty());
    assert_eq!(dataset.len(), 0);
}

#[test]
fn test_add_example() {
    let mut dataset = Dataset::new("test", "1.0.0");
    dataset.add(Example::new("e1", "Hello?", "World"));
    assert_eq!(dataset.len(), 1);
}

#[test]
fn test_example_with_metadata() {
    let ex = Example::new("e1", "input", "output")
        .with_metadata("source", "manual")
        .with_metadata("difficulty", "easy");
    assert_eq!(
        ex.metadata.get("source").map(|s| s.as_str()),
        Some("manual")
    );
    assert_eq!(
        ex.metadata.get("difficulty").map(|s| s.as_str()),
        Some("easy")
    );
}

#[test]
fn test_csv_skips_comments_and_blank_lines() {
    let csv = "# comment\n\nq1,input,expected\n";
    let dataset = Dataset::from_csv("d", "1.0.0", csv).unwrap();
    assert_eq!(dataset.len(), 1);
}
