# Salience and Forgetting Tuning

## Salience Formula

```
score = importance_weight * importance
      + recency_weight * (1 / (1 + age_secs / 3600))
      + frequency_weight * ln(1 + access_count)
```

Default weights: importance=2.0, recency=1.0, frequency=0.5.

## Forgetting Thresholds

- `min_salience`: items scoring below this are pruned.
- `max_age_secs`: items older than this are always dropped regardless of score.

## Tuning Recommendations

- Increase `min_salience` to prune more aggressively.
- Decrease `max_age_secs` for high-churn conversations.
- Increase `importance_weight` when explicit importance signals are reliable.
