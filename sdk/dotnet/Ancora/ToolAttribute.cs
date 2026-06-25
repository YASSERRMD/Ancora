using System;

namespace Ancora;

/// <summary>
/// Marks a method as an Ancora tool.
/// The method will be discovered by <see cref="ToolRegistry.RegisterAll"/>.
/// </summary>
[AttributeUsage(AttributeTargets.Method, Inherited = false, AllowMultiple = false)]
public sealed class ToolAttribute : Attribute
{
    /// <summary>
    /// Prose description sent to the model so it knows when to call the tool.
    /// </summary>
    public string Description { get; }

    /// <summary>
    /// Optional override for the tool name.
    /// If null, the method name in snake_case is used.
    /// </summary>
    public string? Name { get; }

    public ToolAttribute(string description, string? name = null)
    {
        Description = description ?? throw new ArgumentNullException(nameof(description));
        Name = name;
    }
}

/// <summary>
/// Annotates a parameter of a [Tool] method with metadata for the JSON Schema.
/// </summary>
[AttributeUsage(AttributeTargets.Parameter, Inherited = false, AllowMultiple = false)]
public sealed class ToolInputAttribute : Attribute
{
    /// <summary>
    /// Description of this parameter shown in the tool schema.
    /// </summary>
    public string? Description { get; }

    /// <summary>
    /// Whether this parameter is required. Defaults to true.
    /// </summary>
    public bool Required { get; init; } = true;

    public ToolInputAttribute(string? description = null)
    {
        Description = description;
    }
}
