# Air-Gapped Safety Classifiers

ancora-safemon includes a `LocalClassifier` designed for deployment in air-gapped
or network-restricted environments.

## Overview

The `LocalClassifier` runs entirely in-process with zero network calls. It uses
a term-frequency scoring approach with pre-defined category weights to classify
text into safety categories.

## Categories

- Safe - no safety signals detected
- Pii - personally identifiable information indicators
- Toxic - harmful or abusive language
- PolicyViolation - restricted or prohibited content
- Hallucination - overconfident or unverifiable claims

## Usage

```rust
use ancora_safemon::local_classifier::LocalClassifier;

let clf = LocalClassifier::new();

// Classify a single text
let result = clf.classify("Some agent output text");
assert!(result.offline); // always true for LocalClassifier

if !result.is_safe() {
    println!("Category: {}", result.category.as_str());
    println!("Score: {:.2}", result.score);
}

// Get all categories above a threshold
let hits = clf.score_all("Some text with mixed signals", 0.4);
for (category, score) in hits {
    println!("{}: {:.2}", category.as_str(), score);
}
```

## Deployment

### No Network Requirements

The LocalClassifier has zero network dependencies. It uses only Rust std and
carries its classification vocabulary in-binary.

### Performance

Classification is O(n*m) where n is text length and m is vocabulary size.
For typical agent outputs (< 10 KB), classification completes in microseconds.

### Customization

The default vocabulary covers common safety signals. For domain-specific
deployments, instantiate a custom classifier by extending the term weights.

## Limitations

- Pattern-based approach has lower recall than ML-based classifiers
- No learning from new examples without a recompile
- English-language vocabulary only in the default configuration
- High-precision mode: prefers fewer false positives over recall

For higher recall in connected environments, combine with the full
`SafetyClassifier` which can call remote APIs.
