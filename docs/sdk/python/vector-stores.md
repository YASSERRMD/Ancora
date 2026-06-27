# Choosing a Vector Store (Python)

| Store | Install | Best for |
|-------|---------|----------|
| LanceDB | `pip install lancedb` | Embedded, no server, good default |
| pgvector | `pip install pgvector psycopg2` | Existing PostgreSQL infra |
| Milvus | `pip install pymilvus` | High-throughput, multi-tenant |
| Qdrant | `pip install qdrant-client` | Easy on-premise, rich filtering |

## LanceDB (embedded, recommended)

```python
import lancedb
import numpy as np

db = lancedb.connect("/var/lib/myapp/vectors")
table = db.create_table("passages", data=[
    {"vector": embed("Ancora provides durable orchestration."),
     "content": "Ancora provides durable orchestration.", "key": "doc1"},
])

results = table.search(embed("durable agent")).limit(3).to_pandas()
context = "\n".join(results["content"].tolist())
```

## pgvector

```python
import psycopg2
import json

conn = psycopg2.connect("postgresql://localhost/mydb")
cur = conn.cursor()

cur.execute(
    "SELECT content FROM passages ORDER BY embedding <=> %s::vector LIMIT 3",
    (json.dumps(embed("durable agent")),)
)
context = "\n".join(row[0] for row in cur.fetchall())
```

## Milvus

```python
from pymilvus import connections, Collection

connections.connect(host="localhost", port=19530)
col = Collection("passages")
results = col.search([embed("durable agent")], "embedding",
                     {"metric_type": "L2"}, limit=3,
                     output_fields=["content"])
context = "\n".join(hit.entity.get("content") for hit in results[0])
```

## Qdrant

```python
from qdrant_client import QdrantClient

client = QdrantClient(host="localhost", port=6333)
hits = client.search(collection_name="passages",
                     query_vector=embed("durable agent"), limit=3)
context = "\n".join(h.payload["content"] for h in hits)
```

## See also

- [Memory and RAG](memory-and-rag.md)
- [Providers](providers.md)
