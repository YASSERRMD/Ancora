//! SLM-friendly prompt formatting.
//!
//! Small models respond better to:
//! - Short, directive system prompts.
//! - Clear section separators (e.g., `### Instruction`, `### Response`).
//! - Explicit output format instructions embedded in the prompt.
//! - Numbered steps rather than free-form prose.

use crate::model::Message;

/// Style of prompt template to apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptStyle {
    /// Plain chat format (works for most instruction-tuned SLMs).
    Plain,
    /// Alpaca-style format (`### Instruction: / ### Response:`).
    Alpaca,
    /// ChatML format (`<|im_start|>user\n...<|im_end|>`).
    ChatML,
    /// Llama-2 chat format (`[INST] ... [/INST]`).
    Llama2Chat,
}

/// Options controlling prompt construction.
#[derive(Debug, Clone)]
pub struct PromptOptions {
    /// Which template style to use.
    pub style: PromptStyle,
    /// If set, appended to the system message to request JSON output.
    pub request_json: bool,
    /// Maximum number of characters for the formatted prompt.
    /// Excess history turns are truncated from the left.
    pub max_chars: Option<usize>,
}

impl Default for PromptOptions {
    fn default() -> Self {
        Self { style: PromptStyle::Plain, request_json: false, max_chars: None }
    }
}

/// Format a list of [`Message`]s into a single prompt string suitable for an SLM.
///
/// The function applies the chosen [`PromptStyle`], optionally appends a JSON
/// output instruction, and truncates from the left if the result exceeds
/// `max_chars`.
pub fn format_prompt(messages: &[Message], opts: &PromptOptions) -> String {
    let json_suffix = if opts.request_json {
        "\nRespond with valid JSON only. Do not include any prose before or after the JSON object."
    } else {
        ""
    };

    let raw = match opts.style {
        PromptStyle::Plain => format_plain(messages, json_suffix),
        PromptStyle::Alpaca => format_alpaca(messages, json_suffix),
        PromptStyle::ChatML => format_chatml(messages, json_suffix),
        PromptStyle::Llama2Chat => format_llama2(messages, json_suffix),
    };

    if let Some(max) = opts.max_chars {
        truncate_left(&raw, max)
    } else {
        raw
    }
}

fn format_plain(messages: &[Message], json_suffix: &str) -> String {
    use crate::model::Role;
    let mut out = String::new();
    for msg in messages {
        let role_str = match msg.role {
            Role::System => "System",
            Role::User => "User",
            Role::Assistant => "Assistant",
        };
        out.push_str(&format!("{}: {}\n", role_str, msg.content));
    }
    if !json_suffix.is_empty() {
        out.push_str(json_suffix);
        out.push('\n');
    }
    out.push_str("Assistant:");
    out
}

fn format_alpaca(messages: &[Message], json_suffix: &str) -> String {
    use crate::model::Role;
    let mut sys = String::new();
    let mut user = String::new();
    for msg in messages {
        match msg.role {
            Role::System => {
                sys.push_str(&msg.content);
                sys.push('\n');
            }
            Role::User => {
                user.push_str(&msg.content);
                user.push('\n');
            }
            Role::Assistant => {}
        }
    }
    let mut out = String::new();
    if !sys.is_empty() {
        out.push_str("### System:\n");
        out.push_str(&sys);
    }
    out.push_str("### Instruction:\n");
    out.push_str(&user);
    if !json_suffix.is_empty() {
        out.push_str(json_suffix);
        out.push('\n');
    }
    out.push_str("### Response:\n");
    out
}

fn format_chatml(messages: &[Message], json_suffix: &str) -> String {
    use crate::model::Role;
    let mut out = String::new();
    for msg in messages {
        let role_str = match msg.role {
            Role::System => "system",
            Role::User => "user",
            Role::Assistant => "assistant",
        };
        out.push_str(&format!(
            "<|im_start|>{}\n{}<|im_end|>\n",
            role_str, msg.content
        ));
    }
    if !json_suffix.is_empty() {
        out.push_str("<|im_start|>user\n");
        out.push_str(json_suffix);
        out.push_str("<|im_end|>\n");
    }
    out.push_str("<|im_start|>assistant\n");
    out
}

fn format_llama2(messages: &[Message], json_suffix: &str) -> String {
    use crate::model::Role;
    let mut out = String::new();
    let mut first_user = true;
    for msg in messages {
        match msg.role {
            Role::System => {
                out.push_str(&format!("<<SYS>>\n{}\n<</SYS>>\n\n", msg.content));
            }
            Role::User => {
                if first_user {
                    out.push_str(&format!("[INST] {} [/INST]", msg.content));
                    first_user = false;
                } else {
                    out.push_str(&format!("\n[INST] {} [/INST]", msg.content));
                }
            }
            Role::Assistant => {
                out.push_str(&format!(" {} </s>", msg.content));
            }
        }
    }
    if !json_suffix.is_empty() {
        out.push_str(&format!("\n[INST] {} [/INST]", json_suffix));
    }
    out
}

/// Truncate a string from the left so it fits within `max_chars`.
fn truncate_left(s: &str, max_chars: usize) -> String {
    let total = s.chars().count();
    if total <= max_chars {
        return s.to_string();
    }
    let skip = total - max_chars;
    s.chars().skip(skip).collect()
}

/// Build a compact system prompt for SLMs that lists the available steps.
/// This keeps the prompt short and directive, which helps small models follow
/// the structure.
pub fn slm_system_prompt(task_description: &str, steps: &[&str]) -> String {
    let mut out = format!("You are a helpful assistant. Task: {}\n\n", task_description);
    if !steps.is_empty() {
        out.push_str("Follow these steps in order:\n");
        for (i, step) in steps.iter().enumerate() {
            out.push_str(&format!("{}. {}\n", i + 1, step));
        }
    }
    out.push_str("\nBe concise and precise.");
    out
}
