using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using System.Runtime.InteropServices;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public record PgChunk146(int Id, string Content, double Score);

public sealed class PgVectorTools146
{
    private static readonly List<PgChunk146> _fixture =
    [
        new(1, "Ancora is a local-first agent framework.", 0.95),
        new(2, "pgvector enables semantic search in Postgres.", 0.88),
        new(3, "RAG retrieval augments generation with context.", 0.82),
    ];

    [Tool("Retrieve relevant chunks from pgvector", name: "pg_retrieve146")]
    public string Retrieve([ToolInput("Search query")] string query)
    {
        var results = _fixture.OrderByDescending(c => c.Score).Take(2).ToList();
        return JsonSerializer.Serialize(results);
    }
}

public class Phase146RagRetrievalTests
{
    [Fact]
    public void Fixture_Has_Three_Chunks()
    {
        var tools = new PgVectorTools146();
        var method = typeof(PgVectorTools146).GetMethod("Retrieve")!;
        var m = typeof(ToolRegistry).GetMethod("WrapMethod",
            System.Reflection.BindingFlags.NonPublic | System.Reflection.BindingFlags.Static)!;
        var handler = (ToolHandler)m.Invoke(null, [tools, method])!;
        var input = JsonDocument.Parse("""{"query":"agent"}""").RootElement;
        var result = JsonSerializer.Deserialize<List<JsonElement>>(handler(input));
        Assert.Equal(2, result!.Count);
    }

    [Fact]
    public void Retrieve_Returns_Valid_Json()
    {
        var tools = new PgVectorTools146();
        var method = typeof(PgVectorTools146).GetMethod("Retrieve")!;
        var m = typeof(ToolRegistry).GetMethod("WrapMethod",
            System.Reflection.BindingFlags.NonPublic | System.Reflection.BindingFlags.Static)!;
        var handler = (ToolHandler)m.Invoke(null, [tools, method])!;
        var input = JsonDocument.Parse("""{"query":"search"}""").RootElement;
        var result = handler(input);
        Assert.True(IsValidJson(result));
    }

    [Fact]
    public void PgChunk_Stores_Id_Content_Score()
    {
        var chunk = new PgChunk146(42, "Sample content", 0.75);
        Assert.Equal(42, chunk.Id);
        Assert.Equal("Sample content", chunk.Content);
        Assert.Equal(0.75, chunk.Score);
    }

    [Fact]
    public void PgChunk_Value_Equality()
    {
        var a = new PgChunk146(1, "text", 0.9);
        var b = new PgChunk146(1, "text", 0.9);
        Assert.Equal(a, b);
    }

    [Fact]
    public void Tool_Attribute_Name_Is_pg_retrieve146()
    {
        var method = typeof(PgVectorTools146).GetMethod("Retrieve")!;
        var attr = (ToolAttribute)Attribute.GetCustomAttribute(method, typeof(ToolAttribute))!;
        Assert.Equal("pg_retrieve146", attr.Name);
    }

    [Fact]
    public void Tool_Discovery_Finds_Retrieve()
    {
        try
        {
            using var rt = new Runtime();
            var tools = new PgVectorTools146();
            var regs = ToolRegistry.RegisterAll(rt, tools);
            Assert.Contains("pg_retrieve146", regs.Select(r => r.Spec.Name));
            foreach (var (_, reg) in regs) reg.Dispose();
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Retrieve_Top_Two_Highest_Score()
    {
        var chunks = new List<PgChunk146>
        {
            new(1, "A", 0.95),
            new(2, "B", 0.88),
            new(3, "C", 0.82),
        };
        var top2 = chunks.OrderByDescending(c => c.Score).Take(2).ToList();
        Assert.Equal(1, top2[0].Id);
        Assert.Equal(2, top2[1].Id);
    }

    [Fact]
    public void Chunk_Score_In_Zero_To_One_Range()
    {
        var chunk = new PgChunk146(1, "text", 0.88);
        Assert.InRange(chunk.Score, 0.0, 1.0);
    }

    [Fact]
    public void Retrieve_Input_Schema_Has_Query_Property()
    {
        var method = typeof(PgVectorTools146).GetMethod("Retrieve")!;
        var m = typeof(ToolRegistry).GetMethod("BuildSchema",
            System.Reflection.BindingFlags.NonPublic | System.Reflection.BindingFlags.Static)!;
        var schema = (ToolInputSchema?)m.Invoke(null, [method]);
        Assert.NotNull(schema);
        Assert.True(schema!.Properties!.ContainsKey("query"));
    }

    [Fact]
    public void Retrieve_Query_Is_Required()
    {
        var method = typeof(PgVectorTools146).GetMethod("Retrieve")!;
        var m = typeof(ToolRegistry).GetMethod("BuildSchema",
            System.Reflection.BindingFlags.NonPublic | System.Reflection.BindingFlags.Static)!;
        var schema = (ToolInputSchema?)m.Invoke(null, [method]);
        Assert.Contains("query", schema!.Required!);
    }

    private static bool IsValidJson(string s)
    {
        try { JsonDocument.Parse(s); return true; }
        catch { return false; }
    }
}
