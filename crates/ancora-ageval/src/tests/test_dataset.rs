use crate::dataset::{EvalDataset, EvalSample};

#[test]
fn dataset_stores_samples() {
    let mut ds = EvalDataset::new("behavior-v1");
    ds.add(EvalSample::new("s1"));
    ds.add(EvalSample::new("s2"));
    assert_eq!(ds.len(), 2);
    assert!(!ds.is_empty());
}

#[test]
fn dataset_by_tag_filters_correctly() {
    let mut ds = EvalDataset::new("tagged");
    ds.add(EvalSample::new("s1").with_tag("planning"));
    ds.add(EvalSample::new("s2").with_tag("routing"));
    ds.add(EvalSample::new("s3").with_tag("planning"));
    let planning = ds.by_tag("planning");
    assert_eq!(planning.len(), 2);
}

#[test]
fn dataset_sample_metadata() {
    let sample = EvalSample::new("s1")
        .with_meta("version", "1.0")
        .with_meta("author", "test");
    assert_eq!(sample.metadata.get("version").unwrap(), "1.0");
}

#[test]
fn empty_dataset_is_empty() {
    let ds = EvalDataset::new("empty");
    assert!(ds.is_empty());
    assert_eq!(ds.len(), 0);
}
