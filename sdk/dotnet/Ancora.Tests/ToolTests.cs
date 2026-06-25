using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text.Json;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

// --- sample class with [Tool] methods for reflection tests ---

public sealed class EchoTools
{
    [Tool("Echo the message back to the caller", name: "echo")]
    public string Echo(
        [ToolInput("The message to echo")] string message)
    {
        return $"{{\"result\":\"{message}\"}}";
    }

    [Tool("Add two integers")]
    public string AddNumbers(
        [ToolInput("First operand")] int a,
        [ToolInput("Second operand")] int b)
    {
        return $"{{\"sum\":{a + b}}}";
    }
}

// --- attribute tests ---

public class ToolAttributeTests
{
    [Fact]
    public void ToolAttribute_Stores_Description()
    {
        var attr = new ToolAttribute("Does something useful");
        Assert.Equal("Does something useful", attr.Description);
        Assert.Null(attr.Name);
    }

    [Fact]
    public void ToolAttribute_Stores_Custom_Name()
    {
        var attr = new ToolAttribute("Does something", name: "my_tool");
        Assert.Equal("my_tool", attr.Name);
    }

    [Fact]
    public void ToolInputAttribute_Defaults_Required_To_True()
    {
        var attr = new ToolInputAttribute("A parameter");
        Assert.Equal("A parameter", attr.Description);
        Assert.True(attr.Required);
    }

    [Fact]
    public void ToolInputAttribute_Can_Mark_Optional()
    {
        var attr = new ToolInputAttribute() { Required = false };
        Assert.False(attr.Required);
    }
}

// --- schema generation tests (via RegisterAll without a real runtime) ---

public class ToolSchemaTests
{
    [Fact]
    public void ToSnakeCase_Converts_PascalCase_To_Snake_Case()
    {
        Assert.Equal("add_numbers",
            InvokeToSnakeCase("AddNumbers"));
    }

    [Fact]
    public void WrapMethod_Produces_Valid_Json_Result()
    {
        var tools = new EchoTools();
        var method = typeof(EchoTools).GetMethod("Echo")!;
        var handler = InvokeWrapMethod(tools, method);
        var input = JsonDocument.Parse("""{"message":"hello"}""").RootElement;
        var result = handler(input);
        Assert.Contains("hello", result);
    }

    // reflection helpers to access private methods for testing

    private static string InvokeToSnakeCase(string name)
    {
        var method = typeof(ToolRegistry).GetMethod("ToSnakeCase",
            System.Reflection.BindingFlags.NonPublic | System.Reflection.BindingFlags.Static)!;
        return (string)method.Invoke(null, [name])!;
    }

    private static ToolHandler InvokeWrapMethod(object target,
        System.Reflection.MethodInfo method)
    {
        var m = typeof(ToolRegistry).GetMethod("WrapMethod",
            System.Reflection.BindingFlags.NonPublic | System.Reflection.BindingFlags.Static)!;
        return (ToolHandler)m.Invoke(null, [target, method])!;
    }
}

// --- integration tests ---

public class ToolIntegrationTests
{
    [Fact]
    public void Register_Returns_IDisposable()
    {
        try
        {
            using var rt = new Runtime();
            IDisposable reg = ToolRegistry.Register(rt, "ping", "Ping the runtime",
                input => """{"pong":true}""");
            Assert.NotNull(reg);
            reg.Dispose();
            // after dispose the tool should be gone
            Assert.False(rt.ToolExists("ping"));
        }
        catch (DllNotFoundException)
        {
            // Native library not present; CI provides it.
        }
    }

    [Fact]
    public void Register_Makes_Tool_Visible_Via_ToolExists()
    {
        try
        {
            using var rt = new Runtime();
            using var reg = ToolRegistry.Register(rt, "greet", "Say hello",
                _ => """{"greeting":"hello"}""");
            Assert.True(rt.ToolExists("greet"));
            Assert.Equal((nuint)1, rt.ToolCount());
        }
        catch (DllNotFoundException)
        {
            // Native library not present; CI provides it.
        }
    }

    [Fact]
    public void RegisterAll_Discovers_Tool_Methods_Via_Reflection()
    {
        try
        {
            using var rt = new Runtime();
            var echoTools = new EchoTools();
            var registrations = ToolRegistry.RegisterAll(rt, echoTools);
            Assert.Equal(2, registrations.Count);
            var names = registrations.Select(r => r.Spec.Name).ToHashSet();
            Assert.Contains("echo", names);
            Assert.Contains("add_numbers", names);
            // dispose all
            foreach (var (_, reg) in registrations) reg.Dispose();
        }
        catch (DllNotFoundException)
        {
            // Native library not present; CI provides it.
        }
    }

    [Fact]
    public void RegisterAll_Builds_Correct_ToolSpec()
    {
        try
        {
            using var rt = new Runtime();
            var echoTools = new EchoTools();
            var registrations = ToolRegistry.RegisterAll(rt, echoTools);
            var echoSpec = registrations.First(r => r.Spec.Name == "echo").Spec;
            Assert.Equal("Echo the message back to the caller", echoSpec.Description);
            Assert.NotNull(echoSpec.InputSchema);
            Assert.True(echoSpec.InputSchema!.Properties!.ContainsKey("message"));
            Assert.Equal("string", echoSpec.InputSchema.Properties["message"].Type);
            foreach (var (_, reg) in registrations) reg.Dispose();
        }
        catch (DllNotFoundException)
        {
            // Native library not present; CI provides it.
        }
    }

    [Fact]
    public async Task Tool_Runs_Within_A_Run()
    {
        try
        {
            using var rt = new Runtime();
            using var reg = ToolRegistry.Register(rt, "ping", "Ping",
                _ => """{"pong":true}""");
            var spec = new AgentSpec("test-model", "Use ping tool", Tools:
            [
                new ToolSpec("ping", "Ping the runtime")
            ]);
            using var agent = new Agent(rt);
            var handle = agent.Run(spec);
            var events = await handle.CollectAsync();
            Assert.True(events.Count > 0);
        }
        catch (DllNotFoundException)
        {
            // Native library not present; CI provides it.
        }
    }

    [Fact]
    public void Double_Dispose_Registration_Is_Safe()
    {
        try
        {
            using var rt = new Runtime();
            var reg = ToolRegistry.Register(rt, "safe", "Safe tool",
                _ => """{"ok":true}""");
            reg.Dispose();
            reg.Dispose(); // must not throw
        }
        catch (DllNotFoundException)
        {
            // Native library not present; CI provides it.
        }
    }
}
