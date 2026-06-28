# Experiment Design

## Choosing variants

Every experiment needs at least one **control** variant (the existing behaviour)
and one or more **treatment** variants (the proposed changes). Keep variants
minimal - change one thing at a time to ensure clean attribution.

## Traffic weights

Weights must sum exactly to 1.0. Common splits:

| Setup | Weights |
|---|---|
| 50/50 | control=0.5, treatment=0.5 |
| 80/20 (ramp) | control=0.8, treatment=0.2 |
| Three-way | control=0.5, a=0.25, b=0.25 |

Start with a small treatment allocation (10-20 %) when the risk is unknown.

## Choosing a primary metric

Pick one metric that directly measures the outcome you care about:

- **Maximize** - higher is better (success rate, user satisfaction, throughput).
- **Minimize** - lower is better (error rate, latency, cost per call).

Avoid using the same metric as both the primary metric and a guardrail.

## Sample size

The Welch t-test in `analysis::welch_t_test` requires at least 2 observations
per variant. In practice, aim for 50-200 observations per variant before drawing
conclusions to ensure adequate statistical power.

## Guardrails

Attach guardrails to catch regressions before they affect a large fraction of
traffic. Common guardrail metrics: error rate, p99 latency, cost per request.

Set the threshold conservatively - a 2x multiplier over the control mean is a
reasonable starting point for most safety metrics.
