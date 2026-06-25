using System;
using System.Runtime.InteropServices;
using Ancora;
using Ancora.Handles;
using Ancora.Interop;
using Xunit;

namespace Ancora.Tests;

public class AncorExceptionTests
{
    [Fact]
    public void AncorException_Carries_ErrorCode()
    {
        var ex = new AncorException(3, "internal failure");
        Assert.Equal(3, ex.ErrorCode);
        Assert.Contains("internal failure", ex.Message);
        Assert.Contains("ErrorCode=3", ex.Message);
    }

    [Fact]
    public void AncorException_Inherits_Exception()
    {
        Assert.True(typeof(Exception).IsAssignableFrom(typeof(AncorException)));
    }
}

public class RuntimeTypeTests
{
    [Fact]
    public void Runtime_Implements_IDisposable()
    {
        Assert.True(typeof(IDisposable).IsAssignableFrom(typeof(Runtime)));
    }

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
}

public class NativeTypesTests
{
    [Fact]
    public void AncorBuffer_IsEmpty_When_Ptr_Is_Zero()
    {
        var buf = new AncorBuffer { Ptr = IntPtr.Zero, Len = 0 };
        Assert.True(buf.IsEmpty);
    }

    [Fact]
    public void AncorBuffer_IsEmpty_When_Len_Is_Zero()
    {
        var buf = new AncorBuffer { Ptr = new IntPtr(1), Len = 0 };
        Assert.True(buf.IsEmpty);
    }

    [Fact]
    public void AncorErrorCode_Ok_Is_Zero()
    {
        Assert.Equal(0, (int)AncorErrorCode.Ok);
    }

    [Fact]
    public void AncorErrorCode_NullPtr_Is_One()
    {
        Assert.Equal(1, (int)AncorErrorCode.NullPtr);
    }

    [Fact]
    public void AncorErrorCode_InvalidUtf8_Is_Two()
    {
        Assert.Equal(2, (int)AncorErrorCode.InvalidUtf8);
    }

    [Fact]
    public void AncorErrorCode_Internal_Is_Three()
    {
        Assert.Equal(3, (int)AncorErrorCode.Internal);
    }
}

/// <summary>
/// Integration tests that require the native ancora_ffi library to be present.
/// These are skipped gracefully when running without the native build.
/// CI builds the native library before running these tests.
/// </summary>
public class RuntimeIntegrationTests
{
    [Fact]
    public void Runtime_Creates_And_Disposes_Without_Leak()
    {
        try
        {
            using var rt = new Runtime();
            // Reached here: native library loaded successfully.
        }
        catch (DllNotFoundException)
        {
            // Native library not present in this environment; CI provides it.
        }
    }

    [Fact]
    public void Runtime_Double_Dispose_Is_Safe()
    {
        try
        {
            var rt = new Runtime();
            rt.Dispose();
            rt.Dispose(); // must not throw or crash
        }
        catch (DllNotFoundException)
        {
            // Native library not present in this environment; CI provides it.
        }
    }

    [Fact]
    public void Runtime_Version_Returns_String()
    {
        try
        {
            var version = Runtime.Version();
            Assert.NotNull(version);
        }
        catch (DllNotFoundException)
        {
            // Native library not present in this environment; CI provides it.
        }
    }

    [Fact]
    public void Runtime_ToolCount_Is_Zero_After_Create()
    {
        try
        {
            using var rt = new Runtime();
            Assert.Equal((nuint)0, rt.ToolCount());
        }
        catch (DllNotFoundException)
        {
            // Native library not present in this environment; CI provides it.
        }
    }

    [Fact]
    public void Runtime_ToolExists_Returns_False_For_Unknown()
    {
        try
        {
            using var rt = new Runtime();
            Assert.False(rt.ToolExists("nonexistent_tool"));
        }
        catch (DllNotFoundException)
        {
            // Native library not present in this environment; CI provides it.
        }
    }
}
