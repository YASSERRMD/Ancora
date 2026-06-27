# rag-lancedb

Demonstrates the retrieval-augmented generation (RAG) pattern: retrieve
relevant passages from a corpus, assemble them into a context block, and
inject the context into the agent system prompt.

The retrieval is fully in-memory (no LanceDB daemon required) to keep the
example offline and dependency-free.

## Run

```bash
cd sdk/go
go run ./examples/rag-lancedb
```

## What it shows

- Building a small document corpus (stands in for a LanceDB table)
- Keyword-overlap retrieval to select top-K passages
- Injecting retrieved context into the agent system prompt
- Running the agent and draining events
