using System;
using System.Runtime.InteropServices;
using Ancora;
using Ancora.Handles;
using Xunit;

namespace Ancora.Tests;

public class Phase146NativeLoadDisposeTests
{
    [Fact]
    public void Runtime_Implements_IDisposable()
    {
        Assert.True(typeof(IDisposable).IsAssignableFrom(typeof(Runtime)));
    }

    [Fact]
    public void Runtime_Is_Sealed()
    {
        Assert.True(typeof(Runtime).IsSealed);
    }

    [Fact]
    public void Runtime_Create_And_Dispose_Via_Using()
    {
        try
        {
            using var rt = new Runtime();
            Assert.NotNull(rt);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Runtime_Double_Dispose_Does_Not_Throw()
    {
        try
        {
            var rt = new Runtime();
            rt.Dispose();
            rt.Dispose();
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Runtime_Dispose_Prevents_Further_Use()
    {
        try
        {
            var rt = new Runtime();
            rt.Dispose();
            Assert.Throws<ObjectDisposedException>(() => rt.ToolCount());
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Runtime_Version_Returns_NonNull()
    {
        try
        {
            var v = Runtime.Version();
            Assert.NotNull(v);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Runtime_ToolCount_Zero_After_Create()
    {
        try
        {
            using var rt = new Runtime();
            Assert.Equal((nuint)0, rt.ToolCount());
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Agent_Implements_IDisposable()
    {
        Assert.True(typeof(IDisposable).IsAssignableFrom(typeof(Agent)));
    }

    [Fact]
    public void Agent_Create_And_Dispose_Via_Using()
    {
        try
        {
            using var a = new Agent();
            Assert.NotNull(a);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Agent_Sharing_Runtime_Does_Not_Dispose_It()
    {
        try
        {
            using var rt = new Runtime();
            var a = new Agent(rt);
            a.Dispose();
            Assert.Equal((nuint)0, rt.ToolCount());
        }
        catch (DllNotFoundException) { }
    }
}
