using System.Text.Json;

namespace Ancora;

/// <summary>
/// Deserializes a run's final text output into a typed structured-output
/// value, for agents configured with an <c>OutputSchemaJson</c>.
/// </summary>
public static class StructuredOutputExtensions
{
    /// <summary>
    /// Parse <see cref="CompletedEvent.Output"/> as JSON into
    /// <typeparamref name="T"/>. Property matching is case-insensitive
    /// since the model produces the output's naming convention, not this
    /// SDK's wire protocol.
    /// </summary>
    /// <exception cref="JsonException">Thrown if Output is not valid JSON
    /// or does not match the shape of <typeparamref name="T"/>.</exception>
    public static T? Deserialize<T>(this CompletedEvent completed) =>
        JsonSerializer.Deserialize<T>(completed.Output, Wire.StructuredOutputOptions);
}
