use crate::hallucination::HallucinationDetector;
/// Safety classifier hook that runs on agent outputs.
///
/// The `SafetyClassifier` composes all sub-classifiers and returns
/// a aggregated `ClassificationReport` for a given text output.
use crate::pii::PiiDetector;
use crate::policy_violation::PolicyViolationDetector;
use crate::toxicity::ToxicityDetector;

#[derive(Debug, Clone, PartialEq)]
pub enum SafetyLevel {
    Safe,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct ClassificationReport {
    pub level: SafetyLevel,
    pub pii_detected: bool,
    pub toxic: bool,
    pub policy_violation: bool,
    pub hallucination_suspected: bool,
    pub reasons: Vec<String>,
}

impl ClassificationReport {
    pub fn is_safe(&self) -> bool {
        self.level == SafetyLevel::Safe
    }

    pub fn summary(&self) -> String {
        if self.reasons.is_empty() {
            "No safety issues detected.".to_string()
        } else {
            self.reasons.join("; ")
        }
    }
}

pub struct SafetyClassifier {
    pii: PiiDetector,
    toxicity: ToxicityDetector,
    policy: PolicyViolationDetector,
    hallucination: HallucinationDetector,
}

impl SafetyClassifier {
    pub fn new() -> Self {
        Self {
            pii: PiiDetector::new(),
            toxicity: ToxicityDetector::new(),
            policy: PolicyViolationDetector::new(),
            hallucination: HallucinationDetector::new(),
        }
    }

    pub fn classify(&self, text: &str) -> ClassificationReport {
        let mut reasons = Vec::new();
        let mut score = 0u32;

        let pii_detected = self.pii.detect(text).is_some();
        if pii_detected {
            reasons.push("PII detected".to_string());
            score += 3;
        }

        let toxic = self.toxicity.is_toxic(text);
        if toxic {
            reasons.push("Toxic content detected".to_string());
            score += 4;
        }

        let policy_violation = self.policy.check(text).is_some();
        if policy_violation {
            reasons.push("Policy violation detected".to_string());
            score += 3;
        }

        let hallucination_suspected = self.hallucination.suspect(text);
        if hallucination_suspected {
            reasons.push("Potential hallucination detected".to_string());
            score += 1;
        }

        let level = match score {
            0 => SafetyLevel::Safe,
            1..=2 => SafetyLevel::Low,
            3..=4 => SafetyLevel::Medium,
            5..=6 => SafetyLevel::High,
            _ => SafetyLevel::Critical,
        };

        ClassificationReport {
            level,
            pii_detected,
            toxic,
            policy_violation,
            hallucination_suspected,
            reasons,
        }
    }
}

impl Default for SafetyClassifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_text_is_safe() {
        let clf = SafetyClassifier::new();
        let report = clf.classify("The weather today is pleasant.");
        assert_eq!(report.level, SafetyLevel::Safe);
        assert!(!report.pii_detected);
        assert!(!report.toxic);
    }

    #[test]
    fn toxic_text_is_flagged() {
        let clf = SafetyClassifier::new();
        let report = clf.classify("You are an idiot and I hate you!");
        assert!(report.toxic);
        assert_ne!(report.level, SafetyLevel::Safe);
    }
}
