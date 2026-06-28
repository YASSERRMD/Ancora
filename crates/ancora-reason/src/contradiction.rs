//! Contradiction detection: identify conflicting claims across reasoning steps.
//!
//! A claim prefixed with "NOT: " contradicts an otherwise identical claim
//! that lacks the prefix.

use crate::decompose::ReasoningStep;

pub struct ContradictionDetector;

impl ContradictionDetector {
    pub fn detect(steps: &[ReasoningStep]) -> Vec<(usize, usize)> {
        let mut pairs = Vec::new();
        for i in 0..steps.len() {
            for j in (i + 1)..steps.len() {
                if contradicts(&steps[i].claim, &steps[j].claim) {
                    pairs.push((i, j));
                }
            }
        }
        pairs
    }
}

fn contradicts(a: &str, b: &str) -> bool {
    const NOT: &str = "NOT: ";
    if let Some(rest) = a.strip_prefix(NOT) {
        return rest == b;
    }
    if let Some(rest) = b.strip_prefix(NOT) {
        return rest == a;
    }
    false
}
