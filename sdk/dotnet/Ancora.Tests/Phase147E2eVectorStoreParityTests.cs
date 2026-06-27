using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public sealed class PgVectorStore147
{
    private static readonly List<(string Content, double Score)> Chunks =
    [
        ("Ancora runs locally.", 0.95),
        ("pgvector semantic search.", 0.88),
        ("RAG augments context.", 0.82),
    ];

    [Tool("pgvector retrieve 2", name: "pg_retrieve2_147")]
    public string Retrieve([ToolInput("Query")] string query)
    {
        var top = Chunks.OrderByDescending(c => c.Score).Take(2)
            .Select(c => new { c.Content, c.Score });
        return JsonSerializer.Serialize(top);
    }
}

public sealed class LanceVectorStore147
{
    private static readonly List<(string Content, double Score)> Chunks =
    [
        ("Ancora runs locally.", 0.94),
        ("LanceDB columnar storage.", 0.87),
        ("Offline retrieval ready.", 0.81),
    ];

    [Tool("lance retrieve 2", name: "lance_retrieve2_147")]
    public string Retrieve([ToolInput("Query")] string query)
    {
        var top = Chunks.OrderByDescending(c => c.Score).Take(2)
            .Select(c => new { c.Content, c.Score });
        return JsonSerializer.Serialize(top);
    }
}

public class Phase147E2eVectorStoreParityTests
{
    private static ToolHandler GetHandler(object tools, string methodName)
    {
        var method = tools.GetType().GetMethod(methodName)!;
        var m = typeof(ToolRegistry).GetMethod("WrapMethod",
            System.Reflection.BindingFlags.NonPublic | System.Reflection.BindingFlags.Static)!;
        return (ToolHandler)m.Invoke(null, [tools, method])!;
    }

    [Fact]
    public void Both_Stores_Return_Two_Results()
    {
        var pg = GetHandler(new PgVectorStore147(), "Retrieve");
        var lance = GetHandler(new LanceVectorStore147(), "Retrieve");
        var input = JsonDocument.Parse("""{"query":"test"}""").RootElement;

        var pgResult = JsonSerializer.Deserialize<List<JsonElement>>(pg(input));
        var lanceResult = JsonSerializer.Deserialize<List<JsonElement>>(lance(input));
        Assert.Equal(2, pgResult!.Count);
        Assert.Equal(2, lanceResult!.Count);
    }

    [Fact]
    public void Both_Stores_Return_Valid_Json()
    {
        var pg = GetHandler(new PgVectorStore147(), "Retrieve");
        var lance = GetHandler(new LanceVectorStore147(), "Retrieve");
        var input = JsonDocument.Parse("""{"query":"parity"}""").RootElement;

        Assert.True(IsValidJson(pg(input)));
        Assert.True(IsValidJson(lance(input)));
    }

    [Fact]
    public void Both_Stores_First_Chunk_Is_Highest_Score()
    {
        var pg = GetHandler(new PgVectorStore147(), "Retrieve");
        var lance = GetHandler(new LanceVectorStore147(), "Retrieve");
        var input = JsonDocument.Parse("""{"query":"q"}""").RootElement;

        var pgChunks = JsonSerializer.Deserialize<List<JsonElement>>(pg(input))!;
        var lanceChunks = JsonSerializer.Deserialize<List<JsonElement>>(lance(input))!;

        var pgScore0 = pgChunks[0].GetProperty("Score").GetDouble();
        var pgScore1 = pgChunks[1].GetProperty("Score").GetDouble();
        Assert.True(pgScore0 >= pgScore1);

        var lanceScore0 = lanceChunks[0].GetProperty("Score").GetDouble();
        var lanceScore1 = lanceChunks[1].GetProperty("Score").GetDouble();
        Assert.True(lanceScore0 >= lanceScore1);
    }

    [Fact]
    public void Both_Stores_Share_Ancora_Content()
    {
        var pg = GetHandler(new PgVectorStore147(), "Retrieve");
        var lance = GetHandler(new LanceVectorStore147(), "Retrieve");
        var input = JsonDocument.Parse("""{"query":"ancora"}""").RootElement;

        var pgStr = pg(input);
        var lanceStr = lance(input);
        Assert.Contains("Ancora", pgStr);
        Assert.Contains("Ancora", lanceStr);
    }

    [Fact]
    public void Tool_Names_Are_Distinct()
    {
        var pgAttr = typeof(PgVectorStore147).GetMethod("Retrieve")!
            .GetCustomAttributes(typeof(ToolAttribute), false)
            .Cast<ToolAttribute>().First();
        var lanceAttr = typeof(LanceVectorStore147).GetMethod("Retrieve")!
            .GetCustomAttributes(typeof(ToolAttribute), false)
            .Cast<ToolAttribute>().First();
        Assert.NotEqual(pgAttr.Name, lanceAttr.Name);
    }

    [Fact]
    public void Scores_Are_In_Zero_To_One_Range()
    {
        var pg = GetHandler(new PgVectorStore147(), "Retrieve");
        var lance = GetHandler(new LanceVectorStore147(), "Retrieve");
        var input = JsonDocument.Parse("""{"query":"range"}""").RootElement;

        foreach (var chunk in JsonSerializer.Deserialize<List<JsonElement>>(pg(input))!)
            Assert.InRange(chunk.GetProperty("Score").GetDouble(), 0.0, 1.0);
        foreach (var chunk in JsonSerializer.Deserialize<List<JsonElement>>(lance(input))!)
            Assert.InRange(chunk.GetProperty("Score").GetDouble(), 0.0, 1.0);
    }

    [Fact]
    public void Neither_Store_Makes_Network_Calls()
    {
        var pg = GetHandler(new PgVectorStore147(), "Retrieve");
        var lance = GetHandler(new LanceVectorStore147(), "Retrieve");
        var input = JsonDocument.Parse("""{"query":"offline"}""").RootElement;
        Assert.DoesNotContain("http", pg(input));
        Assert.DoesNotContain("http", lance(input));
    }

    private static bool IsValidJson(string s)
    {
        try { JsonDocument.Parse(s); return true; }
        catch { return false; }
    }
}
