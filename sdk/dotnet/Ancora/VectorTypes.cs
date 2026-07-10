using System.Collections.Generic;
using System.Text.Json;

namespace Ancora;

/// <summary>
/// Points a <see cref="Runtime"/> at a real Postgres + pgvector instance for
/// document embeddings instead of the zero-dependency in-memory vector store
/// used by default.
/// </summary>
/// <param name="PgvectorUrl">
/// A standard <c>postgres://</c> connection string. The <c>vector</c>
/// extension must be installed (or installable by the connecting role) on
/// the target database.
/// </param>
public sealed record MemoryConfig(string PgvectorUrl);

/// <summary>
/// A point to upsert into a vector collection.
/// </summary>
/// <param name="Id">
/// Non-negative point id. Required to be numeric (not a UUID string) because
/// the pgvector-backed store uses a <c>BIGINT PRIMARY KEY</c>; keeping the
/// wire format numeric-only means the same request works unmodified against
/// the in-memory default store too.
/// </param>
/// <param name="Vector">The embedding vector.</param>
/// <param name="Payload">
/// Optional metadata attached to the point. Values must be JSON-serializable
/// primitives (string, number, bool, or null).
/// </param>
public sealed record VectorPoint(
    ulong Id,
    float[] Vector,
    IReadOnlyDictionary<string, object?>? Payload = null
);

/// <summary>
/// A point returned by <see cref="Runtime.Query"/>, annotated with its
/// similarity score.
/// </summary>
/// <param name="Id">The point's id.</param>
/// <param name="Score">
/// Similarity score; higher is always more similar, regardless of the
/// underlying distance metric the collection was created with.
/// </param>
/// <param name="Payload">
/// The point's stored metadata. Values are raw <see cref="JsonElement"/>s
/// since a collection's payload shape is caller-defined.
/// </param>
public sealed record ScoredVectorPoint(
    ulong Id,
    float Score,
    IReadOnlyDictionary<string, JsonElement> Payload
);
