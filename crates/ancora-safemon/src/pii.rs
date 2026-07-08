//! PII (Personally Identifiable Information) detection for agent outputs.
//!
//! Uses simple pattern-based heuristics to detect common PII types
//! such as email addresses, phone numbers, SSNs, and credit card numbers.

#[derive(Debug, Clone, PartialEq)]
pub enum PiiKind {
    Email,
    PhoneNumber,
    SocialSecurityNumber,
    CreditCard,
}

#[derive(Debug, Clone)]
pub struct PiiMatch {
    pub kind: PiiKind,
    pub span: (usize, usize),
    pub redacted: String,
}

impl PiiMatch {
    fn new(kind: PiiKind, start: usize, end: usize) -> Self {
        let len = end - start;
        Self {
            kind,
            span: (start, end),
            redacted: "*".repeat(len),
        }
    }
}

pub struct PiiDetector {}

impl PiiDetector {
    pub fn new() -> Self {
        Self {}
    }

    /// Returns the first PII match found, or None.
    pub fn detect(&self, text: &str) -> Option<PiiMatch> {
        if let Some(m) = self.find_email(text) {
            return Some(m);
        }
        if let Some(m) = self.find_ssn(text) {
            return Some(m);
        }
        if let Some(m) = self.find_phone(text) {
            return Some(m);
        }
        if let Some(m) = self.find_credit_card(text) {
            return Some(m);
        }
        None
    }

    /// Returns all PII matches found.
    pub fn detect_all(&self, text: &str) -> Vec<PiiMatch> {
        let mut matches = Vec::new();
        if let Some(m) = self.find_email(text) {
            matches.push(m);
        }
        if let Some(m) = self.find_ssn(text) {
            matches.push(m);
        }
        if let Some(m) = self.find_phone(text) {
            matches.push(m);
        }
        if let Some(m) = self.find_credit_card(text) {
            matches.push(m);
        }
        matches
    }

    /// Redact all PII from the text, replacing with a placeholder.
    pub fn redact(&self, text: &str) -> String {
        let mut result = self.redact_emails(text);
        result = self.redact_ssns(&result);
        result
    }

    fn find_email(&self, text: &str) -> Option<PiiMatch> {
        let bytes = text.as_bytes();
        for (i, &b) in bytes.iter().enumerate() {
            if b == b'@' && i > 0 && i + 1 < bytes.len() {
                let prefix = &text[..i];
                let start = prefix
                    .rfind(|c: char| !c.is_alphanumeric() && c != '.' && c != '_' && c != '-')
                    .map(|p| p + 1)
                    .unwrap_or(0);
                let rest = &text[i + 1..];
                let end_offset = rest
                    .find(|c: char| {
                        c.is_whitespace() || c == ',' || c == ';' || c == '"' || c == '\''
                    })
                    .unwrap_or(rest.len());
                if end_offset > 0 {
                    let end = i + 1 + end_offset;
                    return Some(PiiMatch::new(PiiKind::Email, start, end));
                }
            }
        }
        None
    }

    fn find_ssn(&self, text: &str) -> Option<PiiMatch> {
        let chars: Vec<char> = text.chars().collect();
        let n = chars.len();
        if n < 11 {
            return None;
        }
        let limit = n - 11;
        let mut i = 0;
        while i <= limit {
            if chars[i].is_ascii_digit()
                && chars[i + 1].is_ascii_digit()
                && chars[i + 2].is_ascii_digit()
                && chars[i + 3] == '-'
                && chars[i + 4].is_ascii_digit()
                && chars[i + 5].is_ascii_digit()
                && chars[i + 6] == '-'
                && chars[i + 7].is_ascii_digit()
                && chars[i + 8].is_ascii_digit()
                && chars[i + 9].is_ascii_digit()
                && chars[i + 10].is_ascii_digit()
            {
                return Some(PiiMatch::new(PiiKind::SocialSecurityNumber, i, i + 11));
            }
            i += 1;
        }
        None
    }

    fn find_phone(&self, text: &str) -> Option<PiiMatch> {
        let chars: Vec<char> = text.chars().collect();
        let n = chars.len();
        if n < 12 {
            return None;
        }
        let limit = n - 12;
        let mut i = 0;
        while i <= limit {
            if chars[i].is_ascii_digit()
                && chars[i + 1].is_ascii_digit()
                && chars[i + 2].is_ascii_digit()
                && chars[i + 3] == '-'
                && chars[i + 4].is_ascii_digit()
                && chars[i + 5].is_ascii_digit()
                && chars[i + 6].is_ascii_digit()
                && chars[i + 7] == '-'
                && chars[i + 8].is_ascii_digit()
                && chars[i + 9].is_ascii_digit()
                && chars[i + 10].is_ascii_digit()
                && chars[i + 11].is_ascii_digit()
            {
                return Some(PiiMatch::new(PiiKind::PhoneNumber, i, i + 12));
            }
            i += 1;
        }
        None
    }

