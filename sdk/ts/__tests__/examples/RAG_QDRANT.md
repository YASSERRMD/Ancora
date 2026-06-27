# rag-qdrant

Demonstrates the RAG pattern: retrieve relevant passages from an in-memory
corpus (standing in for a Qdrant collection) and inject them into the agent
system prompt before the run.
Runs fully offline -- no Qdrant daemon required.

## Test

```bash
cd sdk/ts
npx jest __tests__/examples/rag-qdrant-example
```

## What it shows

- Keyword-overlap retrieval over a small document corpus
- Injecting retrieved passages as `instructions` in `buildSpec`
- Verifying the agent run starts and completes with the context
