# Evidence and Citation Handling

## Evidence Store

`EvidenceStore` tracks supporting sources for each claim string:

```rust
let mut store = EvidenceStore::new();
store.add("the sky is blue", "reference-1".into());
store.add("the sky is blue", "reference-2".into());
assert_eq!(store.count("the sky is blue"), 2);
```

Evidence is keyed by the exact claim string. Keys are case-sensitive.

## Citation Store

`CitationStore` stores formatted citation references for claims:

```rust
let mut citations = CitationStore::new();
citations.add("claim-text", "https://example.org/fact/42".into());
assert!(citations.has_citations("claim-text"));
```

Use `all_cited_claims()` to enumerate every claim that has at least one citation.

## Fact Checking

`FactChecker::check` grounds a claim against a tool function that returns
`Option<String>`. A `Some` response marks the claim as grounded with its source:

```rust
let result = FactChecker::check("water boils at 100C", |claim| {
    db.lookup(claim) // returns Option<String>
});
if result.grounded {
    store.add(&result.claim, result.source.clone());
}
```

## Reasoning Journal

All evidence, citations, and verification decisions should be journaled for
deterministic replay:

```rust
journal.record(tick, ReasoningEvent::FactChecked {
    claim: result.claim.clone(),
    grounded: result.grounded,
});
journal.record(tick, ReasoningEvent::CitationAdded {
    claim: result.claim.clone(),
    citation: result.source.clone(),
});
```

`ReasoningJournal::replay()` returns events in insertion order, enabling
deterministic re-execution from any saved trace.
