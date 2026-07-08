/// Toxicity detection for agent outputs.
///
/// Uses a keyword-based approach with severity scoring to classify
/// content as non-toxic, mildly toxic, or highly toxic.

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ToxicityLevel {
    None,
    Mild,
    Moderate,
    Severe,
}

#[derive(Debug, Clone)]
pub struct ToxicityResult {
    pub level: ToxicityLevel,
    pub score: f32,
    pub matched_terms: Vec<String>,
}

impl ToxicityResult {
    pub fn is_toxic(&self) -> bool {
        self.level > ToxicityLevel::None
    }
}

pub struct ToxicityDetector {
    severe_terms: Vec<&'static str>,
    moderate_terms: Vec<&'static str>,
    mild_terms: Vec<&'static str>,
}

impl ToxicityDetector {
    pub fn new() -> Self {
        Self {
            severe_terms: vec!["kill", "murder", "abuse", "attack", "threat"],
            moderate_terms: vec!["hate", "racist", "violent", "bully"],
            mild_terms: vec!["idiot", "stupid", "dumb", "fool", "annoying"],
        }
    }

    pub fn analyze(&self, text: &str) -> ToxicityResult {
        let lower = text.to_lowercase();
        let mut score = 0.0f32;
        let mut matched = Vec::new();

        for term in &self.severe_terms {
            if lower.contains(term) {
                score += 0.9;
                matched.push(term.to_string());
            }
        }
        for term in &self.moderate_terms {
            if lower.contains(term) {
                score += 0.5;
                matched.push(term.to_string());
            }
        }
        for term in &self.mild_terms {
            if lower.contains(term) {
                score += 0.2;
                matched.push(term.to_string());
            }
        }

        score = score.min(1.0);

        let level = if score >= 0.8 {
            ToxicityLevel::Severe
        } else if score >= 0.4 {
            ToxicityLevel::Moderate
        } else if score > 0.0 {
            ToxicityLevel::Mild
        } else {
            ToxicityLevel::None
        };

        ToxicityResult {
            level,
            score,
            matched_terms: matched,
        }
    }

    pub fn is_toxic(&self, text: &str) -> bool {
        self.analyze(text).is_toxic()
    }
}

impl Default for ToxicityDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_text_not_toxic() {
        let d = ToxicityDetector::new();
        let r = d.analyze("The quick brown fox jumps over the lazy dog.");
        assert_eq!(r.level, ToxicityLevel::None);
        assert!(!r.is_toxic());
    }

    #[test]
    fn mild_term_is_mild() {
        let d = ToxicityDetector::new();
        let r = d.analyze("That was a stupid mistake.");
        assert_eq!(r.level, ToxicityLevel::Mild);
        assert!(r.is_toxic());
    }

    #[test]
    fn severe_term_is_severe() {
        let d = ToxicityDetector::new();
        let r = d.analyze("I will kill the process if it hangs again.");
        assert_eq!(r.level, ToxicityLevel::Severe);
    }

    #[test]
    fn score_capped_at_one() {
        let d = ToxicityDetector::new();
        let r = d.analyze("kill murder abuse attack threat hate");
        assert!(r.score <= 1.0);
    }
}
