# Choosing a Vector Store (Java)

| Store | Maven Artifact | Best for |
|-------|--------------|----------|
| Milvus | `io.milvus:milvus-sdk-java` | High-throughput, multi-tenant |
| pgvector | `org.postgresql:postgresql` | Existing PostgreSQL infra |
| Qdrant | `io.qdrant:client` | Easy on-premise, rich filtering |
| Weaviate | `io.weaviate:client` | Hybrid search, GraphQL |

## Milvus

```java
import io.milvus.client.*;
import io.milvus.param.*;
import io.milvus.param.dml.SearchParam;

var client = new MilvusServiceClient(
    ConnectParam.newBuilder().withHost("localhost").withPort(19530).build()
);

var searchParam = SearchParam.newBuilder()
    .withCollectionName("passages")
    .withVectors(List.of(embed("durable agent")))
    .withTopK(3)
    .addOutField("content")
    .build();

var results = client.search(searchParam);
var context = results.getData().getResults().stream()
    .map(r -> r.getFieldsMap().get("content").getStringVal())
    .collect(Collectors.joining("\n"));
```

## pgvector

```java
import java.sql.*;

try (var conn = DriverManager.getConnection("jdbc:postgresql://localhost/mydb", props)) {
    var stmt = conn.prepareStatement(
        "SELECT content FROM passages ORDER BY embedding <=> ?::vector LIMIT 3");
    stmt.setObject(1, Arrays.toString(embed("durable agent")));

    var rs = stmt.executeQuery();
    var passages = new ArrayList<String>();
    while (rs.next()) passages.add(rs.getString("content"));

    var context = String.join("\n", passages);
}
```

## Qdrant

```java
import io.qdrant.client.*;
import io.qdrant.client.grpc.Points.*;

var client = new QdrantClient(QdrantGrpcClient.newBuilder("localhost", 6334, false).build());
var results = client.searchAsync(
    "passages",
    embed("durable agent"),
    3,
    null,
    true,
    null
).get();

var context = results.stream()
    .map(r -> r.getPayloadMap().get("content").getStringValue())
    .collect(Collectors.joining("\n"));
```

## See also

- [Memory and RAG](memory-and-rag.md)
- [Providers](providers.md)
