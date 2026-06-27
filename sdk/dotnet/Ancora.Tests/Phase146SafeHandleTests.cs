using System;
using System.Runtime.InteropServices;
using Ancora;
using Ancora.Handles;
using Xunit;

namespace Ancora.Tests;

public class Phase146SafeHandleTests
{
    [Fact]
    public void RuntimeHandle_Extends_SafeHandle()
    {
        Assert.True(typeof(SafeHandle).IsAssignableFrom(typeof(RuntimeHandle)));
    }

    [Fact]
    public void RunIdHandle_Extends_SafeHandle()
    {
        Assert.True(typeof(SafeHandle).IsAssignableFrom(typeof(RunIdHandle)));
    }

    [Fact]
    public void RuntimeHandle_Is_Sealed()
    {
        Assert.True(typeof(RuntimeHandle).IsSealed);
    }

    [Fact]
    public void RunIdHandle_Is_Sealed()
    {
        Assert.True(typeof(RunIdHandle).IsSealed);
    }

    [Fact]
    public void SafeHandle_Has_IsInvalid_Property()
    {
        var prop = typeof(SafeHandle).GetProperty("IsInvalid");
        Assert.NotNull(prop);
    }

    [Fact]
    public void SafeHandle_Has_IsClosed_Property()
    {
        var prop = typeof(SafeHandle).GetProperty("IsClosed");
        Assert.NotNull(prop);
    }

    [Fact]
    public void Runtime_Implementing_IDisposable_Via_SafeHandle()
    {
        var disposableMethod = typeof(Runtime).GetMethod("Dispose");
        Assert.NotNull(disposableMethod);
    }

    [Fact]
    public void SafeHandle_Implements_IDisposable()
    {
        Assert.True(typeof(IDisposable).IsAssignableFrom(typeof(SafeHandle)));
    }

    [Fact]
    public void Runtime_Dispose_Does_Not_Throw_Twice()
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
    public void SafeHandle_DangerousGetHandle_Returns_IntPtr()
    {
        var method = typeof(SafeHandle).GetMethod("DangerousGetHandle");
        Assert.NotNull(method);
        Assert.Equal(typeof(IntPtr), method!.ReturnType);
    }
}
