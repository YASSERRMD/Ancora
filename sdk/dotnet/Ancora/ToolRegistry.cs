using System;
using System.Collections.Generic;
using System.Reflection;
using System.Runtime.ExceptionServices;
using System.Text.Json;
using System.Text.RegularExpressions;
using Ancora.Interop;

namespace Ancora;

/// <summary>
/// Central registry for associating managed tool handlers with an Ancora runtime.
/// Tools registered here are invoked by the core when the model calls them.
/// </summary>
public static class ToolRegistry
{
    /// <summary>
    /// Register a named tool with a managed handler delegate.
    /// Returns an IDisposable that unregisters the tool when disposed.
    /// The returned value must stay alive as long as the tool is needed.
    /// </summary>
    public static IDisposable Register(
        Runtime runtime,
        string name,
        string description,
        ToolHandler handler)
    {
        ArgumentNullException.ThrowIfNull(runtime);
        ArgumentNullException.ThrowIfNull(name);
        ArgumentNullException.ThrowIfNull(handler);

        var bridge = new ToolBridge(handler);
        runtime.RegisterCallback(name, bridge.NativeCallback);
        return new ToolDisposable(runtime, name, bridge);
    }

    /// <summary>
    /// Discover all [Tool]-decorated public methods on target and register each one.
    /// Returns a list pairing each ToolSpec with its IDisposable registration.
    /// Disposing a registration unregisters that tool.
    /// </summary>
    public static IReadOnlyList<(ToolSpec Spec, IDisposable Registration)> RegisterAll(
        Runtime runtime,
        object target)
    {
        ArgumentNullException.ThrowIfNull(runtime);
        ArgumentNullException.ThrowIfNull(target);

        var results = new List<(ToolSpec, IDisposable)>();
        var type = target.GetType();

        foreach (var method in type.GetMethods(BindingFlags.Public | BindingFlags.Instance))
        {
            var attr = method.GetCustomAttribute<ToolAttribute>();
            if (attr is null) continue;

            var name = attr.Name ?? ToSnakeCase(method.Name);
            var schema = BuildSchema(method);
            var spec = new ToolSpec(name, attr.Description, schema);
            var handler = WrapMethod(target, method);
            var reg = Register(runtime, name, attr.Description, handler);
            results.Add((spec, reg));
        }

        return results;
    }

    // --- private helpers ---

    private static ToolInputSchema? BuildSchema(MethodInfo method)
    {
        var parameters = method.GetParameters();
        if (parameters.Length == 0) return null;

        var props = new Dictionary<string, ToolInputProperty>();
        var required = new List<string>();

        foreach (var param in parameters)
        {
            var attr = param.GetCustomAttribute<ToolInputAttribute>();
            var jsonType = TypeToJsonType(param.ParameterType);
            props[param.Name!] = new ToolInputProperty(jsonType, attr?.Description);
            if (attr?.Required != false)
                required.Add(param.Name!);
        }

        return new ToolInputSchema(Properties: props, Required: required);
    }

    private static string TypeToJsonType(Type type)
    {
        if (type == typeof(string)) return "string";
        if (type == typeof(int) || type == typeof(long) || type == typeof(short)) return "integer";
        if (type == typeof(double) || type == typeof(float) || type == typeof(decimal)) return "number";
        if (type == typeof(bool)) return "boolean";
        return "object";
    }

    private static ToolHandler WrapMethod(object target, MethodInfo method)
    {
        var parameters = method.GetParameters();
        return (JsonElement input) =>
        {
            var args = new object?[parameters.Length];
            for (int i = 0; i < parameters.Length; i++)
            {
                var param = parameters[i];
                if (input.TryGetProperty(param.Name!, out var value))
                {
                    args[i] = JsonSerializer.Deserialize(value.GetRawText(), param.ParameterType);
                }
                else
                {
                    args[i] = param.HasDefaultValue ? param.DefaultValue : null;
                }
            }
            object? result;
            try
            {
                result = method.Invoke(target, args);
            }
            catch (TargetInvocationException tie) when (tie.InnerException is not null)
            {
                ExceptionDispatchInfo.Capture(tie.InnerException).Throw();
                throw; // unreachable, satisfies compiler
            }
            return result is string s ? s : JsonSerializer.Serialize(result);
        };
    }

    private static string ToSnakeCase(string name)
    {
        return Regex.Replace(name, @"(?<=[a-z0-9])([A-Z])", "_$1").ToLowerInvariant();
    }
}

/// <summary>
/// Manages the lifetime of a single tool callback in the native runtime.
/// Disposing unregisters the tool and releases the delegate.
/// </summary>
internal sealed class ToolBridge : IDisposable
{
    private readonly AncorToolCallback _nativeCallback;
    private bool _disposed;

    internal ToolBridge(ToolHandler handler)
    {
        _nativeCallback = CreateNativeCallback(handler);
    }

    internal AncorToolCallback NativeCallback => _nativeCallback;

    private static unsafe AncorToolCallback CreateNativeCallback(ToolHandler handler)
    {
        return (byte* input, nuint inputLen, AncorBuffer* outBuf) =>
        {
            try
            {
                var span = new ReadOnlySpan<byte>(input, (int)inputLen);
                var element = JsonSerializer.Deserialize<JsonElement>(span);
                var result = handler(element);
                var bytes = System.Text.Encoding.UTF8.GetBytes(result);
                fixed (byte* p = bytes)
                    *outBuf = AncoraNative.ancora_buffer_new(p, (nuint)bytes.Length);
                return AncorErrorCode.Ok;
            }
            catch
            {
                *outBuf = default;
                return AncorErrorCode.Internal;
            }
        };
    }

    public void Dispose() { _disposed = true; }
}

/// <summary>
/// Ties tool lifetime to a Runtime registration: disposes the bridge and unregisters.
/// </summary>
internal sealed class ToolDisposable : IDisposable
{
    private readonly Runtime _runtime;
    private readonly string _name;
    private readonly ToolBridge _bridge;
    private bool _disposed;

    internal ToolDisposable(Runtime runtime, string name, ToolBridge bridge)
    {
        _runtime = runtime;
        _name = name;
        _bridge = bridge;
    }

    public void Dispose()
    {
        if (_disposed) return;
        _runtime.UnregisterCallback(_name);
        _bridge.Dispose();
        _disposed = true;
    }
}
