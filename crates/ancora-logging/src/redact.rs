const REDACTED: &str = "[REDACTED]";

const SECRET_FIELD_NAMES: &[&str] = &[
    "api_key", "token", "password", "secret", "credential", "private_key", "auth",
];

/// Redact secrets from a JSON string representation.
pub fn redact_json(json: &str) -> String {
    let mut out = json.to_string();
    for field in SECRET_FIELD_NAMES {
        let search = format!("\"{}\":\"", field);
        let mut offset = 0;
        while let Some(rel) = out[offset..].find(&search) {
            let start = offset + rel;
            let val_start = start + search.len();
            if out[val_start..].starts_with(REDACTED) {
                // Already redacted; skip past it.
                offset = val_start + REDACTED.len();
                continue;
            }
            if let Some(end_rel) = out[val_start..].find('"') {
                let end = val_start + end_rel;
                out.replace_range(val_start..end, REDACTED);
                offset = val_start + REDACTED.len();
            } else {
                break;
            }
        }
    }
    out
}

/// Check that none of the secret field names appear with non-redacted values.
pub fn is_clean(json: &str) -> bool {
    for field in SECRET_FIELD_NAMES {
        let search = format!("\"{}\":\"", field);
        if let Some(pos) = json.find(&search) {
            let val_start = pos + search.len();
            if !json[val_start..].starts_with(REDACTED) {
                return false;
            }
        }
    }
    true
}
