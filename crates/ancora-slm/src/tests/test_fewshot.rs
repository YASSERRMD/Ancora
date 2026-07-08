use crate::fewshot::{format_few_shot_block, inject_few_shots, FewShotExample, FewShotLibrary};

fn make_lib() -> FewShotLibrary {
    let mut lib = FewShotLibrary::new();
    lib.add(FewShotExample::new(
        "ner",
        "Sentence: 'Alice works at ACME.'",
        r#"{"entities": ["Alice", "ACME"]}"#,
        0.95,
    ));
    lib.add(FewShotExample::new(
        "ner",
        "Sentence: 'Bob is in London.'",
        r#"{"entities": ["Bob", "London"]}"#,
        0.80,
    ));
    lib.add(FewShotExample::new(
        "ner",
        "Sentence: 'No entities here.'",
        r#"{"entities": []}"#,
        0.70,
    ));
    lib.add(FewShotExample::new("qa", "Q: What is 2+2?", "A: 4", 1.0));
    lib
}

#[test]
fn test_library_stores_examples() {
    let lib = make_lib();
    assert_eq!(lib.len(), 4);
}

#[test]
fn test_retrieve_filters_by_tag() {
    let lib = make_lib();
    let examples = lib.retrieve("ner", 10);
    assert_eq!(examples.len(), 3, "should retrieve only NER examples");
    assert!(examples.iter().all(|e| e.task_tag == "ner"));
}

#[test]
fn test_retrieve_sorts_by_quality_descending() {
    let lib = make_lib();
    let examples = lib.retrieve("ner", 3);
    assert!(
        examples[0].quality >= examples[1].quality,
        "examples should be sorted by quality descending"
    );
}

#[test]
fn test_retrieve_respects_max_n() {
    let lib = make_lib();
    let examples = lib.retrieve("ner", 2);
    assert_eq!(examples.len(), 2, "should return at most n examples");
}

#[test]
fn test_inject_few_shots_prepends_examples() {
    let lib = make_lib();
    let prompt = "Sentence: 'Carol runs StartupCo.'";
    let augmented = inject_few_shots(prompt, &lib, "ner", 2);
    assert!(
        augmented.contains("Example 1:"),
        "augmented prompt should contain example header"
    );
    assert!(
        augmented.contains(prompt),
        "original prompt should be preserved"
    );
}

#[test]
fn test_inject_few_shots_no_examples_returns_original() {
    let lib = make_lib();
    let prompt = "Translate: hello";
    let augmented = inject_few_shots(prompt, &lib, "translation", 3);
    assert_eq!(
        augmented, prompt,
        "should return original prompt when no examples found"
    );
}

#[test]
fn test_format_few_shot_block_numbered() {
    let lib = make_lib();
    let examples = lib.retrieve("ner", 2);
    let block = format_few_shot_block(&examples);
    assert!(block.contains("Example 1:"), "should include 'Example 1:'");
    assert!(block.contains("Example 2:"), "should include 'Example 2:'");
}

#[test]
fn test_format_few_shot_block_empty() {
    let block = format_few_shot_block(&[]);
    assert!(
        block.is_empty(),
        "empty examples should produce empty block"
    );
}
