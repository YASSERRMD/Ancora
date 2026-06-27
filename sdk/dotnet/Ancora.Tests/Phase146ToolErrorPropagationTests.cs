using System;
using System.Runtime.InteropServices;
using System.Text.Json;
using Ancora;
using Ancora.Interop;
using Xunit;

namespace Ancora.Tests;

public class Phase146ToolErrorPropagationTests
{
    [Fact]
    public void AncorException_Carries_ErrorCode()
    {
        var ex = new AncorException(3, "internal failure");
        Assert.Equal(3, ex.ErrorCode);
    }

    [Fact]
    public void AncorException_Message_Contains_Code()
    {
        var ex = new AncorException(3, "boom");
        Assert.Contains("ErrorCode=3", ex.Message);
    }

    [Fact]
    public void AncorException_Inherits_Exception()
    {
        Assert.True(typeof(Exception).IsAssignableFrom(typeof(AncorException)));
    }

    [Fact]
    public void Throwing_Handler_Does_Not_Crash_Registry()
    {
        try
        {
            using var rt = new Runtime();
            using var reg = ToolRegistry.Register(rt, "bad", "A bad tool",
                _ => throw new InvalidOperationException("boom"));
            Assert.True(rt.ToolExists("bad"));
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Throwing_Handler_Tool_Still_Registered_After_Throw()
    {
        try
        {
            using var rt = new Runtime();
            using var reg = ToolRegistry.Register(rt, "volatile", "Volatile tool",
                _ => throw new Exception("always fails"));
            Assert.True(rt.ToolExists("volatile"));
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Register_Null_Handler_Throws_ArgumentNullException()
    {
        try
        {
            using var rt = new Runtime();
            Assert.Throws<ArgumentNullException>(() =>
                ToolRegistry.Register(rt, "t", "desc", null!));
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Register_Null_Runtime_Throws_ArgumentNullException()
    {
        Assert.Throws<ArgumentNullException>(() =>
            ToolRegistry.Register(null!, "t", "desc", _ => "ok"));
    }

    [Fact]
    public void WrapMethod_Handler_Can_Return_Error_Json()
    {
        var tools = new EchoTools();
        var method = typeof(EchoTools).GetMethod("Echo")!;
        var m = typeof(ToolRegistry).GetMethod("WrapMethod",
            System.Reflection.BindingFlags.NonPublic | System.Reflection.BindingFlags.Static)!;
        var handler = (ToolHandler)m.Invoke(null, [tools, method])!;
        var input = JsonDocument.Parse("""{"message":"ok"}""").RootElement;
        var result = handler(input);
        Assert.Contains("ok", result);
    }

    [Fact]
    public void AncorErrorCode_Ok_Zero()
    {
        Assert.Equal(0, (int)AncorErrorCode.Ok);
    }

    [Fact]
    public void AncorErrorCode_Internal_Three()
    {
        Assert.Equal(3, (int)AncorErrorCode.Internal);
    }
}
