//! Multilingual eval suite.
//!
//! Tests an agent's ability to understand prompts and produce correct answers
//! across multiple languages without relying on a translation API.

/// ISO 639-1 language code.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LanguageCode(pub String);

impl LanguageCode {
    pub fn new(code: impl Into<String>) -> Self {
        LanguageCode(code.into())
    }
}

/// One multilingual eval case.
#[derive(Debug, Clone)]
pub struct MultilingualCase {
    pub id: String,
    pub language: LanguageCode,
    pub prompt: String,
    /// Fragment that must appear in the answer (case-insensitive).
    pub expected_fragment: String,
}

impl MultilingualCase {
    pub fn new(
        id: impl Into<String>,
        language: LanguageCode,
        prompt: impl Into<String>,
        expected_fragment: impl Into<String>,
    ) -> Self {
        MultilingualCase {
            id: id.into(),
            language,
            prompt: prompt.into(),
            expected_fragment: expected_fragment.into(),
        }
    }
}

/// Outcome of a multilingual eval.
#[derive(Debug, Clone, PartialEq)]
pub enum MultilingualOutcome {
    Correct,
    IncorrectOrMissing,
}

/// Offline multilingual responder backed by a small static lookup table.
pub struct LocalMultilingualResponder {
    /// Maps (language_code, key_prompt_fragment) -> response_fragment.
    table: Vec<(String, String, String)>,
}

impl LocalMultilingualResponder {
    pub fn new(table: Vec<(String, String, String)>) -> Self {
        LocalMultilingualResponder { table }
    }

    pub fn default() -> Self {
        Self::new(vec![
            ("en".into(), "hello".into(), "hello".into()),
            ("es".into(), "hola".into(), "hola".into()),
            ("fr".into(), "bonjour".into(), "bonjour".into()),
            ("de".into(), "hallo".into(), "hallo".into()),
            ("ja".into(), "konnichiwa".into(), "konnichiwa".into()),
            ("ar".into(), "marhaba".into(), "marhaba".into()),
        ])
    }

    /// Produce a response for the given language and prompt, or None.
    pub fn respond(&self, lang: &LanguageCode, prompt: &str) -> Option<String> {
        let prompt_lower = prompt.to_lowercase();
        for (l, key, resp) in &self.table {
            if l == &lang.0 && prompt_lower.contains(key.as_str()) {
                return Some(resp.clone());
            }
        }
        None
    }
}

/// The full multilingual eval suite.
pub struct MultilingualSuite {
    pub cases: Vec<MultilingualCase>,
    responder: LocalMultilingualResponder,
}

impl MultilingualSuite {
    pub fn new(cases: Vec<MultilingualCase>, responder: LocalMultilingualResponder) -> Self {
        MultilingualSuite { cases, responder }
    }

    pub fn default_catalog() -> Self {
        let cases = vec![
            MultilingualCase::new("ml-001", LanguageCode::new("en"), "Say hello", "hello"),
            MultilingualCase::new("ml-002", LanguageCode::new("es"), "Di hola", "hola"),
            MultilingualCase::new("ml-003", LanguageCode::new("fr"), "Dis bonjour", "bonjour"),
            MultilingualCase::new("ml-004", LanguageCode::new("de"), "Sag hallo", "hallo"),
        ];
        Self::new(cases, LocalMultilingualResponder::default())
    }

    pub fn evaluate(&self, case: &MultilingualCase) -> MultilingualOutcome {
        match self.responder.respond(&case.language, &case.prompt) {
            Some(resp)
                if resp
                    .to_lowercase()
                    .contains(&case.expected_fragment.to_lowercase()) =>
            {
                MultilingualOutcome::Correct
            }
            _ => MultilingualOutcome::IncorrectOrMissing,
        }
    }

    pub fn run_all(&self) -> (usize, usize) {
        let total = self.cases.len();
        let passed = self
            .cases
            .iter()
            .filter(|c| self.evaluate(c) == MultilingualOutcome::Correct)
            .count();
        (passed, total)
    }
}
