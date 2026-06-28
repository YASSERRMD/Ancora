# Optimization Playbook

This playbook describes concrete actions to reduce LLM spend using data from
ancora-costan.

## 1. Enable prompt caching

**When to act:** cache hit rate below 30%.

Prompt caching saves the full cost of recomputing shared prefixes (system
prompts, tool schemas, long context) on every request. A hit typically costs
10-20% of the original inference.

**Steps:**
1. Identify the longest common prefix across requests in a session.
2. Mark it as a cache-control block in the API request.
3. Monitor `CacheSavingsTracker::hit_rate()` weekly.

**Expected savings:** 20-40% of total monthly cost for chat-heavy workloads.

## 2. Route low-complexity tasks to smaller models

**When to act:** a large model (Opus, GPT-4) handles more than 50% of cost,
and a significant fraction of those requests have low output entropy
(short, factual, or templated answers).

**Steps:**
1. Add a routing step that classifies query complexity (word count, tool
   count, required reasoning depth).
2. For simple queries, route to a faster, cheaper model (Haiku, GPT-4o-mini).
3. Validate quality with an automated eval gate.

**Expected savings:** 30-60% on the routed fraction.

## 3. Compress system prompts

**When to act:** input tokens account for more than 40% of cost.

**Steps:**
1. Audit system prompts for redundancy, boilerplate, and repeated examples.
2. Move static knowledge to retrieval (RAG) rather than embedding it in every
   context window.
3. Use `CostTimeSeries::total_tokens()` to measure before/after.

**Expected savings:** 10-20% of input token cost.

## 4. Deduplicate tool calls

**When to act:** a single tool accounts for more than 20% of total cost and
its invocation count is high relative to unique queries.

**Steps:**
1. Cache tool results within a single agent turn using an in-memory map keyed
   on (tool, args).
2. Add deduplication at the orchestrator layer before dispatching tool calls.

**Expected savings:** 10-30% on tool-heavy workflows.

## 5. Batch small requests

**When to act:** many small requests (< 200 tokens) are sent individually.

**Steps:**
1. Collect requests in a short buffer window (50-200ms).
2. Concatenate them as multiple turns in a single API call.
3. Parse the response to split outputs back to callers.

**Expected savings:** reduces per-request overhead; savings depend on
provider pricing model.

## 6. Respond to anomaly alerts

When `AnomalyDetector` raises an alert (z-score > threshold):

1. Check the tenant, model, and capability breakdown at the alert timestamp.
2. Identify runaway agents (infinite loops, retry storms).
3. Apply rate limits or circuit breakers via ancora-quota.
4. Investigate and adjust the anomaly threshold if alert fatigue occurs.

## Monitoring

Track these metrics weekly:

| Metric | Target |
|---|---|
| Cache hit rate | > 40% |
| Top model cost fraction | < 60% |
| Month-over-month cost growth | < 10% unless usage grows |
| Anomaly alert count | 0 per week (investigate any alerts) |
