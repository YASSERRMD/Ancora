use crate::pr_eval::{select_datasets, SelectorRule};

#[test]
fn pr_eval_selects_datasets_by_changed_files() {
    let rules = vec![
        SelectorRule::new("crates/ancora-cost", vec!["cost-bench".to_string()]),
        SelectorRule::new("crates/ancora-inference", vec!["mmlu".to_string(), "hellaswag".to_string()]),
    ];
    let always = vec!["smoke".to_string()];

    let changed = vec!["crates/ancora-inference/src/lib.rs"];
    let datasets = select_datasets(&changed, &rules, &always);

    assert!(datasets.contains(&"smoke".to_string()), "always-run dataset must be included");
    assert!(datasets.contains(&"mmlu".to_string()), "mmlu should be selected for inference changes");
    assert!(datasets.contains(&"hellaswag".to_string()), "hellaswag should be selected for inference changes");
    assert!(!datasets.contains(&"cost-bench".to_string()), "cost-bench should not run for inference changes");
}

#[test]
fn pr_eval_deduplicates_datasets() {
    let rules = vec![
        SelectorRule::new("crates/", vec!["smoke".to_string()]),
    ];
    let always = vec!["smoke".to_string()];
    let changed = vec!["crates/foo/bar.rs"];
    let datasets = select_datasets(&changed, &rules, &always);
    assert_eq!(datasets.iter().filter(|d| d.as_str() == "smoke").count(), 1);
}
