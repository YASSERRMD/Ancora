# Choosing a Vector Store (TypeScript)

| Store | Install | Best for |
|-------|---------|----------|
| LanceDB (`vectordb`) | `npm install vectordb` | Embedded, no server, good default |
| pgvector | `npm install pg` | Existing PostgreSQL infra |
| Milvus | `npm install @zilliz/milvus2-sdk-node` | High-throughput, multi-tenant |
| Qdrant | `npm install @qdrant/js-client-rest` | Easy on-premise, rich filtering |

## LanceDB (embedded, recommended)

```ts
import { connect } from 'vectordb'
import type { Table } from 'vectordb'

const db = await connect('/var/lib/myapp/vectors')
const table = await db.openTable('passages')

const results = await table.search(embed('durable agent')).limit(3).execute()
const context = results.map(r => r.content as string).join('\n')
```

## pgvector

```ts
import { Client } from 'pg'

const client = new Client({ connectionString: 'postgresql://localhost/mydb' })
await client.connect()

const vector = JSON.stringify(embed('durable agent'))
const { rows } = await client.query(
  'SELECT content FROM passages ORDER BY embedding <=> $1::vector LIMIT 3',
  [vector]
)
const context = rows.map(r => r.content).join('\n')
```

## Milvus

```ts
import { MilvusClient } from '@zilliz/milvus2-sdk-node'

const client = new MilvusClient({ address: 'localhost:19530' })
const results = await client.search({
  collection_name: 'passages',
  vector: embed('durable agent'),
  limit: 3,
  output_fields: ['content'],
})
const context = results.results.map(r => r.content).join('\n')
```

## Qdrant

```ts
import { QdrantClient } from '@qdrant/js-client-rest'

const client = new QdrantClient({ host: 'localhost', port: 6333 })
const results = await client.search('passages', {
  vector: embed('durable agent'),
  limit: 3,
  with_payload: true,
})
const context = results.map(r => (r.payload as { content: string }).content).join('\n')
```

## See also

- [Memory and RAG](memory-and-rag.md)
- [Providers](providers.md)
