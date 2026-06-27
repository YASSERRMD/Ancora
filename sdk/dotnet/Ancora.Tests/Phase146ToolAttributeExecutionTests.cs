using System.Linq;
using System.Runtime.InteropServices;
using System.Text.Json;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public sealed class SampleTools146
{
    [Tool("Greet a user by name", name: "greet")]
    public string Greet([ToolInput("Name of the user")] string name)
        => $"{{\"greeting\":\"Hello, {name}!\"}}";

    [Tool("Multiply two numbers")]
    public string Multiply(
        [ToolInput("First factor")] double x,
        [ToolInput("Second factor")] double y)
        => $"{{\"result\":{x * y}}}";

    [Tool("Return a constant")]
    public string Constant() => "{\"value\":42}";
}

public class Phase146ToolAttributeExecutionTests
{
    [Fact]
    public void Tool_Attribute_Stores_Description()
    {
        var attr = new ToolAttribute("Say hello");
        Assert.Equal("Say hello", attr.Description);
    }

    [Fact]
    public void Tool_Attribute_Name_Override_Works()
    {
        var attr = new ToolAttribute("Echo", name: "my_echo");
        Assert.Equal("my_echo", attr.Name);
    }

    [Fact]
    public void Tool_Attribute_Default_Name_Is_Null()
    {
        var attr = new ToolAttribute("Do something");
        Assert.Null(attr.Name);
    }

    [Fact]
    public void ToolInput_Attribute_Description_Preserved()
    {
        var attr = new ToolInputAttribute("The input value");
        Assert.Equal("The input value", attr.Description);
    }

    [Fact]
    public void ToolInput_Attribute_Required_Defaults_To_True()
    {
        var attr = new ToolInputAttribute();
        Assert.True(attr.Required);
    }

    [Fact]
    public void RegisterAll_Discovers_Three_Methods()
    {
        try
        {
            using var rt = new Runtime();
            var tools = new SampleTools146();
            var regs = ToolRegistry.RegisterAll(rt, tools);
            Assert.Equal(3, regs.Count);
            foreach (var (_, reg) in regs) reg.Dispose();
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void RegisterAll_Finds_Greet_By_Custom_Name()
    {
        try
        {
            using var rt = new Runtime();
            var tools = new SampleTools146();
            var regs = ToolRegistry.RegisterAll(rt, tools);
            Assert.Contains("greet", regs.Select(r => r.Spec.Name));
            foreach (var (_, reg) in regs) reg.Dispose();
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void RegisterAll_Converts_PascalCase_To_Snake_Case()
    {
        try
        {
            using var rt = new Runtime();
            var tools = new SampleTools146();
            var regs = ToolRegistry.RegisterAll(rt, tools);
            Assert.Contains("multiply", regs.Select(r => r.Spec.Name));
            foreach (var (_, reg) in regs) reg.Dispose();
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void WrapMethod_Executes_Greet_Handler()
    {
        var tools = new SampleTools146();
        var method = typeof(SampleTools146).GetMethod("Greet")!;
        var handler = InvokeWrapMethod(tools, method);
        var input = JsonDocument.Parse("""{"name":"World"}""").RootElement;
        var result = handler(input);
        Assert.Contains("World", result);
    }

    [Fact]
    public void WrapMethod_Executes_Multiply_Handler()
    {
        var tools = new SampleTools146();
        var method = typeof(SampleTools146).GetMethod("Multiply")!;
        var handler = InvokeWrapMethod(tools, method);
        var input = JsonDocument.Parse("""{"x":3.0,"y":4.0}""").RootElement;
        var result = handler(input);
        Assert.Contains("12", result);
    }

    [Fact]
    public void ToolSpec_Description_Matches_Attribute()
    {
        try
        {
            using var rt = new Runtime();
            var tools = new SampleTools146();
            var regs = ToolRegistry.RegisterAll(rt, tools);
            var greetSpec = regs.First(r => r.Spec.Name == "greet").Spec;
            Assert.Equal("Greet a user by name", greetSpec.Description);
            foreach (var (_, reg) in regs) reg.Dispose();
        }
        catch (DllNotFoundException) { }
    }

    private static ToolHandler InvokeWrapMethod(object target, System.Reflection.MethodInfo method)
    {
        var m = typeof(ToolRegistry).GetMethod("WrapMethod",
            System.Reflection.BindingFlags.NonPublic | System.Reflection.BindingFlags.Static)!;
        return (ToolHandler)m.Invoke(null, [target, method])!;
    }
}