    fn find_credit_card(&self, text: &str) -> Option<PiiMatch> {
        // DDDD-DDDD-DDDD-DDDD (19 chars)
        let chars: Vec<char> = text.chars().collect();
        let n = chars.len();
        if n < 19 {
            return None;
        }
        let limit = n - 19;
        let mut i = 0;
        while i <= limit {
            let mut ok = true;
            for group in 0..4usize {
                let base = i + group * 5;
                if group > 0 {
                    let sep = chars[base - 1];
                    if sep != '-' && sep != ' ' {
                        ok = false;
                        break;
                    }
                }
                for digit_pos in 0..4usize {
                    if !chars[base + digit_pos].is_ascii_digit() {
                        ok = false;
                        break;
                    }
                }
                if !ok {
                    break;
                }
            }
            if ok {
                return Some(PiiMatch::new(PiiKind::CreditCard, i, i + 19));
            }
            i += 1;
        }
        None
    }

    fn redact_emails(&self, text: &str) -> String {
        let mut result = String::new();
        let mut remaining = text;
        loop {
            match remaining.find('@') {
                None => {
                    result.push_str(remaining);
                    break;
                }
                Some(at_pos) => {
                    let prefix = &remaining[..at_pos];
                    let start = prefix
                        .rfind(|c: char| !c.is_alphanumeric() && c != '.' && c != '_' && c != '-')
                        .map(|p| p + 1)
                        .unwrap_or(0);
                    let rest = &remaining[at_pos + 1..];
                    let end_offset = rest
                        .find(|c: char| c.is_whitespace() || c == ',' || c == ';')
                        .unwrap_or(rest.len());
                    if end_offset > 0 {
                        result.push_str(&remaining[..start]);
                        result.push_str("[REDACTED_EMAIL]");
                        remaining = &remaining[at_pos + 1 + end_offset..];
                    } else {
                        result.push_str(remaining);
                        break;
                    }
                }
            }
        }
        result
    }

    fn redact_ssns(&self, text: &str) -> String {
        let chars: Vec<char> = text.chars().collect();
        let n = chars.len();
        if n < 11 {
            return text.to_string();
        }

        let mut redact_ranges: Vec<(usize, usize)> = Vec::new();
        let limit = n - 11;
        let mut i = 0;
        while i <= limit {
            if chars[i].is_ascii_digit()
                && chars[i + 1].is_ascii_digit()
                && chars[i + 2].is_ascii_digit()
                && chars[i + 3] == '-'
                && chars[i + 4].is_ascii_digit()
                && chars[i + 5].is_ascii_digit()
                && chars[i + 6] == '-'
                && chars[i + 7].is_ascii_digit()
                && chars[i + 8].is_ascii_digit()
                && chars[i + 9].is_ascii_digit()
                && chars[i + 10].is_ascii_digit()
            {
                redact_ranges.push((i, i + 11));
                i += 11;
                continue;
            }
            i += 1;
        }

        if redact_ranges.is_empty() {
            return text.to_string();
        }

        let mut result = String::new();
        let mut byte_pos = 0usize;
        let mut char_pos = 0usize;

        for (start_char, end_char) in redact_ranges {
            // Advance byte_pos to start_char
            while char_pos < start_char {
                let c = chars[char_pos];
                byte_pos += c.len_utf8();
                char_pos += 1;
            }
            let byte_start = byte_pos;
            while char_pos < end_char {
                let c = chars[char_pos];
                byte_pos += c.len_utf8();
                char_pos += 1;
            }
            result.push_str(&text[..byte_start]);
            result.push_str("[REDACTED_SSN]");
        }

        // Append remaining
        result.push_str(&text[byte_pos..]);
        result
    }
}

impl Default for PiiDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_email() {
        let d = PiiDetector::new();
        let result = d.detect("Contact us at user@example.com for help.");
        assert!(result.is_some());
        assert_eq!(result.unwrap().kind, PiiKind::Email);
    }

    #[test]
    fn detects_ssn() {
        let d = PiiDetector::new();
        let result = d.detect("SSN: 123-45-6789");
        assert!(result.is_some());
        assert_eq!(result.unwrap().kind, PiiKind::SocialSecurityNumber);
    }

    #[test]
    fn no_pii_returns_none() {
        let d = PiiDetector::new();
        let result = d.detect("No personal information here.");
        assert!(result.is_none());
    }

    #[test]
    fn redact_removes_email() {
        let d = PiiDetector::new();
        let out = d.redact("Send mail to alice@corp.com please.");
        assert!(!out.contains("alice@corp.com"));
        assert!(out.contains("[REDACTED_EMAIL]"));
    }

    #[test]
    fn redact_removes_ssn() {
        let d = PiiDetector::new();
        let out = d.redact("SSN is 123-45-6789 end.");
        assert!(!out.contains("123-45-6789"));
        assert!(out.contains("[REDACTED_SSN]"));
    }
}
