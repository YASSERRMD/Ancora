# Interpreting Eval Run Results

This guide explains how to read and act on the metrics produced by `ancora-evalrun`.

## Pass Rate and Confidence Intervals

The primary metric is **pass rate**: the fraction of rollouts that produced a passing output.

A 95% Wilson score confidence interval is computed automatically. Use the CI to assess whether differences between runs are statistically meaningful:
- If the CIs of two runs overlap, the difference may not be significant.
- If they do not overlap, the difference is likely real.

Example:
- Run A: pass_rate=0.72 CI=[0.62, 0.80]
- Run B: pass_rate=0.85 CI=[0.77, 0.91]
- These CIs do not overlap - Run B is significantly better.

## pass@k and pass-power-k

**pass@k**: probability that at least one of k sampled rollouts passes. High pass@k means the model can solve the problem when given multiple attempts.

**pass-power-k (pass^k)**: probability that all k rollouts pass. High pass^k means the model is consistently correct, not just occasionally lucky.

Use pass^k to identify cases where the model is unreliable even if it sometimes succeeds.

## Per-Case Breakdown

The breakdown table shows each case individually. Cases with low pass rates are worth investigating:
- Check the failure reasons for patterns.
- Use failure clustering to group similar failures.

## Failure Clustering

Clustered failures reveal systemic issues. If 80% of failures share the same error pattern, fixing that one issue will have large impact.

## Cost and Latency

- **p50/p95 latency**: typical and worst-case response times. Use p95 to assess SLA compliance.
- **Total tokens**: total inference cost for the run. Use budget checks to gate expensive runs.
- **Mean cost per rollout**: helps estimate the cost of future runs at different scales.

## Comparing Runs

Use `compare_runs` to quantify the delta between two eval runs:
- A positive pass_rate_delta indicates improvement.
- Deltas smaller than 1 percentage point are reported as Neutral (not significant).
- Cost and latency deltas help assess whether a quality improvement came at a price.

## Acting on Results

| Situation | Action |
|-----------|--------|
| pass_rate < 0.5 | Investigate top failure clusters; fix prompt or model |
| CI overlaps between runs | Run more rollouts before concluding a change helped |
| pass^k << pass@k | Model is inconsistent; consider majority voting |
| p95 latency > SLA | Profile inference path; consider caching |
| Cost budget exceeded | Reduce rollout count or use a smaller model for cheap cases |
