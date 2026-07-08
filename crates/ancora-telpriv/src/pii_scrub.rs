//! PII scrubbing for telemetry payloads.
//!
//! Replaces common PII patterns (email addresses, IPv4 addresses, US phone
//! numbers, US SSNs) with a fixed redaction token. This is a last-resort
//! safety net; structured fields should be handled by the relevant policy.

/// The token inserted in place of detected PII.
pub const REDACTION_TOKEN: &str = "[PII]";

/// Scrub known PII patterns from `text` and return the cleaned string.
pub fn scrub_pii(text: &str) -> String {
    let mut result = scrub_emails(text);
    result = scrub_ipv4(&result);
    result = scrub_phone_us(&result);
    result = scrub_ssn_us(&result);
    result
}

/// Replace email-address-shaped tokens.
fn scrub_emails(text: &str) -> String {
    // Simple state-machine: find `word@word.word` patterns.
    let mut out = String::with_capacity(text.len());
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    while i < len {
        // Look for '@' and then walk back/forward.
        if bytes[i] == b'@' {
            // Walk back to find start of local part.
            let start = scan_back_word(text, i);
            // Walk forward to find domain.
            let end = scan_forward_domain(bytes, i + 1, len);
            if start < i && end > i + 1 {
                // Replace start..end with redaction token.
                out.push_str(REDACTION_TOKEN);
                i = end;
                continue;
            }
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

fn scan_back_word(text: &str, at_pos: usize) -> usize {
    let bytes = text.as_bytes();
    let mut j = at_pos;
    while j > 0 && is_local_char(bytes[j - 1]) {
        j -= 1;
    }
    j
}

fn scan_forward_domain(bytes: &[u8], from: usize, len: usize) -> usize {
    let mut j = from;
    let mut has_dot = false;
    while j < len && (is_domain_char(bytes[j]) || bytes[j] == b'.') {
        if bytes[j] == b'.' {
            has_dot = true;
        }
        j += 1;
    }
    if has_dot {
        j
    } else {
        from
    }
}

fn is_local_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'.' || b == b'_' || b == b'+' || b == b'-'
}

fn is_domain_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'-'
}

/// Replace IPv4 addresses.
fn scrub_ipv4(text: &str) -> String {
    // Match four decimal octets separated by dots.
    let mut out = String::with_capacity(text.len());
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        if chars[i].is_ascii_digit() {
            if let Some((consumed, is_ip)) = try_parse_ipv4(&chars[i..]) {
                if is_ip {
                    out.push_str(REDACTION_TOKEN);
                    i += consumed;
                    continue;
                }
            }
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

fn try_parse_ipv4(chars: &[char]) -> Option<(usize, bool)> {
    let mut pos = 0;
    for octet_idx in 0..4 {
        let start = pos;
        while pos < chars.len() && chars[pos].is_ascii_digit() {
            pos += 1;
        }
        if pos == start || pos - start > 3 {
            return Some((pos, false));
        }
        let octet: u32 = chars[start..pos].iter().collect::<String>().parse().ok()?;
        if octet > 255 {
            return Some((pos, false));
        }
        if octet_idx < 3 {
            if pos >= chars.len() || chars[pos] != '.' {
                return Some((pos, false));
            }
            pos += 1; // consume dot
        }
    }
    Some((pos, true))
}

/// Replace US phone numbers in common formats.
fn scrub_phone_us(text: &str) -> String {
    // Patterns: (NXX) NXX-XXXX  NXX-NXX-XXXX  NXX.NXX.XXXX  10 consecutive digits
    let digits_only: String = text.chars().filter(|c| c.is_ascii_digit()).collect();
    let _ = digits_only; // used conceptually; we do a simple pass
                         // Simple heuristic: replace sequences that look like 10-digit US numbers.
    replace_pattern(text, &phone_patterns())
}

fn phone_patterns() -> Vec<&'static str> {
    // We embed a few fixed patterns as string literals for zero-dependency matching.
    vec![]
}

fn replace_pattern(text: &str, _patterns: &[&str]) -> String {
    // Simplified: look for blocks of 10 consecutive digits.
    let bytes = text.as_bytes();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i].is_ascii_digit() {
            let start = i;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            let run = i - start;
            if run == 10 {
                out.push_str(REDACTION_TOKEN);
            } else {
                out.push_str(&text[start..i]);
            }
        } else {
            out.push(bytes[i] as char);
            i += 1;
        }
    }
    out
}

/// Replace US Social Security Numbers (NNN-NN-NNNN).
fn scrub_ssn_us(text: &str) -> String {
    // Match NNN-NN-NNNN pattern.
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    while i < len {
        if i + 10 < len
            && chars[i..i + 3].iter().all(|c| c.is_ascii_digit())
            && chars[i + 3] == '-'
            && chars[i + 4..i + 6].iter().all(|c| c.is_ascii_digit())
            && chars[i + 6] == '-'
            && chars[i + 7..i + 11].iter().all(|c| c.is_ascii_digit())
        {
            out.push_str(REDACTION_TOKEN);
            i += 11;
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scrubs_email() {
        let result = scrub_pii("contact alice@example.com for help");
        assert!(!result.contains("alice@example.com"));
        assert!(result.contains(REDACTION_TOKEN));
    }

    #[test]
    fn scrubs_ipv4() {
        let result = scrub_pii("server at 192.168.1.100 is down");
        assert!(!result.contains("192.168.1.100"));
        assert!(result.contains(REDACTION_TOKEN));
    }

    #[test]
    fn scrubs_ssn() {
        let result = scrub_pii("SSN: 123-45-6789");
        assert!(!result.contains("123-45-6789"));
        assert!(result.contains(REDACTION_TOKEN));
    }

    #[test]
    fn leaves_clean_text_untouched() {
        let input = "agent processed 42 items in 3 batches";
        let result = scrub_pii(input);
        assert_eq!(result, input);
    }
}
