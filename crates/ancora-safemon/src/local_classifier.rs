/// Local classifier for air-gapped environments.
///
/// Runs entirely in-process with no network calls, suitable for
/// deployment in environments without internet access.
/// Uses a simple Naive Bayes-inspired term-frequency scoring approach.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LocalCategory {
    Safe,
    Pii,
    Toxic,
    PolicyViolation,
    Hallucination,
}

impl LocalCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            LocalCategory::Safe => "safe",
            LocalCategory::Pii => "pii",
            LocalCategory::Toxic => "toxic",
            LocalCategory::PolicyViolation => "policy_violation",
            LocalCategory::Hallucination => "hallucination",
        }
    }
}

#[derive(Debug, Clone)]
pub struct LocalClassification {
    pub category: LocalCategory,
    pub score: f32,
    pub offline: bool,
}

impl LocalClassification {
    pub fn is_safe(&self) -> bool {
        self.category == LocalCategory::Safe
    }
}

pub struct LocalClassifier {
    /// Maps category -> list of indicator terms with weights.
    term_weights: HashMap<LocalCategory, Vec<(String, f32)>>,
}

impl LocalClassifier {
    pub fn new() -> Self {
        let mut tw: HashMap<LocalCategory, Vec<(String, f32)>> = HashMap::new();

        tw.insert(LocalCategory::Pii, vec![
            ("@".to_string(), 0.8),
            ("ssn".to_string(), 0.9),
            ("social security".to_string(), 0.95),
            ("credit card".to_string(), 0.85),
            ("passport".to_string(), 0.7),
            ("date of birth".to_string(), 0.7),
        ]);

        tw.insert(LocalCategory::Toxic, vec![
            ("hate".to_string(), 0.8),
            ("kill".to_string(), 0.9),
            ("murder".to_string(), 0.95),
            ("idiot".to_string(), 0.6),
            ("stupid".to_string(), 0.5),
            ("abuse".to_string(), 0.85),
        ]);

        tw.insert(LocalCategory::PolicyViolation, vec![
            ("confidential".to_string(), 0.8),
            ("bypass security".to_string(), 0.95),
            ("disable authentication".to_string(), 0.9),
            ("internal only".to_string(), 0.75),
            ("trade secret".to_string(), 0.85),
        ]);

        tw.insert(LocalCategory::Hallucination, vec![
            ("definitely".to_string(), 0.5),
            ("100%".to_string(), 0.6),
            ("experts say".to_string(), 0.65),
            ("it is a fact that".to_string(), 0.7),
            ("guaranteed".to_string(), 0.55),
        ]);

        Self { term_weights: tw }
    }

    /// Classify text entirely offline. Returns the highest-scoring category.
    pub fn classify(&self, text: &str) -> LocalClassification {
        let lower = text.to_lowercase();
        let mut best_category = LocalCategory::Safe;
        let mut best_score = 0.0f32;

        for (category, terms) in &self.term_weights {
            let score: f32 = terms
                .iter()
                .filter(|(term, _)| lower.contains(term.as_str()))
                .map(|(_, weight)| weight)
                .sum::<f32>()
                .min(1.0);

            if score > best_score {
                best_score = score;
                best_category = category.clone();
            }
        }

        LocalClassification {
            category: best_category,
            score: best_score,
            offline: true,
        }
    }

    /// Returns true if text is considered safe by local classifier.
    pub fn is_safe(&self, text: &str) -> bool {
        self.classify(text).is_safe()
    }

    /// Returns all category scores above a threshold.
    pub fn score_all(&self, text: &str, threshold: f32) -> Vec<(LocalCategory, f32)> {
        let lower = text.to_lowercase();
        let mut results = Vec::new();

        for (category, terms) in &self.term_weights {
            let score: f32 = terms
                .iter()
                .filter(|(term, _)| lower.contains(term.as_str()))
                .map(|(_, weight)| weight)
                .sum::<f32>()
                .min(1.0);

            if score >= threshold {
                results.push((category.clone(), score));
            }
        }

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results
    }
}

impl Default for LocalClassifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_text_classified_safe() {
        let clf = LocalClassifier::new();
        let result = clf.classify("The weather is nice today.");
        assert_eq!(result.category, LocalCategory::Safe);
        assert!(result.offline);
    }

    #[test]
    fn pii_text_classified() {
        let clf = LocalClassifier::new();
        let result = clf.classify("My SSN is on file.");
        assert_eq!(result.category, LocalCategory::Pii);
    }

    #[test]
    fn toxic_text_classified() {
        let clf = LocalClassifier::new();
        let result = clf.classify("I hate this and want to murder it.");
        assert_eq!(result.category, LocalCategory::Toxic);
    }

    #[test]
    fn classifier_is_fully_offline() {
        let clf = LocalClassifier::new();
        let r = clf.classify("bypass security check");
        assert!(r.offline);
        assert!(!r.is_safe());
    }
}
