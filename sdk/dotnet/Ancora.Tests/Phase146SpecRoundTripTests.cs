using System.Collections.Generic;
using System.Text;
using System.Text.Json;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public class Phase146SpecRoundTripTests
{
    private static byte[] Encode(AgentSpec spec) => Wire.EncodeAgentSpec(spec);

    [Fact]
    public void AgentSpec_Minimal_Encodes_To_Json()
    {
        var spec = new AgentSpec("llama3");
        var bytes = Encode(spec);
        var json = Encoding.UTF8.GetString(bytes);
        Assert.Contains("llama3", json);
    }

    [Fact]
    public void AgentSpec_Model_Preserved()
    {
        var spec = new AgentSpec("gpt-4o");
        var json = Encoding.UTF8.GetString(Encode(spec));
        Assert.Contains("gpt-4o", json);
    }

    [Fact]
    public void AgentSpec_Instructions_Preserved()
    {
        var spec = new AgentSpec("llama3", Instructions: "Be concise.");
        var json = Encoding.UTF8.GetString(Encode(spec));
        Assert.Contains("Be concise.", json);
    }

    [Fact]
    public void AgentSpec_ToolSpec_Name_Preserved()
    {
        var spec = new AgentSpec("llama3", Tools: [new ToolSpec("my_tool", "Does things")]);
        var json = Encoding.UTF8.GetString(Encode(spec));
        Assert.Contains("my_tool", json);
    }

    [Fact]
    public void AgentSpec_ToolInputSchema_Type_Is_Object()
    {
        var schema = new ToolInputSchema();
        Assert.Equal("object", schema.Type);
    }

    [Fact]
    public void AgentSpec_With_MaxTokens_Encodes()
    {
        var spec = new AgentSpec("llama3", MaxTokens: 512);
        var json = Encoding.UTF8.GetString(Encode(spec));
        Assert.Contains("512", json);
    }

    [Fact]
    public void AgentSpec_With_Temperature_Encodes()
    {
        var spec = new AgentSpec("llama3", Temperature: 0.7);
        var json = Encoding.UTF8.GetString(Encode(spec));
        Assert.Contains("0.7", json);
    }

    [Fact]
    public void AgentSpec_Null_Tools_Omitted()
    {
        var spec = new AgentSpec("llama3");
        var json = Encoding.UTF8.GetString(Encode(spec));
        Assert.DoesNotContain("tools", json);
    }

    [Fact]
    public void AgentSpec_Uses_Snake_Case()
    {
        var spec = new AgentSpec("llama3", MaxTokens: 100);
        var json = Encoding.UTF8.GetString(Encode(spec));
        Assert.Contains("max_tokens", json);
    }

    [Fact]
    public void GraphSpec_Encodes_Nodes_And_Edges()
    {
        var graph = new GraphSpec(
            Nodes: [new GraphNode("n1", NodeKind.Agent, new AgentSpec("llama3"))],
            Edges: [new GraphEdge("n1", "n1")]
        );
        var bytes = Wire.EncodeGraphSpec(graph);
        var json = Encoding.UTF8.GetString(bytes);
        Assert.Contains("n1", json);
    }
}
