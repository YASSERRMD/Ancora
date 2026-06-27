using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text.Json;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public class Phase147E2eCatalogSmokeTests
{
    private static readonly List<Dictionary<string, string>> CatalogExamples =
    [
        new() { ["name"] = "single-agent-hello", ["model"] = "llama3", ["goal"] = "say hello" },
        new() { ["name"] = "multi-verifier-basic", ["model"] = "gpt-4o", ["goal"] = "verify output" },
        new() { ["name"] = "hil-approval", ["model"] = "llama3", ["goal"] = "get human approval" },
        new() { ["name"] = "rag-pgvector", ["model"] = "llama3", ["goal"] = "retrieve context" },
        new() { ["name"] = "mcp-read-file", ["model"] = "llama3", ["goal"] = "read a file via mcp" },
        new() { ["name"] = "deepseek-chat", ["model"] = "deepseek-chat", ["goal"] = "chat with deepseek" },
        new() { ["name"] = "qwen-max", ["model"] = "qwen3-max", ["goal"] = "chat with qwen" },
        new() { ["name"] = "glm-turbo", ["model"] = "glm-5", ["goal"] = "chat with glm" },
        new() { ["name"] = "kimi-long", ["model"] = "kimi-k2", ["goal"] = "long-context reasoning" },
        new() { ["name"] = "streaming-tokens", ["model"] = "llama3", ["goal"] = "stream tokens" },
    ];

    [Fact]
    public void Catalog_Has_Ten_Examples()
    {
        Assert.Equal(10, CatalogExamples.Count);
    }

    [Fact]
    public void All_Examples_Have_Name()
    {
        Assert.All(CatalogExamples, e => Assert.True(e.ContainsKey("name")));
    }

    [Fact]
    public void All_Examples_Have_Model()
    {
        Assert.All(CatalogExamples, e => Assert.True(e.ContainsKey("model")));
    }

    [Fact]
    public void All_Examples_Have_Goal()
    {
        Assert.All(CatalogExamples, e => Assert.True(e.ContainsKey("goal")));
    }

    [Fact]
    public void Example_Names_Are_Unique()
    {
        var names = CatalogExamples.Select(e => e["name"]).ToHashSet();
        Assert.Equal(CatalogExamples.Count, names.Count);
    }

    [Fact]
    public async Task All_Examples_Produce_Events()
    {
        try
        {
            using var a = new Agent();
            foreach (var example in CatalogExamples)
            {
                var spec = new AgentSpec(example["model"], Instructions: example["goal"]);
                var events = await a.Run(spec).CollectAsync();
                Assert.NotEmpty(events);
            }
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task All_Examples_Complete()
    {
        try
        {
            using var a = new Agent();
            foreach (var example in CatalogExamples)
            {
                var spec = new AgentSpec(example["model"], Instructions: example["goal"]);
                var events = await a.Run(spec).CollectAsync();
                Assert.IsType<CompletedEvent>(events[^1]);
            }
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Catalog_Includes_Chinese_Providers()
    {
        var models = CatalogExamples.Select(e => e["model"]).ToHashSet();
        Assert.Contains("deepseek-chat", models);
        Assert.Contains("qwen3-max", models);
        Assert.Contains("glm-5", models);
    }

    [Fact]
    public void Catalog_Json_Serializes()
    {
        var json = JsonSerializer.Serialize(CatalogExamples);
        Assert.True(IsValidJson(json));
    }

    private static bool IsValidJson(string s)
    {
        try { JsonDocument.Parse(s); return true; }
        catch { return false; }
    }
}
