# rag_memory

Demonstrates retrieval-augmented generation: tools retrieve and summarize
documents, results are stored in a `MemoryStore`, and the agent runs with
context injected from the store.
Runs fully offline.

## Run

```bash
cd sdk/python
python -m examples.rag_memory
```

## What it shows

- Defining `@tool` functions for retrieval and summarization
- Storing retrieved context in a `MemoryStore` before the run
- Reading the response back from memory after the run
