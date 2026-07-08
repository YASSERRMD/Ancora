use crate::model::Message;
use crate::prompt::{format_prompt, slm_system_prompt, PromptOptions, PromptStyle};

#[test]
fn test_plain_format_contains_roles() {
    let msgs = vec![
        Message::system("You are helpful."),
        Message::user("What is 2+2?"),
    ];
    let opts = PromptOptions {
        style: PromptStyle::Plain,
        ..Default::default()
    };
    let out = format_prompt(&msgs, &opts);
    assert!(
        out.contains("System:"),
        "expected 'System:' in plain format"
    );
    assert!(out.contains("User:"), "expected 'User:' in plain format");
    assert!(out.contains("Assistant:"), "expected trailing 'Assistant:'");
}

#[test]
fn test_alpaca_format_has_instruction_section() {
    let msgs = vec![Message::user("Summarise this text.")];
    let opts = PromptOptions {
        style: PromptStyle::Alpaca,
        ..Default::default()
    };
    let out = format_prompt(&msgs, &opts);
    assert!(
        out.contains("### Instruction:"),
        "expected Alpaca instruction section"
    );
    assert!(
        out.contains("### Response:"),
        "expected Alpaca response section"
    );
}

#[test]
fn test_chatml_format_uses_im_tags() {
    let msgs = vec![Message::system("Be concise."), Message::user("Hello.")];
    let opts = PromptOptions {
        style: PromptStyle::ChatML,
        ..Default::default()
    };
    let out = format_prompt(&msgs, &opts);
    assert!(
        out.contains("<|im_start|>system"),
        "expected im_start for system"
    );
    assert!(
        out.contains("<|im_start|>user"),
        "expected im_start for user"
    );
    assert!(
        out.contains("<|im_start|>assistant"),
        "expected im_start for assistant turn"
    );
}

#[test]
fn test_llama2_format_uses_inst_tags() {
    let msgs = vec![Message::user("What is Rust?")];
    let opts = PromptOptions {
        style: PromptStyle::Llama2Chat,
        ..Default::default()
    };
    let out = format_prompt(&msgs, &opts);
    assert!(out.contains("[INST]"), "expected [INST] in llama2 format");
    assert!(out.contains("[/INST]"), "expected [/INST] in llama2 format");
}

#[test]
fn test_json_suffix_appended() {
    let msgs = vec![Message::user("Extract entities.")];
    let opts = PromptOptions {
        style: PromptStyle::Plain,
        request_json: true,
        max_chars: None,
    };
    let out = format_prompt(&msgs, &opts);
    assert!(
        out.contains("valid JSON"),
        "expected JSON instruction when request_json=true"
    );
}

#[test]
fn test_max_chars_truncation() {
    let msgs = vec![Message::user(
        "This is a very long message that should be truncated.",
    )];
    let opts = PromptOptions {
        style: PromptStyle::Plain,
        request_json: false,
        max_chars: Some(20),
    };
    let out = format_prompt(&msgs, &opts);
    assert!(
        out.chars().count() <= 20,
        "output should be at most 20 chars, got {}",
        out.chars().count()
    );
}

#[test]
fn test_slm_system_prompt_includes_steps() {
    let prompt = slm_system_prompt("Translate text", &["Read the input", "Translate", "Return"]);
    assert!(
        prompt.contains("1. Read the input"),
        "expected numbered step 1"
    );
    assert!(prompt.contains("2. Translate"), "expected numbered step 2");
    assert!(prompt.contains("3. Return"), "expected numbered step 3");
}
