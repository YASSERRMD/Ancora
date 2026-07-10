using System;
using System.IO;
using System.Reflection;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

namespace Ancora.Interop;

/// <summary>
/// Resolves the native <c>ancora_ffi</c> library explicitly instead of relying
/// on the OS default search path (which requires callers to set
/// <c>LD_LIBRARY_PATH</c>/<c>DYLD_LIBRARY_PATH</c>/<c>PATH</c> themselves).
///
/// Probes, in order: the directory the SDK assembly is loaded from (covers
/// <c>dotnet run</c>/local builds where the native library was copied next
/// to the managed output), then the NuGet <c>runtimes/&lt;rid&gt;/native/</c>
/// layout relative to that same directory (covers a manually-extracted or
/// self-contained-published package where the host's own native asset
/// resolution didn't already find it). Falls through to the OS default
/// resolver (returning <see cref="IntPtr.Zero"/>) if neither probe hits, so
/// this never makes a previously-working setup stop working.
/// </summary>
internal static class NativeLibraryResolver
{
    private const string LibraryName = "ancora_ffi";

    // A library-wide resolver must run before the first P/Invoke call, and
    // there is no library entry point to hook into for that -- a module
    // initializer is the standard, intended way to do this for exactly this
    // scenario, not just application/source-generator code.
#pragma warning disable CA2255
    [ModuleInitializer]
    internal static void Register()
    {
        NativeLibrary.SetDllImportResolver(typeof(NativeLibraryResolver).Assembly, Resolve);
    }
#pragma warning restore CA2255

    private static IntPtr Resolve(string libraryName, Assembly assembly, DllImportSearchPath? searchPath)
    {
        if (libraryName != LibraryName)
            return IntPtr.Zero;

        var baseDir = AppContext.BaseDirectory;
        foreach (var candidate in CandidatePaths(baseDir))
        {
            if (File.Exists(candidate) &&
                NativeLibrary.TryLoad(candidate, out var handle))
            {
                return handle;
            }
        }

        // Nothing found via explicit probing; let the OS default resolver
        // (PATH / LD_LIBRARY_PATH / DYLD_LIBRARY_PATH / standard NuGet
        // runtime asset resolution) take over.
        return IntPtr.Zero;
    }

    private static System.Collections.Generic.IEnumerable<string> CandidatePaths(string baseDir)
    {
        var fileName = PlatformFileName();
        // Next to the assembly (dotnet run / local build output).
        yield return Path.Combine(baseDir, fileName);
        // NuGet runtimes/<rid>/native/ layout, in case the host's own
        // resolution didn't already pick this up.
        foreach (var rid in CandidateRids())
        {
            yield return Path.Combine(baseDir, "runtimes", rid, "native", fileName);
        }
    }

    private static string PlatformFileName()
    {
        if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
            return "ancora_ffi.dll";
        if (RuntimeInformation.IsOSPlatform(OSPlatform.OSX))
            return "libancora_ffi.dylib";
        return "libancora_ffi.so";
    }

    private static System.Collections.Generic.IEnumerable<string> CandidateRids()
    {
        var arch = RuntimeInformation.ProcessArchitecture switch
        {
            Architecture.X64 => "x64",
            Architecture.Arm64 => "arm64",
            var other => other.ToString().ToLowerInvariant(),
        };
        if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
            yield return $"win-{arch}";
        else if (RuntimeInformation.IsOSPlatform(OSPlatform.OSX))
            yield return $"osx-{arch}";
        else
            yield return $"linux-{arch}";
    }
}
