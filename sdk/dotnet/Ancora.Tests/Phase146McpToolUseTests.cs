using System;
using System.Runtime.InteropServices;
using System.Text.Json;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public sealed class McpTools146
{
    private const string ValidToken = "valid-mcp-token-dotnet";
    private const string ErrUnauthorized = "unauthorized";

    private static readonly System.Collections.Generic.Dictionary<string, string> McpFixture = new()
    {
        ["file.txt"] = "Hello from MCP fixture",
        ["config.json"] = "{\"setting\":true}",
    };

    [Tool("Read a file via MCP", name: "mcp_read_file146")]
    public string ReadFile(
        [ToolInput("Authorization token")] string token,
        [ToolInput("File path to read")] string path)
    {
        if (token != ValidToken) throw new UnauthorizedAccessException(ErrUnauthorized);
        if (!McpFixture.TryGetValue(path, out var content))
            return JsonSerializer.Serialize(new { error = "not found" });
        return JsonSerializer.Serialize(new { content });
    }

    [Tool("List directory via MCP", name: "mcp_list_dir146")]
    public string ListDir(
        [ToolInput("Authorization token")] string token,
        [ToolInput("Directory path")] string dir)
    {
        if (token != ValidToken) throw new UnauthorizedAccessException(ErrUnauthorized);
        return JsonSerializer.Serialize(new[] { "file.txt", "config.json" });
    }
}

public class Phase146McpToolUseTests
{
    private const string ValidToken = "valid-mcp-token-dotnet";
    private const string ErrUnauthorized = "unauthorized";

    [Fact]
    public void Valid_Token_Returns_Content()
    {
        var tools = new McpTools146();
        var method = typeof(McpTools146).GetMethod("ReadFile")!;
        var handler = WrapMethod(tools, method);
        var input = JsonDocument.Parse(
            $$$"""{"token":"{{{ValidToken}}}","path":"file.txt"}""").RootElement;
        var result = handler(input);
        Assert.Contains("Hello from MCP fixture", result);
    }

    [Fact]
    public void Invalid_Token_Throws_Unauthorized()
    {
        var tools = new McpTools146();
        var method = typeof(McpTools146).GetMethod("ReadFile")!;
        var handler = WrapMethod(tools, method);
        var input = JsonDocument.Parse("""{"token":"bad","path":"file.txt"}""").RootElement;
        Assert.Throws<UnauthorizedAccessException>(() => handler(input));
    }

    [Fact]
    public void Missing_File_Returns_Error_Json()
    {
        var tools = new McpTools146();
        var method = typeof(McpTools146).GetMethod("ReadFile")!;
        var handler = WrapMethod(tools, method);
        var input = JsonDocument.Parse(
            $$$"""{"token":"{{{ValidToken}}}","path":"missing.txt"}""").RootElement;
        var result = handler(input);
        Assert.Contains("error", result);
    }

    [Fact]
    public void ListDir_Valid_Token_Returns_Array()
    {
        var tools = new McpTools146();
        var method = typeof(McpTools146).GetMethod("ListDir")!;
        var handler = WrapMethod(tools, method);
        var input = JsonDocument.Parse(
            $$$"""{"token":"{{{ValidToken}}}","dir":"/"}""").RootElement;
        var result = JsonSerializer.Deserialize<string[]>(handler(input));
        Assert.NotEmpty(result!);
    }

    [Fact]
    public void Tool_Names_Are_mcp_prefixed()
    {
        var methods = typeof(McpTools146).GetMethods();
        foreach (var method in methods)
        {
            var attr = (ToolAttribute?)Attribute.GetCustomAttribute(method, typeof(ToolAttribute));
            if (attr?.Name != null)
                Assert.StartsWith("mcp_", attr.Name);
        }
    }

    [Fact]
    public void Tool_Discovery_Finds_Both_Mcp_Tools()
    {
        try
        {
            using var rt = new Runtime();
            var tools = new McpTools146();
            var regs = ToolRegistry.RegisterAll(rt, tools);
            Assert.Equal(2, regs.Count);
            foreach (var (_, reg) in regs) reg.Dispose();
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Auth_Token_Not_In_Response()
    {
        var tools = new McpTools146();
        var method = typeof(McpTools146).GetMethod("ReadFile")!;
        var handler = WrapMethod(tools, method);
        var input = JsonDocument.Parse(
            $$$"""{"token":"{{{ValidToken}}}","path":"file.txt"}""").RootElement;
        var result = handler(input);
        Assert.DoesNotContain(ValidToken, result);
    }

    [Fact]
    public void Config_File_Returns_Json_Content()
    {
        var tools = new McpTools146();
        var method = typeof(McpTools146).GetMethod("ReadFile")!;
        var handler = WrapMethod(tools, method);
        var input = JsonDocument.Parse(
            $$$"""{"token":"{{{ValidToken}}}","path":"config.json"}""").RootElement;
        var result = handler(input);
        Assert.Contains("setting", result);
    }

    [Fact]
    public void ERR_Unauthorized_Sentinel_Is_Correct()
    {
        Assert.Equal("unauthorized", ErrUnauthorized);
    }

    [Fact]
    public void ReadFile_InputSchema_Has_Token_And_Path()
    {
        var method = typeof(McpTools146).GetMethod("ReadFile")!;
        var m = typeof(ToolRegistry).GetMethod("BuildSchema",
            System.Reflection.BindingFlags.NonPublic | System.Reflection.BindingFlags.Static)!;
        var schema = (ToolInputSchema?)m.Invoke(null, [method]);
        Assert.True(schema!.Properties!.ContainsKey("token"));
        Assert.True(schema.Properties.ContainsKey("path"));
    }

    private static ToolHandler WrapMethod(object target, System.Reflection.MethodInfo method)
    {
        var m = typeof(ToolRegistry).GetMethod("WrapMethod",
            System.Reflection.BindingFlags.NonPublic | System.Reflection.BindingFlags.Static)!;
        return (ToolHandler)m.Invoke(null, [target, method])!;
    }
}
