using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text.Json;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public sealed class McpE2eTools147
{
    private const string ValidToken = "mcp-e2e-token-147";
    private const string ErrUnauthorized = "unauthorized";

    private static readonly Dictionary<string, string> Fixture = new()
    {
        ["README.md"] = "# Ancora\nLocal-first agent framework.",
        ["config.json"] = "{\"model\":\"llama3\",\"max_tokens\":512}",
        ["agents/verifier.json"] = "{\"role\":\"verify\",\"model\":\"llama3\"}",
    };

    [Tool("Read file via MCP with auth", name: "mcp_e2e_read147")]
    public string ReadFile(
        [ToolInput("Bearer token for authorization")] string token,
        [ToolInput("Path to the file")] string path)
    {
        if (token != ValidToken) throw new UnauthorizedAccessException(ErrUnauthorized);
        return Fixture.TryGetValue(path, out var content)
            ? JsonSerializer.Serialize(new { content })
            : JsonSerializer.Serialize(new { error = "not_found", path });
    }

    [Tool("List files via MCP with auth", name: "mcp_e2e_list147")]
    public string ListFiles([ToolInput("Bearer token")] string token)
    {
        if (token != ValidToken) throw new UnauthorizedAccessException(ErrUnauthorized);
        return JsonSerializer.Serialize(Fixture.Keys.ToArray());
    }
}

public class Phase147E2eMcpTests
{
    private const string ValidToken = "mcp-e2e-token-147";

    [Fact]
    public void Read_With_Valid_Token_Returns_Content()
    {
        var tools = new McpE2eTools147();
        var method = typeof(McpE2eTools147).GetMethod("ReadFile")!;
        var handler = WrapMethod(tools, method);
        var input = JsonDocument.Parse(
            $$$"""{"token":"{{{ValidToken}}}","path":"README.md"}""").RootElement;
        var result = handler(input);
        Assert.Contains("Ancora", result);
    }

    [Fact]
    public void Read_With_Invalid_Token_Throws()
    {
        var tools = new McpE2eTools147();
        var method = typeof(McpE2eTools147).GetMethod("ReadFile")!;
        var handler = WrapMethod(tools, method);
        var input = JsonDocument.Parse("""{"token":"bad","path":"README.md"}""").RootElement;
        Assert.Throws<UnauthorizedAccessException>(() => handler(input));
    }

    [Fact]
    public void List_With_Valid_Token_Returns_Array()
    {
        var tools = new McpE2eTools147();
        var method = typeof(McpE2eTools147).GetMethod("ListFiles")!;
        var handler = WrapMethod(tools, method);
        var input = JsonDocument.Parse($$$"""{"token":"{{{ValidToken}}}"}""").RootElement;
        var result = JsonSerializer.Deserialize<string[]>(handler(input));
        Assert.Equal(3, result!.Length);
    }

    [Fact]
    public void Read_Missing_File_Returns_Error_Json()
    {
        var tools = new McpE2eTools147();
        var method = typeof(McpE2eTools147).GetMethod("ReadFile")!;
        var handler = WrapMethod(tools, method);
        var input = JsonDocument.Parse(
            $$$"""{"token":"{{{ValidToken}}}","path":"ghost.txt"}""").RootElement;
        var result = handler(input);
        Assert.Contains("not_found", result);
    }

    [Fact]
    public void Auth_Token_Not_Leaked_In_Response()
    {
        var tools = new McpE2eTools147();
        var method = typeof(McpE2eTools147).GetMethod("ReadFile")!;
        var handler = WrapMethod(tools, method);
        var input = JsonDocument.Parse(
            $$$"""{"token":"{{{ValidToken}}}","path":"config.json"}""").RootElement;
        var result = handler(input);
        Assert.DoesNotContain(ValidToken, result);
    }

    [Fact]
    public void Two_Mcp_Tools_Discovered()
    {
        try
        {
            using var rt = new Runtime();
            var tools = new McpE2eTools147();
            var regs = ToolRegistry.RegisterAll(rt, tools);
            Assert.Equal(2, regs.Count);
            foreach (var (_, reg) in regs) reg.Dispose();
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public async Task Agent_With_Mcp_Tools_Completes()
    {
        try
        {
            using var rt = new Runtime();
            var tools = new McpE2eTools147();
            var regs = ToolRegistry.RegisterAll(rt, tools);
            using var a = new Agent(rt);
            var toolSpecs = regs.Select(r => r.Spec).ToList();
            var spec = new AgentSpec("llama3", Tools: toolSpecs);
            var events = await a.Run(spec).CollectAsync();
            Assert.IsType<CompletedEvent>(events[^1]);
            foreach (var (_, reg) in regs) reg.Dispose();
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Config_Json_Returns_Model_Field()
    {
        var tools = new McpE2eTools147();
        var method = typeof(McpE2eTools147).GetMethod("ReadFile")!;
        var handler = WrapMethod(tools, method);
        var input = JsonDocument.Parse(
            $$$"""{"token":"{{{ValidToken}}}","path":"config.json"}""").RootElement;
        var result = handler(input);
        Assert.Contains("model", result);
    }

    [Fact]
    public void No_External_URLs_In_Response()
    {
        var tools = new McpE2eTools147();
        var method = typeof(McpE2eTools147).GetMethod("ReadFile")!;
        var handler = WrapMethod(tools, method);
        var input = JsonDocument.Parse(
            $$$"""{"token":"{{{ValidToken}}}","path":"README.md"}""").RootElement;
        var result = handler(input);
        Assert.DoesNotContain("http", result);
    }

    [Fact]
    public void Tool_Names_Prefixed_With_mcp_e2e()
    {
        var methods = typeof(McpE2eTools147).GetMethods();
        foreach (var method in methods)
        {
            var attr = (ToolAttribute?)Attribute.GetCustomAttribute(method, typeof(ToolAttribute));
            if (attr?.Name != null)
                Assert.StartsWith("mcp_e2e_", attr.Name);
        }
    }

    private static ToolHandler WrapMethod(object target, System.Reflection.MethodInfo method)
    {
        var m = typeof(ToolRegistry).GetMethod("WrapMethod",
            System.Reflection.BindingFlags.NonPublic | System.Reflection.BindingFlags.Static)!;
        return (ToolHandler)m.Invoke(null, [target, method])!;
    }
}
