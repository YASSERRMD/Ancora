use serde_json::Value;
use crate::error::StructuredError;

/// Extracts structured JSON from a model response string.
/// Tries direct parse first, then looks for the first JSON object in the string.
pub struct JsonExtractor;

impl JsonExtractor {
    pub fn extract(text: &str) -> Result<Value, StructuredError> {
        let trimmed = text.trim();

        if let Ok(v) = serde_json::from_str::<Value>(trimmed) {
            return Ok(v);
        }

        if let Some(start) = trimmed.find('{') {
            if let Some(end) = Self::find_matching_brace(trimmed, start) {
                let json_str = &trimmed[start..=end];
                if let Ok(v) = serde_json::from_str::<Value>(json_str) {
                    return Ok(v);
                }
            }
        }

        Err(StructuredError::ExtractionFailed)
    }

    fn find_matching_brace(s: &str, start: usize) -> Option<usize> {
        let bytes = s.as_bytes();
        let mut depth = 0i32;
        let mut in_string = false;
        let mut escape = false;
        for (i, &b) in bytes.iter().enumerate().skip(start) {
            if escape { escape = false; continue; }
            if b == b'\\' && in_string { escape = true; continue; }
            if b == b'"' { in_string = !in_string; continue; }
            if in_string { continue; }
            if b == b'{' { depth += 1; }
            if b == b'}' {
                depth -= 1;
                if depth == 0 { return Some(i); }
            }
        }
        None
    }
}
