use crate::data_analysis::{DataAnalyzer, DataSet};

#[test]
fn data_analysis_summarise_offline() {
    let mut ds = DataSet::new(
        "monthly_revenue",
        vec![
            "month".to_string(),
            "revenue".to_string(),
            "costs".to_string(),
        ],
    );
    ds.add_row(vec![1.0, 120_000.0, 80_000.0]).unwrap();
    ds.add_row(vec![2.0, 135_000.0, 82_000.0]).unwrap();
    ds.add_row(vec![3.0, 98_000.0, 75_000.0]).unwrap();

    let stats = DataAnalyzer::summarise(&ds, "revenue").unwrap();
    assert_eq!(stats.count, 3);
    assert!((stats.mean - 117_666.666_666_666_7).abs() < 1.0);
    assert!((stats.min - 98_000.0).abs() < 1e-9);
    assert!((stats.max - 135_000.0).abs() < 1e-9);
}

#[test]
fn top_n_returns_sorted_rows() {
    let mut ds = DataSet::new("scores", vec!["id".to_string(), "score".to_string()]);
    ds.add_row(vec![1.0, 70.0]).unwrap();
    ds.add_row(vec![2.0, 90.0]).unwrap();
    ds.add_row(vec![3.0, 55.0]).unwrap();

    let top2 = DataAnalyzer::top_n(&ds, "score", 2).unwrap();
    assert_eq!(top2.len(), 2);
    assert!(
        (top2[0][1] - 90.0).abs() < 1e-9,
        "first row should be highest score"
    );
}

#[test]
fn error_on_missing_column() {
    let ds = DataSet::new("empty", vec!["a".to_string()]);
    let result = DataAnalyzer::summarise(&ds, "nonexistent");
    assert!(result.is_err());
}

#[test]
fn row_length_mismatch_returns_err() {
    let mut ds = DataSet::new("ds", vec!["x".to_string(), "y".to_string()]);
    let result = ds.add_row(vec![1.0]);
    assert!(result.is_err());
}
