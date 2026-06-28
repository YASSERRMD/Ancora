/// Hallucination heuristic detection for agent outputs.
///
/// Uses confidence-marker analysis, hedging language, and statistical
/// improbability signals to flag potential hallucinations.

#[derive(Debug, Clone, PartialEq)]
pub enum HallucinationSignal {
    OverconfidentFact,
    ConflictingStatement,
    UnverifiableClaim,
    HedgingWithFalsePrecision,
}

#[derive(Debug, Clone)]
pub struct HallucinationFlag {
    pub signal: HallucinationSignal,
    pub excerpt: String,
    pub confidence: f32,
}

pub struct HallucinationDetector {
    overconfidence_phrases: Vec<&'static str>,
    hedging_with_precision: Vec<&'static str>,
    unverifiable_markers: Vec<&'static str>,
}

impl HallucinationDetector {
    pub fn new() -> Self {
        Self {
            overconfidence_phrases: vec![
                "definitely",
                "100%",
                "always true",
                "guaranteed",
                "it is a fact that",
                "studies show that",
                "research proves",
            ],
            hedging_with_precision: vec![
                "approximately 97.3",
                "roughly 84.7",
                "about 62.1",
                "around 43.8",
            ],
            unverifiable_markers: vec![
                "according to unnamed sources",
                "experts say",
                "it is widely believed",
                "some people think",
                "scientists have discovered",
            ],
        }
    }

    /// Returns true if the text exhibits hallucination signals.
    pub fn suspect(&self, text: &str) -> bool {
        !self.analyze(text).is_empty()
    }

    /// Returns all detected hallucination flags.
    pub fn analyze(&self, text: &str) -> Vec<HallucinationFlag> {
        let lower = text.to_lowercase();
        let mut flags = Vec::new();

        for phrase in &self.overconfidence_phrases {
            if lower.contains(phrase) {
                flags.push(HallucinationFlag {
                    signal: HallucinationSignal::OverconfidentFact,
                    excerpt: phrase.to_string(),
                    confidence: 0.7,
                });
            }
        }

        for phrase in &self.hedging_with_precision {
            if lower.contains(phrase) {
                flags.push(HallucinationFlag {
                    signal: HallucinationSignal::HedgingWithFalsePrecision,
                    excerpt: phrase.to_string(),
                    confidence: 0.8,
                });
            }
        }

        for phrase in &self.unverifiable_markers {
            if lower.contains(phrase) {
                flags.push(HallucinationFlag {
                    signal: HallucinationSignal::UnverifiableClaim,
                    excerpt: phrase.to_string(),
                    confidence: 0.6,
                });
            }
        }

        // Conflicting statements heuristic: look for "but" after "definitely"
        if lower.contains("definitely") && lower.contains("but actually") {
            flags.push(HallucinationFlag {
                signal: HallucinationSignal::ConflictingStatement,
                excerpt: "definitely ... but actually".to_string(),
                confidence: 0.75,
            });
        }

        flags
    }
}

impl Default for HallucinationDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_text_no_flags() {
        let d = HallucinationDetector::new();
        assert!(!d.suspect("The capital of France is Paris."));
    }

    #[test]
    fn overconfident_phrase_flagged() {
        let d = HallucinationDetector::new();
        let flags = d.analyze("It is definitely true that the moon is made of cheese.");
        assert!(!flags.is_empty());
        assert_eq!(flags[0].signal, HallucinationSignal::OverconfidentFact);
    }

    #[test]
    fn unverifiable_claim_detected() {
        let d = HallucinationDetector::new();
        let flags = d.analyze("According to unnamed sources, the project was cancelled.");
        assert!(!flags.is_empty());
        assert_eq!(flags[0].signal, HallucinationSignal::UnverifiableClaim);
    }

    #[test]
    fn multiple_signals_detected() {
        let d = HallucinationDetector::new();
        let text = "Studies show that it is 100% guaranteed according to unnamed sources.";
        let flags = d.analyze(text);
        assert!(flags.len() >= 2);
    }
}
