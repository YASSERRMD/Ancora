using System;
using System.Collections.Generic;
using System.Text;
using System.Text.Json;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public class AgentSpecSerializationTests
{
    [Fact]
    public void AgentSpec_Serializes_Model_And_Instructions()
    {
        var spec = new AgentSpec("llama3", "You are helpful");
        var bytes = Wire.EncodeAgentSpec(spec);
        var json = Encoding.UTF8.GetString(bytes);
        Assert.Contains("\"model\":", json);
        Assert.Contains("\"llama3\"", json);
        Assert.Contains("\"instructions\":", json);
        Assert.Contains("\"You are helpful\"", json);
    }

    [Fact]
    public void AgentSpec_Omits_Null_Optional_Fields()
    {
        var spec = new AgentSpec("llama3");
        var bytes = Wire.EncodeAgentSpec(spec);
        var json = Encoding.UTF8.GetString(bytes);
        Assert.DoesNotContain("max_tokens", json);
        Assert.DoesNotContain("temperature", json);
    }

    [Fact]
    public void AgentSpec_Includes_MaxTokens_When_Set()
    {
        var spec = new AgentSpec("llama3", MaxTokens: 512);
        var bytes = Wire.EncodeAgentSpec(spec);
        var json = Encoding.UTF8.GetString(bytes);
        Assert.Contains("\"max_tokens\":512", json);
    }

    [Fact]
    public void AgentSpec_Serializes_Tool_Spec()
    {
        var tool = new ToolSpec("echo", "Echo the input",
            new ToolInputSchema(Properties: new Dictionary<string, ToolInputProperty>
            {
                ["message"] = new ToolInputProperty("string", "The message to echo"),
            },
            Required: ["message"]));
        var spec = new AgentSpec("llama3", Tools: [tool]);
        var bytes = Wire.EncodeAgentSpec(spec);
        var json = Encoding.UTF8.GetString(bytes);
        Assert.Contains("\"echo\"", json);
        Assert.Contains("\"message\"", json);
    }
}

public class RunEventDeserializationTests
{
    [Fact]
    public void ParseEvent_Returns_StartedEvent()
    {
        var json = """{"kind":"started","run_id":"r1","spec":"{}"}""";
        var ev = Wire.ParseEvent(json);
        var started = Assert.IsType<StartedEvent>(ev);
        Assert.Equal("r1", started.RunId);
        Assert.Equal("{}", started.Spec);
        Assert.Equal("started", started.Kind);
    }

    [Fact]
    public void ParseEvent_Returns_CompletedEvent()
    {
        var json = """{"kind":"completed","run_id":"r2"}""";
        var ev = Wire.ParseEvent(json);
        var completed = Assert.IsType<CompletedEvent>(ev);
        Assert.Equal("r2", completed.RunId);
    }

    [Fact]
    public void ParseEvent_Returns_TokenEvent()
    {
        var json = """{"kind":"token","run_id":"r3","text":"hello"}""";
        var ev = Wire.ParseEvent(json);
        var token = Assert.IsType<TokenEvent>(ev);
        Assert.Equal("hello", token.Text);
    }

    [Fact]
    public void ParseEvent_Returns_ResumedEvent()
    {
        var json = """{"kind":"resumed","run_id":"r4","decision":"approve"}""";
        var ev = Wire.ParseEvent(json);
        var resumed = Assert.IsType<ResumedEvent>(ev);
        Assert.Equal("approve", resumed.Decision);
    }

    [Fact]
    public void ParseEvent_Returns_ToolCallEvent()
    {
        var json = """{"kind":"tool_call","run_id":"r5","name":"echo","input":"{}"}""";
        var ev = Wire.ParseEvent(json);
        var toolCall = Assert.IsType<ToolCallEvent>(ev);
        Assert.Equal("echo", toolCall.Name);
        Assert.Equal("{}", toolCall.Input);
    }

    [Fact]
    public void ParseEvent_Throws_On_Unknown_Kind()
    {
        var json = """{"kind":"unknown_kind","run_id":"r6"}""";
        Assert.Throws<JsonException>(() => Wire.ParseEvent(json));
    }
}

public class GraphSpecTests
{
    [Fact]
    public void GraphNode_Stores_Kind_And_Id()
    {
        var node = new GraphNode("n1", NodeKind.Agent,
            new AgentSpec("llama3", "Be helpful"));
        Assert.Equal("n1", node.Id);
        Assert.Equal(NodeKind.Agent, node.Kind);
        Assert.Equal("llama3", node.AgentSpec!.Model);
    }

    [Fact]
    public void GraphEdge_Stores_From_And_To()
    {
        var edge = new GraphEdge("n1", "n2");
        Assert.Equal("n1", edge.From);
        Assert.Equal("n2", edge.To);
        Assert.Null(edge.Condition);
    }
}
