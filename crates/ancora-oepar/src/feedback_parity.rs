//! Feedback parity - validates that human/automated feedback is captured uniformly across SDKs.

use std::collections::HashMap;

/// Feedback signal type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeedbackKind {
    Thumbs,
    Rating,
    Correction,
    Flag,
}

impl FeedbackKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            FeedbackKind::Thumbs => "thumbs",
            FeedbackKind::Rating => "rating",
            FeedbackKind::Correction => "correction",
            FeedbackKind::Flag => "flag",
        }
    }
}

/// A single feedback event.
#[derive(Debug, Clone)]
pub struct FeedbackEvent {
    pub run_id: String,
    pub span_id: String,
    pub kind: FeedbackKind,
    pub value: f64,
    pub comment: Option<String>,
    pub language: String,
}

impl FeedbackEvent {
    pub fn new(
        run_id: impl Into<String>,
        span_id: impl Into<String>,
        kind: FeedbackKind,
        value: f64,
        language: impl Into<String>,
    ) -> Self {
        Self {
            run_id: run_id.into(),
            span_id: span_id.into(),
            kind,
            value,
            comment: None,
            language: language.into(),
        }
    }

    pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }

    /// Required attributes that must be present in the serialized form.
    pub fn required_fields() -> &'static [&'static str] {
        &["run_id", "span_id", "kind", "value", "language"]
    }

    pub fn to_attributes(&self) -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("run_id".to_string(), self.run_id.clone());
        m.insert("span_id".to_string(), self.span_id.clone());
        m.insert("kind".to_string(), self.kind.as_str().to_string());
        m.insert("value".to_string(), self.value.to_string());
        m.insert("language".to_string(), self.language.clone());
        if let Some(ref c) = self.comment {
            m.insert("comment".to_string(), c.clone());
        }
        m
    }
}

/// Validate that a feedback event has all required fields.
pub fn validate_feedback_event(event: &FeedbackEvent) -> Vec<String> {
    let mut errors = Vec::new();
    if event.run_id.is_empty() {
        errors.push("run_id is empty".to_string());
    }
    if event.span_id.is_empty() {
        errors.push("span_id is empty".to_string());
    }
    if event.language.is_empty() {
        errors.push("language is empty".to_string());
    }
    if !(0.0..=1.0).contains(&event.value) && event.kind == FeedbackKind::Thumbs {
        errors.push(format!("thumbs value out of range: {}", event.value));
    }
    errors
}

/// Build a reference feedback event for parity testing.
pub fn reference_feedback(language: impl Into<String>) -> FeedbackEvent {
    FeedbackEvent::new("run-001", "span-root", FeedbackKind::Thumbs, 1.0, language)
        .with_comment("Great answer!")
}

/// Check parity of feedback events across languages.
pub fn check_feedback_parity(events: &[FeedbackEvent]) -> Vec<String> {
    let mut issues = Vec::new();
    if let Some(first) = events.first() {
        for other in events.iter().skip(1) {
            if first.kind != other.kind {
                issues.push(format!(
                    "feedback kind mismatch: {:?} vs {:?} (languages: {:?} vs {:?})",
                    first.kind.as_str(),
                    other.kind.as_str(),
                    first.language,
                    other.language
                ));
            }
            if (first.value - other.value).abs() > 1e-9 {
                issues.push(format!(
                    "feedback value mismatch: {:?} vs {:?} (languages: {:?} vs {:?})",
                    first.value, other.value, first.language, other.language
                ));
            }
        }
    }
    issues
}
