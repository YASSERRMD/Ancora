namespace Ancora;

/// <summary>
/// Points a <see cref="Runtime"/> at a real OpenAI-compatible chat-completions
/// endpoint (hosted or self-hosted, including NVIDIA NIM) instead of the
/// offline echo model client used by the parameterless <see cref="Runtime"/>
/// constructor.
/// </summary>
/// <param name="BaseUrl">
/// Provider root URL, e.g. <c>"https://api.openai.com"</c> or
/// <c>"https://integrate.api.nvidia.com/v1"</c> for NVIDIA NIM (hosted or
/// self-hosted; switching endpoints is a <c>BaseUrl</c> change only).
/// </param>
/// <param name="AuthEnvVar">
/// Environment variable holding the bearer token, e.g.
/// <c>"OPENAI_API_KEY"</c> or <c>"NVIDIA_API_KEY"</c>. Omit for
/// unauthenticated endpoints (e.g. a local self-hosted container).
/// </param>
/// <param name="ChatCompletionsPath">
/// Overrides the default <c>"/v1/chat/completions"</c> completions path.
/// Needed when <c>BaseUrl</c> already carries a <c>/v1</c> segment, as
/// NVIDIA NIM's documented base URL does (set this to
/// <c>"/chat/completions"</c> in that case).
/// </param>
public sealed record ProviderConfig(
    string BaseUrl,
    string? AuthEnvVar = null,
    string? ChatCompletionsPath = null
);

/// <summary>
/// A typed cost summary for a run, parsed from the FFI's raw JSON.
/// </summary>
/// <param name="RunId">The run this cost summary belongs to.</param>
/// <param name="TotalUsd">Total accumulated cost in USD across every model
/// call in the run. 0 for offline (no-provider-configured) runs, or when
/// the model's pricing metadata isn't registered.</param>
public sealed record Cost(string RunId, double TotalUsd);
