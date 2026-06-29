//! Tool-call repair for weaker models.
//!
//! Small models often produce malformed tool-call JSON:
//! - Trailing commas
//! - Single-quoted strings
//! - Missing closing braces
//! - Wrong field names (`function_name` vs `name`)
//! - Extra prose wrapping the JSON

use serde_json::Value;

/// Errors that can occur during tool-call repair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepairError {
    /// The text contains no recognisable JSON object.
    NoJsonFound,
    /// The extracted JSON is structurally invalid even after repair attempts.
    InvalidStructure(String),
}

impl std::fmt::Display for RepairError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoJsonFound => write!(f, "no JSON object found in model output"),
            Self::InvalidStructure(s) => write!(f, "invalid tool-call structure: {}", s),
        }
    }
}

/// A repaired, normalised tool call ready for dispatch.
#[derive(Debug, Clone, PartialEq)]
pub struct ToolCall {
    /// Tool / function name.
    pub name: String,
    /// Parsed arguments as a JSON object.
    pub arguments: Value,
}

/// Attempt to extract and repair a tool call from raw model output.
///
/// Strategy (applied in order):
/// 1. Try parsing the raw text directly as JSON.
/// 2. Extract the first `{…}` substring and parse that.
/// 3. Apply lightweight heuristic fixes (trailing commas, single quotes).
/// 4. Normalise the key names (`function_name` → `name`, `args` → `arguments`).
/// 5. Validate that `name` and `arguments` are present.
pub fn repair_tool_call(raw: &str) -> Result<ToolCall, RepairError> {
    // Step 1: direct parse.
    if let Ok(v) = serde_json::from_str::<Value>(raw.trim()) {
        return normalise(v);
    }

    // Step 2: extract first {...} block.
    let extracted = extract_json_object(raw).ok_or(RepairError::NoJsonFound)?;

    // Step 3: heuristic fixes.
    let fixed = apply_fixes(&extracted);

    // Step 4+5: parse and normalise.
    let v: Value = serde_json::from_str(&fixed)
        .map_err(|e| RepairError::InvalidStructure(e.to_string()))?;
    normalise(v)
}

/// Extract the first balanced `{…}` substring from `text`.
fn extract_json_object(text: &str) -> Option<String> {
    let start = text.find('{')?;
    let mut depth: i32 = 0;
    let mut end = start;
    let chars: Vec<char> = text[start..].chars().collect();
    let mut in_string = false;
    let mut escape = false;
    for (i, &ch) in chars.iter().enumerate() {
        if escape {
            escape = false;
            continue;
        }
        if ch == '\\' && in_string {
            escape = true;
            continue;
        }
        if ch == '"' {
            in_string = !in_string;
            continue;
        }
        if in_string {
            continue;
        }
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    end = start + i + 1;
                    break;
                }
            }
            _ => {}
        }
    }
    if depth != 0 {
        return None;
    }
    Some(text[start..end].to_string())
}

/// Apply lightweight heuristic fixes to a JSON string.
fn apply_fixes(s: &str) -> String {
    // Replace single-quoted strings with double-quoted ones (very naive).
    let s = replace_single_quotes(s);
    // Remove trailing commas before } or ].
    let s = remove_trailing_commas(&s);
    s
}

fn replace_single_quotes(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_double = false;
    let mut prev = '\0';
    for ch in s.chars() {
        if ch == '"' && prev != '\\' {
            in_double = !in_double;
        }
        if ch == '\'' && !in_double {
            out.push('"');
        } else {
            out.push(ch);
        }
        prev = ch;
    }
    out
}

fn remove_trailing_commas(s: &str) -> String {
    // Remove commas that are followed (ignoring whitespace) by } or ].
    let mut result = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    while i < len {
        let ch = bytes[i] as char;
        if ch == ',' {
            // Peek ahead, skipping whitespace.
            let mut j = i + 1;
            while j < len && (bytes[j] as char).is_whitespace() {
                j += 1;
            }
            if j < len && (bytes[j] as char == '}' || bytes[j] as char == ']') {
                // Skip this comma.
                i += 1;
                continue;
            }
        }
        result.push(ch);
        i += 1;
    }
    result
}

/// Normalise field names in a parsed JSON value and return a [`ToolCall`].
fn normalise(mut v: Value) -> Result<ToolCall, RepairError> {
    let obj = v.as_object_mut().ok_or_else(|| {
        RepairError::InvalidStructure("top-level value is not an object".into())
    })?;

    // Normalise `function_name` / `fn` → `name`.
    if !obj.contains_key("name") {
        if let Some(val) = obj.remove("function_name").or_else(|| obj.remove("fn")) {
            obj.insert("name".into(), val);
        }
    }

    // Normalise `args` / `params` / `parameters` → `arguments`.
    if !obj.contains_key("arguments") {
        if let Some(val) = obj
            .remove("args")
            .or_else(|| obj.remove("params"))
            .or_else(|| obj.remove("parameters"))
        {
            obj.insert("arguments".into(), val);
        }
    }

    let name = obj
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| RepairError::InvalidStructure("missing 'name' field".into()))?
        .to_string();

    let arguments = obj
        .get("arguments")
        .cloned()
        .unwrap_or(Value::Object(serde_json::Map::new()));

    Ok(ToolCall { name, arguments })
}
