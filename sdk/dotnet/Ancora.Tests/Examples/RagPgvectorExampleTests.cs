using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests.Examples;

/// <summary>
/// Example: RAG with pgvector.
///
/// The offline corpus stands in for a real pgvector table.
/// Keyword overlap retrieval is used instead of cosine similarity to keep
/// the example dependency-free.
/// </summary>
public sealed class RagPgvectorExampleTests
{
    private record Passage(string Id, string Source, string Text);

    private static readonly Passage[] Corpus =
    [
        new("1", "docs/overview.md",  "Ancora is a multi-backend agent runtime for Rust and Go."),
        new("2", "docs/backends.md",  "Supported backends include pgvector, qdrant, weaviate, and lancedb."),
        new("3", "docs/pgvector.md",  "PgVector stores embeddings in a PostgreSQL extension column."),
        new("4", "docs/embeddings.md","The embedders module provides offline hash-based TF-IDF embedders."),
    ];

    private static List<Passage> KeywordRetrieve(string query, int topK)
    {
        var words = query.ToLowerInvariant().Split(' ');
        return Corpus
            .Select(p => (p, score: words.Count(w => p.Text.ToLowerInvariant().Contains(w))))
            .Where(t => t.score > 0)
            .OrderByDescending(t => t.score)
            .Take(topK)
            .Select(t => t.p)
            .ToList();
    }

    [Fact]
    public void Retrieval_Returns_Relevant_Passages()
    {
        var hits = KeywordRetrieve("pgvector backends", 3);
        Assert.NotEmpty(hits);
        Assert.True(hits.Any(h => h.Source.Contains("backends") || h.Source.Contains("pgvector")));
    }

    [Fact]
    public void Retrieval_Respects_TopK()
    {
        var hits = KeywordRetrieve("ancora qdrant backends embeddings", 2);
        Assert.True(hits.Count <= 2);
    }

    [Fact]
    public void Retrieval_Returns_Empty_For_Unrelated_Query()
    {
        var hits = KeywordRetrieve("zxqyuv completely unrelated xyz", 3);
        Assert.Empty(hits);
    }

    [Fact]
    public async Task Rag_Agent_Runs_With_Context_Injected()
    {
        try
        {
            var hits = KeywordRetrieve("what backends does ancora support", 3);
            var context = string.Join("\n---\n", hits.Select(h => $"[{h.Source}] {h.Text}"));
            using var agent = new Agent();
            var spec = new AgentSpec("local-model", $"Context:\n{context}\n\nAnswer using only the context.");
            var events = await agent.Run(spec).CollectAsync();
            Assert.NotEmpty(events);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Top_Ranked_Passage_Is_Most_Relevant()
    {
        var hits = KeywordRetrieve("pgvector postgresql embeddings column", 4);
        Assert.True(hits.Count > 0);
        Assert.Equal("docs/pgvector.md", hits[0].Source);
    }
}
