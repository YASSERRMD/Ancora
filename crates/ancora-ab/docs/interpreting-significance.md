# Interpreting Significance

## What the p-value means

The p-value from `analysis::welch_t_test` is the probability of observing a
difference as large as or larger than the one you measured, *if the two variants
actually perform identically*. It does NOT measure the size or importance of
the effect.

A p-value below your significance threshold (alpha, typically 0.05) means the
result is statistically significant - the observed difference is unlikely to be
due to chance alone.

## Common thresholds

| alpha | Interpretation |
|---|---|
| 0.10 | Lenient - acceptable for low-risk changes in early exploration |
| 0.05 | Standard - recommended default for most experiments |
| 0.01 | Conservative - use when the cost of a false positive is high |

## Reading the SignificanceResult

```
SignificanceResult {
    control_variant: "control",
    treatment_variant: "treatment",
    mean_difference: 0.15,   // treatment mean - control mean
    p_value: 0.0023,         // probability under the null hypothesis
    is_significant: true,    // p_value < alpha
    alpha: 0.05,
}
```

`mean_difference > 0` means treatment is better for a Maximize metric.
`mean_difference < 0` means treatment is better for a Minimize metric.

## When not to ship the winner

Statistical significance does not guarantee practical significance. Also check:

- Is the effect size large enough to justify the operational change?
- Did any guardrail trigger during the experiment?
- Is the result consistent across different user segments?
- Was the experiment running long enough to capture weekly patterns?

## Inconclusive results

If the experiment concludes without a significant result, record the winner as
`None` and keep the current control behaviour. Collect more data or revisit
the experiment design before re-running.
