use crate::feedback_e2e::{feedback_to_eval_entries, FeedbackItem, FeedbackEvalDataset};
use std::collections::HashMap;

fn make_inputs_outputs() -> (HashMap<String, String>, HashMap<String, String>) {
    let mut inputs = HashMap::new();
    let mut outputs = HashMap::new();
    for i in 0..5 {
        let run_id = format!("run-{}", i);
        inputs.insert(run_id.clone(), format!("input-{}", i));
        outputs.insert(run_id.clone(), format!("output-{}", i));
    }
    (inputs, outputs)
}

#[test]
fn feedback_feeds_an_eval_dataset() {
    let (inputs, outputs) = make_inputs_outputs();
    let feedback = vec![
        FeedbackItem::new("run-0", 1).with_label("positive"),
        FeedbackItem::new("run-1", -1).with_label("negative"),
        FeedbackItem::new("run-2", 1),
    ];

    let dataset = feedback_to_eval_entries(&feedback, &inputs, &outputs);

    assert_eq!(dataset.len(), 3);
    assert!(!dataset.is_empty());
}

#[test]
fn feedback_label_is_inferred_from_rating() {
    let (inputs, outputs) = make_inputs_outputs();
    let feedback = vec![
        FeedbackItem::new("run-0", 1),
        FeedbackItem::new("run-1", -1),
    ];

    let dataset = feedback_to_eval_entries(&feedback, &inputs, &outputs);

    let pos = dataset.filter_by_label("positive");
    let neg = dataset.filter_by_label("negative");
    assert_eq!(pos.len(), 1);
    assert_eq!(neg.len(), 1);
}

#[test]
fn feedback_for_unknown_run_is_skipped() {
    let inputs = HashMap::new();
    let outputs = HashMap::new();
    let feedback = vec![FeedbackItem::new("unknown-run", 1)];

    let dataset = feedback_to_eval_entries(&feedback, &inputs, &outputs);
    assert!(dataset.is_empty());
}

#[test]
fn feedback_item_positive_negative_helpers() {
    let pos = FeedbackItem::new("r", 1);
    let neg = FeedbackItem::new("r", -1);
    let neutral = FeedbackItem::new("r", 0);

    assert!(pos.is_positive());
    assert!(!pos.is_negative());
    assert!(neg.is_negative());
    assert!(!neg.is_positive());
    assert!(!neutral.is_positive());
    assert!(!neutral.is_negative());
}
