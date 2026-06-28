# Metrics and Evals Catalog Index

## Metrics

| ID | Name | Description | Stable |
| --- | --- | --- | --- |
| M001 | agent.request.latency_p99 | P99 request latency histogram | Yes |
| M002 | agent.request.count | Total request count | Yes |
| M003 | agent.error.rate | Error rate (errors/total) | Yes |
| M004 | agent.token.input_count | LLM input token count | Yes |
| M005 | agent.token.output_count | LLM output token count | Yes |
| M006 | agent.cost.usd | Estimated cost in USD | Beta |
| M007 | agent.memory.bytes | Memory usage in bytes | Yes |
| M008 | agent.cpu.utilization | CPU utilization ratio | Yes |
| M009 | eval.run.duration_ms | Eval run wall-clock duration | Yes |
| M010 | eval.score.distribution | Score distribution histogram | Beta |

## Evals

| ID | Name | Description | Stable |
| --- | --- | --- | --- |
| E001 | factual-accuracy | Factual accuracy against ground truth | Yes |
| E002 | rouge-l | ROUGE-L recall score | Yes |
| E003 | semantic-similarity | Cosine similarity of embeddings | Yes |
| E004 | hallucination-rate | Fraction of hallucinated claims | Beta |
| E005 | instruction-following | Adherence to system prompt | Yes |
| E006 | toxicity | Toxicity classifier score | Yes |
| E007 | latency-vs-quality | Pareto frontier eval | Beta |

## Dashboards

| ID | Name | Description | Stable |
| --- | --- | --- | --- |
| D001 | obs-overview | Observability system overview | Yes |
| D002 | eval-trends | Eval score trends over time | Yes |
| D003 | cost-analysis | Cost breakdown by agent and model | Beta |

## Alerts

| ID | Name | Condition | Stable |
| --- | --- | --- | --- |
| A001 | high-error-rate | error_rate > 0.05 for 5m | Yes |
| A002 | latency-budget-exceeded | p99 > 500ms for 10m | Yes |
| A003 | eval-score-drop | score < baseline - 0.1 | Beta |

Last updated: 2026-06-29
