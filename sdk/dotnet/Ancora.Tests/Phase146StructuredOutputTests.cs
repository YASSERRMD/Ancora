using System.Collections.Generic;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public record AnswerRecord146(string Answer, double Confidence);

public class Phase146StructuredOutputTests
{
    [Fact]
    public void AgentSpec_Is_Record_Type()
    {
        Assert.True(typeof(AgentSpec).IsClass);
        var methods = typeof(AgentSpec).GetMethods();
        Assert.Contains(methods, m => m.Name == "<Clone>$");
    }

    [Fact]
    public void AgentSpec_Value_Equality()
    {
        var a = new AgentSpec("llama3", Instructions: "Be helpful");
        var b = new AgentSpec("llama3", Instructions: "Be helpful");
        Assert.Equal(a, b);
    }

    [Fact]
    public void AgentSpec_With_Expression_Creates_New_Instance()
    {
        var a = new AgentSpec("llama3");
        var b = a with { Model = "gpt-4o" };
        Assert.Equal("gpt-4o", b.Model);
        Assert.Equal("llama3", a.Model);
    }

    [Fact]
    public void ToolSpec_Is_Record_Type()
    {
        var t = new ToolSpec("tool_a", "Does A");
        var t2 = t with { Name = "tool_b" };
        Assert.Equal("tool_b", t2.Name);
        Assert.Equal("tool_a", t.Name);
    }

    [Fact]
    public void AnswerRecord_Stores_Fields()
    {
        var ans = new AnswerRecord146("Paris", 0.99);
        Assert.Equal("Paris", ans.Answer);
        Assert.Equal(0.99, ans.Confidence);
    }

    [Fact]
    public void AnswerRecord_Value_Equality()
    {
        var a = new AnswerRecord146("Paris", 0.99);
        var b = new AnswerRecord146("Paris", 0.99);
        Assert.Equal(a, b);
    }

    [Fact]
    public void AnswerRecord_With_Expression()
    {
        var a = new AnswerRecord146("Paris", 0.99);
        var b = a with { Confidence = 0.5 };
        Assert.Equal(0.5, b.Confidence);
        Assert.Equal(0.99, a.Confidence);
    }

    [Fact]
    public void StartedEvent_Is_Record()
    {
        var ev = new StartedEvent("run-1", "{}");
        var ev2 = ev with { Spec = "{}" };
        Assert.Equal(ev.RunId, ev2.RunId);
    }

    [Fact]
    public void TokenEvent_Stores_Text()
    {
        var ev = new TokenEvent("run-1", "Hello");
        Assert.Equal("Hello", ev.Text);
        Assert.Equal("token", ev.Kind);
    }

    [Fact]
    public void ToolCallEvent_Stores_Name_And_Input()
    {
        var ev = new ToolCallEvent("run-1", "search", "{\"q\":\"cats\"}");
        Assert.Equal("search", ev.Name);
        Assert.Equal("{\"q\":\"cats\"}", ev.Input);
    }
}
