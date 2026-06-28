# Recipe Catalog

The `ancora-recipe` crate ships the following built-in recipes.

| ID | Name | Steps | Description |
|----|------|-------|-------------|
| `rag-citations` | RAG with Citations | 2 | Retrieve passages and generate an answer with inline citations. |
| `research-report` | Research and Report | 4 | Multi-step research outline, evidence retrieval, drafting, and review. |
| `code-review` | Code Review | 4 | Parse, lint, security-check, and summarize source files. |
| `data-extraction` | Data Extraction | 4 | Preprocess, extract, validate, and serialize structured fields. |
| `customer-support` | Customer Support | 4 | Classify, retrieve KB, respond, and optionally escalate tickets. |
| `multi-agent-debate` | Multi-Agent Debate | 4+ | Run N-round debate among M agents and produce a consensus verdict. |
| `document-processing` | Document Processing | 4 | Ingest, chunk, enrich, and index documents. |

All recipes accept a `ParamSet` and run entirely offline.
