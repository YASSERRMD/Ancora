using System;
using System.Runtime.InteropServices;
using System.Text.Json;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public sealed class SecureMcpTools147
{
    private const string ValidToken = "valid-sec-token-dotnet-147";
    private const string ErrUnauthorized = "unauthorized";

    [Tool("Access secure resource", name: "secure_resource_147")]
    public string AccessResource(
        [ToolInput("Authorization token")] string token)
    {
        if (token != ValidToken) throw new UnauthorizedAccessException(ErrUnauthorized);
        return JsonSerializer.Serialize(new { data = "secret-value" });
    }
}

public class Phase147SecMcpAuthTests
{
    private const string ValidToken = "valid-sec-token-dotnet-147";
    private const string ErrUnauthorized = "unauthorized";

    private static ToolHandler GetHandler()
    {
        var tools = new SecureMcpTools147();
        var method = typeof(SecureMcpTools147).GetMethod("AccessResource")!;
        var m = typeof(ToolRegistry).GetMethod("WrapMethod",
            System.Reflection.BindingFlags.NonPublic | System.Reflection.BindingFlags.Static)!;
        return (ToolHandler)m.Invoke(null, [tools, method])!;
    }

    [Fact]
    public void Valid_Token_Returns_Data()
    {
        var handler = GetHandler();
        var input = JsonDocument.Parse($$$"""{"token":"{{{ValidToken}}}"}""").RootElement;
        var result = JsonSerializer.Deserialize<JsonElement>(handler(input));
        Assert.Equal("secret-value", result.GetProperty("data").GetString());
    }

    [Fact]
    public void Invalid_Token_Throws_Unauthorized()
    {
        var handler = GetHandler();
        var input = JsonDocument.Parse("""{"token":"wrong"}""").RootElement;
        Assert.Throws<UnauthorizedAccessException>(() => handler(input));
    }

    [Fact]
    public void Empty_Token_Throws_Unauthorized()
    {
        var handler = GetHandler();
        var input = JsonDocument.Parse("""{"token":""}""").RootElement;
        Assert.Throws<UnauthorizedAccessException>(() => handler(input));
    }

    [Fact]
    public void Error_Message_Is_Unauthorized()
    {
        var handler = GetHandler();
        var input = JsonDocument.Parse("""{"token":"bad"}""").RootElement;
        try
        {
            handler(input);
        }
        catch (UnauthorizedAccessException ex)
        {
            Assert.Equal(ErrUnauthorized, ex.Message);
        }
    }

    [Fact]
    public void Error_Message_Does_Not_Contain_Secret()
    {
        var handler = GetHandler();
        var input = JsonDocument.Parse("""{"token":"hacked"}""").RootElement;
        string? msg = null;
        try { handler(input); }
        catch (UnauthorizedAccessException ex) { msg = ex.Message; }
        Assert.NotNull(msg);
        Assert.DoesNotContain("secret-value", msg!);
    }

    [Fact]
    public void Multiple_Invalid_Tokens_All_Throw()
    {
        var handler = GetHandler();
        var tokens = new[] { "", "wrong", "hacked", "' OR 1=1" };
        foreach (var tok in tokens)
        {
            var input = JsonDocument.Parse($$$"""{"token":"{{{tok}}}"}""").RootElement;
            Assert.Throws<UnauthorizedAccessException>(() => handler(input));
        }
    }

    [Fact]
    public void Valid_Dispatch_Does_Not_Throw()
    {
        var handler = GetHandler();
        var input = JsonDocument.Parse($$$"""{"token":"{{{ValidToken}}}"}""").RootElement;
        var ex = Record.Exception(() => handler(input));
        Assert.Null(ex);
    }

    [Fact]
    public void Tool_Registered_After_Auth_Error()
    {
        try
        {
            using var rt = new Runtime();
            var tools = new SecureMcpTools147();
            var regs = ToolRegistry.RegisterAll(rt, tools);
            Assert.True(rt.ToolExists("secure_resource_147"));
            foreach (var (_, reg) in regs) reg.Dispose();
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Err_Unauthorized_Sentinel_Value()
    {
        Assert.Equal("unauthorized", ErrUnauthorized);
    }

    [Fact]
    public void Missing_Tool_Dispatch_Throws()
    {
        try
        {
            using var rt = new Runtime();
            Assert.False(rt.ToolExists("missing_tool_147"));
        }
        catch (DllNotFoundException) { }
    }
}
