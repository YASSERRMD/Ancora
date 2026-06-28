use ancora_reason::{
    CitationStore, ContradictionDetector, EvidenceStore, FactChecker, ReasoningEvent,
    ReasoningJournal, StepDecomposer, StepVerifier,
};
use ancora_ageval::ReasoningMetric;

#[test]
fn reasoning_chain_plus_citations() {
    let claims = vec![
        "Water boils at 100 degrees Celsius".to_string(),
        "NOT: Water boils at 100 degrees Celsius".to_string(),
    ];
    let mut steps = StepDecomposer::decompose(claims);
    let mut journal = ReasoningJournal::default();
    let mut evidence = EvidenceStore::new();
    let mut citations = CitationStore::new();

    for step in &steps {
        journal.record(step.index as u64, ReasoningEvent::StepAdded {
            index: step.index,
            claim: step.claim.clone(),
        });
    }

    // Verify step 0
    let result = StepVerifier::verify(&mut steps[0], |c| c == "Water boils at 100 degrees Celsius");
    assert!(result.passed);
    journal.record(0, ReasoningEvent::StepVerified { index: 0 });
    evidence.add(&steps[0].claim, "thermodynamics-101".into());
    citations.add(&steps[0].claim, "ref://thermo/boiling".into());

    // Detect contradiction between step 0 and step 1
    let contradictions = ContradictionDetector::detect(&steps);
    assert_eq!(contradictions.len(), 1);

    // Eval reasoning metric: 1/2 verified
    let score = ReasoningMetric::score(1, 2);
    assert!((score - 0.5).abs() < 1e-10);

    assert!(citations.has_citations(&steps[0].claim));
    assert_eq!(evidence.count(&steps[0].claim), 1);
}

#[test]
fn fact_check_grounds_claim_into_evidence() {
    let mut evidence = EvidenceStore::new();
    let fc = FactChecker::check("gravity exists", |_| Some("physics-db".into()));
    assert!(fc.grounded);
    evidence.add(&fc.claim, fc.source.clone());
    assert_eq!(evidence.count("gravity exists"), 1);
}
