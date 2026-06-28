//! Example: verified reasoning chain with citations, evidence, and abstention.

use ancora_reason::{
    AbstentionPolicy, CitationStore, ContradictionDetector, EvidenceStore, FactChecker,
    ReasoningEvent, ReasoningJournal, StepDecomposer, StepVerifier,
};

pub fn run() {
    let claims = vec![
        "Water boils at 100 degrees Celsius at sea level".to_string(),
        "The speed of light is approximately 299792 km/s".to_string(),
        "NOT: The speed of light is approximately 299792 km/s".to_string(),
    ];

    let mut steps = StepDecomposer::decompose(claims);
    let mut journal = ReasoningJournal::default();
    let mut evidence = EvidenceStore::new();
    let mut citations = CitationStore::new();

    for step in &steps {
        journal.record(
            step.index as u64,
            ReasoningEvent::StepAdded {
                index: step.index,
                claim: step.claim.clone(),
            },
        );
    }

    let known_facts = vec![
        "Water boils at 100 degrees Celsius at sea level",
        "The speed of light is approximately 299792 km/s",
    ];
    let knowledge_base: Vec<&str> = known_facts.clone();

    for step in steps.iter_mut().take(2) {
        let claim = step.claim.clone();
        let found = knowledge_base.iter().any(|f| *f == claim.as_str());
        let result = StepVerifier::verify(step, |_| found);
        if result.passed {
            journal.record(step.index as u64, ReasoningEvent::StepVerified { index: step.index });
            evidence.add(&claim, "knowledge-base".into());
            citations.add(&claim, format!("fact-db://entry/{}", step.index));
            journal.record(
                step.index as u64,
                ReasoningEvent::CitationAdded {
                    claim: claim.clone(),
                    citation: format!("fact-db://entry/{}", step.index),
                },
            );
        } else {
            journal.record(step.index as u64, ReasoningEvent::StepRefuted { index: step.index });
        }
    }

    let fc = FactChecker::check(&steps[0].claim, |claim| {
        if knowledge_base.iter().any(|f| *f == claim) {
            Some("verified-facts-db".into())
        } else {
            None
        }
    });
    journal.record(
        0,
        ReasoningEvent::FactChecked {
            claim: fc.claim.clone(),
            grounded: fc.grounded,
        },
    );

    let contradictions = ContradictionDetector::detect(&steps);
    for (a, b) in &contradictions {
        journal.record(0, ReasoningEvent::ContradictionFound { a: *a, b: *b });
    }

    let weak_claim = "unverifiable assertion".to_string();
    let mut weak_steps = StepDecomposer::decompose(vec![weak_claim.clone()]);
    let policy = AbstentionPolicy::new(0.7);
    let abstained = policy.apply(&mut weak_steps[0], &[0.2, 0.3]);
    if abstained {
        journal.record(
            99,
            ReasoningEvent::StepAbstained { index: weak_steps[0].index },
        );
    }

    println!("Reasoning trace ({} events):", journal.events().len());
    for event in journal.replay() {
        println!("  {:?}", event);
    }
    println!("Contradictions found: {}", contradictions.len());
    println!("Citations for step 0: {:?}", citations.get(&steps[0].claim));
    println!("Evidence for step 0: {} source(s)", evidence.count(&steps[0].claim));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verified_reasoning_example_runs() {
        run();
    }
}
