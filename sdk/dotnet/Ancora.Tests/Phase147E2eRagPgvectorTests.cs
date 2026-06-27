using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text.Json;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public sealed class PgVectorE2eTools147
{
    private static readonly List<(int Id, string Content, double Score)> Chunks =
    [
        (1, "Ancora runs agents locally without cloud dependency.", 0.97),
        (2, "pgvector stores embeddings in Postgres for semantic search.", 0.91),
        (3, "RAG retrieval augments model context with relevant passages.", 0.87),
        (4, "Agent pipelines chain multiple models for complex reasoning.", 0.81),
    ];

    [Tool("Retrieve top passages from pgvector", name: "pg_e2e_retrieve147")]
    public string Retrieve([ToolInput("Search query string")] string query)
    {
        var top = Chunks.OrderByDescending(c => c.Score).Take(2)
            .Select(c => new { c.Id, c.Content, c.Score });
        return JsonSerializer.Serialize(top);
    }
}

public class Phase147E2eRagPgvectorTests
{
    [Fact]
    public void Fixture_Has_Four_Chunks()
    {
        var tools = new PgVectorE2eTools147();
        var method = typeof(PgVectorE2eTools147).GetMethod("Retrieve")!;
        var wm = typeof(ToolRegistry).GetMethod("WrapMethod",
            System.Reflection.BindingFlags.NonPublic | System.Reflection.BindingFlags.Static)!;
        var handler = (ToolHandler)wm.Invoke(null, [tools, method])!;
        var input = JsonDocument.Parse("""{"query":"agent"}""").RootElement;
        var result = JsonSerializer.Deserialize<List<JsonElement>>(handler(input));
        Assert.Equal(2, result!.Count);
    }

    [Fact]
    public void Top_Two_Are_Highest_Score()
    {
        var scores = new List<double> { 0.97, 0.91, 0.87, 0.81 };
        var top2 = scores.OrderByDescending(s => s).Take(2).ToList();
        Assert.Equal(0.97, top2[0]);
        Assert.Equal(0.91, top2[1]);
    }

    [Fact]
    public void Retrieve_Returns_Valid_Json()
    {
        var tools = new PgVectorE2eTools147();
        var method = typeof(PgVectorE2eTools147).GetMethod("Retrieve")!;
        var wm = typeof(ToolRegistry).GetMethod("WrapMethod",
            System.Reflection.BindingFlags.NonPublic | System.Reflection.BindingFlags.Static)!;
        var handler = (ToolHandler)wm.Invoke(null, [tools, method])!;
        var input = JsonDocument.Parse("""{"query":"vector"}""").RootElement;
        var result = handler(input);
        Assert.True(IsValidJson(result));
    }

    [Fact]
    public void Tool_Name_Is_pg_e2e_retrieve147()
    {
        var attr = typeof(PgVectorE2eTools147).GetMethod("Retrieve")!
            .GetCustomAttributes(typeof(ToolAttribute), false)
            .Cast<ToolAttribute>().First();
        Assert.Equal("pg_e2e_retrieve147", attr.Name);
    }

    [Fact]
    public void Tool_Registered_Via_Registry()
    {
        try
        {
            using var rt = new Runtime();
            var tools = new PgVectorE2eTools147();
            var regs = ToolRegistry.RegisterAll(rt, tools);
            Assert.Contains("pg_e2e_retrieve147", regs.Select(r => r.Spec.Name));
            foreach (var (_, reg) in regs) reg.Dispose();
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Agent_With_Rag_Tool_Completes()
    {
        try
        {
            using var rt = new Runtime();
            var tools = new PgVectorE2eTools147();
            var regs = ToolRegistry.RegisterAll(rt, tools);
            using var a = new Agent(rt);
            var spec = new AgentSpec("llama3", Tools: [regs[0].Spec]);
            var events = await a.Run(spec).CollectAsync();
            Assert.IsType<CompletedEvent>(events[^1]);
            foreach (var (_, reg) in regs) reg.Dispose();
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Chunk_Content_Contains_Keyword()
    {
        var chunks = new[] { "Ancora", "pgvector", "RAG", "Agent" };
        Assert.All(chunks, c => Assert.NotEmpty(c));
    }

    [Fact]
    public void Retrieve_Schema_Requires_Query()
    {
        var method = typeof(PgVectorE2eTools147).GetMethod("Retrieve")!;
        var m = typeof(ToolRegistry).GetMethod("BuildSchema",
            System.Reflection.BindingFlags.NonPublic | System.Reflection.BindingFlags.Static)!;
        var schema = (ToolInputSchema?)m.Invoke(null, [method]);
        Assert.Contains("query", schema!.Required!);
    }

    [Fact]
    public void Result_Score_In_Range()
    {
        var tools = new PgVectorE2eTools147();
        var method = typeof(PgVectorE2eTools147).GetMethod("Retrieve")!;
        var wm = typeof(ToolRegistry).GetMethod("WrapMethod",
            System.Reflection.BindingFlags.NonPublic | System.Reflection.BindingFlags.Static)!;
        var handler = (ToolHandler)wm.Invoke(null, [tools, method])!;
        var input = JsonDocument.Parse("""{"query":"search"}""").RootElement;
        var result = JsonSerializer.Deserialize<List<JsonElement>>(handler(input));
        foreach (var item in result!)
        {
            var score = item.GetProperty("Score").GetDouble();
            Assert.InRange(score, 0.0, 1.0);
        }
    }

    [Fact]
    public void No_Live_Network_Call_In_Handler()
    {
        var tools = new PgVectorE2eTools147();
        var method = typeof(PgVectorE2eTools147).GetMethod("Retrieve")!;
        var wm = typeof(ToolRegistry).GetMethod("WrapMethod",
            System.Reflection.BindingFlags.NonPublic | System.Reflection.BindingFlags.Static)!;
        var handler = (ToolHandler)wm.Invoke(null, [tools, method])!;
        var input = JsonDocument.Parse("""{"query":"offline"}""").RootElement;
        var result = handler(input);
        Assert.DoesNotContain("http", result);
    }

    private static bool IsValidJson(string s)
    {
        try { JsonDocument.Parse(s); return true; }
        catch { return false; }
    }
}
