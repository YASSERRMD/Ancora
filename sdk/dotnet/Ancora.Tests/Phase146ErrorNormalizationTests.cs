using System;
using Ancora;
using Ancora.Interop;
using Xunit;

namespace Ancora.Tests;

public class Phase146ErrorNormalizationTests
{
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

    [Fact]
    public void AncorErrorCode_Ok_Not_Equal_Internal()
    {
        Assert.NotEqual(AncorErrorCode.Ok, AncorErrorCode.Internal);
    }

    [Fact]
    public void AncorErrorCode_NullPtr_Not_Equal_Ok()
    {
        Assert.NotEqual(AncorErrorCode.Ok, AncorErrorCode.NullPtr);
    }

    [Fact]
    public void AncorErrorCode_InvalidUtf8_Not_Equal_Ok()
    {
        Assert.NotEqual(AncorErrorCode.Ok, AncorErrorCode.InvalidUtf8);
    }

    [Fact]
    public void AncorException_ErrorCode_Zero_Is_Ok()
    {
        var ex = new AncorException(0, "success?");
        Assert.Equal(0, ex.ErrorCode);
    }

    [Fact]
    public void AncorException_ErrorCode_Matches_Internal()
    {
        var ex = new AncorException((int)AncorErrorCode.Internal, "internal error");
        Assert.Equal((int)AncorErrorCode.Internal, ex.ErrorCode);
    }

    [Fact]
    public void AncorBuffer_IsEmpty_When_Zero_Ptr()
    {
        var buf = new AncorBuffer { Ptr = IntPtr.Zero, Len = 0 };
        Assert.True(buf.IsEmpty);
    }

    [Fact]
    public void AncorBuffer_IsEmpty_When_Zero_Len()
    {
        var buf = new AncorBuffer { Ptr = new IntPtr(1), Len = 0 };
        Assert.True(buf.IsEmpty);
    }

    [Fact]
    public void AncorBuffer_Not_Empty_When_Ptr_And_Len_Set()
    {
        var buf = new AncorBuffer { Ptr = new IntPtr(1), Len = 1 };
        Assert.False(buf.IsEmpty);
    }
}
