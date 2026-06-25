using System;

namespace Ancora;

/// <summary>
/// Exception raised when an Ancora FFI function returns a non-Ok error code.
/// </summary>
public sealed class AncorException : Exception
{
    /// <summary>
    /// The integer error code from the FFI call.
    /// </summary>
    public int ErrorCode { get; }

    public AncorException(int errorCode, string message)
        : base($"{message} (ErrorCode={errorCode})")
    {
        ErrorCode = errorCode;
    }
}
