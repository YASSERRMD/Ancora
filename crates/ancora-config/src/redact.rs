use crate::schema::AncoraCfg;
use serde_json::Value;

const REDACTED: &str = "[REDACTED]";

/// Return a JSON representation of the config with all secret refs replaced.
/// Secret refs are identified by field names containing "ref" or "key".
pub fn redacted_dump(cfg: &AncoraCfg) -> Value {
    let mut v = serde_json::to_value(cfg).unwrap_or(Value::Null);
    redact_recursive(&mut v);
    v
}

fn redact_recursive(v: &mut Value) {
    match v {
        Value::Object(map) => {
            for (k, val) in map.iter_mut() {
                let k_lower = k.to_lowercase();
                if k_lower.contains("ref") || k_lower.contains("key") || k_lower.contains("secret") {
                    if !val.is_null() {
                        *val = Value::String(REDACTED.into());
                    }
                } else {
                    redact_recursive(val);
                }
            }
        }
        Value::Array(arr) => {
            for item in arr.iter_mut() {
                redact_recursive(item);
            }
        }
        _ => {}
    }
}
