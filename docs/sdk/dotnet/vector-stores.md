# Choosing a Vector Store (.NET)

| Store | NuGet Package | Best for |
|-------|--------------|----------|
| pgvector | `Npgsql` | Existing PostgreSQL infra |
| Milvus | `Milvus.Client` | High-throughput, multi-tenant |
| Qdrant | `Qdrant.Client` | Easy on-premise, rich filtering |
| Azure AI Search | `Azure.Search.Documents` | Azure-hosted deployments |

## pgvector

```csharp
using Npgsql;

await using var conn = new NpgsqlConnection("Host=localhost;Database=mydb;Username=postgres");
await conn.OpenAsync();

var vector = Embed("durable agent");   // your embedding function

await using var cmd = new NpgsqlCommand(
    "SELECT content FROM passages ORDER BY embedding <=> $1::vector LIMIT 3", conn);
cmd.Parameters.AddWithValue(vector);

await using var reader = await cmd.ExecuteReaderAsync();
var passages = new List<string>();
while (await reader.ReadAsync())
    passages.Add(reader.GetString(0));

var context = string.Join("\n", passages);
```

## Qdrant

```csharp
using Qdrant.Client;
using Qdrant.Client.Grpc;

var client = new QdrantClient("localhost");
var results = await client.SearchAsync(
    collectionName: "passages",
    vector: Embed("durable agent"),
    limit: 3,
    withPayload: true
);

var context = string.Join("\n", results.Select(r => r.Payload["content"].StringValue));
```

## Milvus

```csharp
using Milvus.Client;

var client = new MilvusClient("localhost");
var collection = client.GetCollection("passages");

var results = await collection.SearchAsync(
    vectorFieldName: "embedding",
    vectors: new[] { Embed("durable agent") },
    limit: 3,
    outputFields: new[] { "content" }
);

var context = string.Join("\n", results.Results.Select(r => r["content"].ToString()!));
```

## See also

- [Memory and RAG](memory-and-rag.md)
- [Providers](providers.md)
