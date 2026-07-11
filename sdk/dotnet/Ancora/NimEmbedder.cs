using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.Net.Http.Headers;
using System.Text;
using System.Text.Json;
using System.Text.Json.Serialization;
using System.Threading;
using System.Threading.Tasks;

namespace Ancora;

/// <summary>
/// Points a <see cref="NimEmbedder"/> at a real embeddings endpoint (hosted
/// NVIDIA NIM or any OpenAI-compatible <c>/embeddings</c> server, including a
/// self-hosted NIM container -- switching is a <see cref="BaseUrl"/> change
/// only).
/// </summary>
/// <param name="BaseUrl">
/// Provider root URL, e.g. <c>"https://integrate.api.nvidia.com/v1"</c> for
/// hosted NVIDIA NIM, or <c>"http://localhost:8000/v1"</c> for a self-hosted
/// NIM container.
/// </param>
/// <param name="Model">
/// Embedding model identifier, e.g. <c>"nvidia/nv-embedqa-e5-v5"</c> (a NeMo
/// Retriever model).
/// </param>
/// <param name="AuthEnvVar">
/// Environment variable holding the bearer token, e.g. <c>"NVIDIA_API_KEY"</c>
/// (NIM keys are prefixed <c>nvapi-</c>). Omit for unauthenticated endpoints
/// (e.g. a local self-hosted container).
/// </param>
/// <param name="EmbeddingsPath">
/// Overrides the default <c>"/embeddings"</c> path, for endpoints that need
/// a different suffix.
/// </param>
public sealed record EmbedderConfig(
    string BaseUrl,
    string Model,
    string? AuthEnvVar = null,
    string EmbeddingsPath = "/embeddings"
);

/// <summary>
/// Distinguishes how a text will be used, for embedding models (like NIM's
/// NeMo Retriever models) that produce asymmetric passage/query embeddings.
/// </summary>
public enum EmbedInputType
{
    /// <summary>A document chunk being indexed for later retrieval.</summary>
    Passage,
    /// <summary>A search query being embedded to retrieve matching passages.</summary>
    Query,
}

/// <summary>
/// A real HTTP client for an OpenAI-compatible embeddings endpoint (NVIDIA
/// NIM's NeMo Retriever models, hosted or self-hosted). Independent of
/// <see cref="Runtime"/> -- pair its output with <see cref="Runtime.Upsert"/>
/// and <see cref="Runtime.Query"/> to build a retrieval pipeline.
/// </summary>
public sealed class NimEmbedder : IDisposable
{
    private readonly HttpClient _http;
    private readonly EmbedderConfig _config;
    private bool _disposed;

    public NimEmbedder(EmbedderConfig config, HttpClient? httpClient = null)
    {
        ArgumentNullException.ThrowIfNull(config);
        _config = config;
        _http = httpClient ?? new HttpClient();

        if (config.AuthEnvVar is { } envVar)
        {
            var key = Environment.GetEnvironmentVariable(envVar);
            if (!string.IsNullOrEmpty(key))
                _http.DefaultRequestHeaders.Authorization = new AuthenticationHeaderValue("Bearer", key);
        }
    }

    /// <summary>
    /// Embed a single text. <paramref name="inputType"/> should match how
    /// the text will be used: <see cref="EmbedInputType.Passage"/> when
    /// indexing a document chunk, <see cref="EmbedInputType.Query"/> when
    /// embedding a search query, for models that produce asymmetric
    /// passage/query embeddings.
    /// </summary>
    public async Task<float[]> EmbedAsync(
        string text, EmbedInputType inputType = EmbedInputType.Passage, CancellationToken cancellationToken = default)
    {
        var results = await EmbedBatchAsync(new[] { text }, inputType, cancellationToken).ConfigureAwait(false);
        return results[0];
    }

    /// <summary>
    /// Embed a batch of texts in a single request.
    /// </summary>
    public async Task<IReadOnlyList<float[]>> EmbedBatchAsync(
        IReadOnlyList<string> texts,
        EmbedInputType inputType = EmbedInputType.Passage,
        CancellationToken cancellationToken = default)
    {
        ThrowIfDisposed();
        ArgumentNullException.ThrowIfNull(texts);
        if (texts.Count == 0) return Array.Empty<float[]>();

        var request = new EmbeddingsRequest(
            texts.ToArray(),
            _config.Model,
            inputType == EmbedInputType.Query ? "query" : "passage");

        var url = _config.BaseUrl.TrimEnd('/') + _config.EmbeddingsPath;
        using var httpResponse = await _http
            .PostAsync(url, JsonContent(request), cancellationToken)
            .ConfigureAwait(false);

        var body = await httpResponse.Content.ReadAsStringAsync(cancellationToken).ConfigureAwait(false);
        if (!httpResponse.IsSuccessStatusCode)
        {
            throw new AncorException(
                (int)httpResponse.StatusCode,
                $"embeddings request failed with status {(int)httpResponse.StatusCode}: {body}");
        }

        var parsed = JsonSerializer.Deserialize<EmbeddingsResponse>(body, JsonOptions)
            ?? throw new AncorException(0, "embeddings response could not be parsed");

        return parsed.Data
            .OrderBy(d => d.Index)
            .Select(d => d.Embedding)
            .ToList();
    }

    public void Dispose()
    {
        if (_disposed) return;
        _http.Dispose();
        _disposed = true;
    }

    private void ThrowIfDisposed() => ObjectDisposedException.ThrowIf(_disposed, this);

    private static StringContent JsonContent(EmbeddingsRequest request)
    {
        var json = JsonSerializer.Serialize(request, JsonOptions);
        return new StringContent(json, Encoding.UTF8, "application/json");
    }

    private static readonly JsonSerializerOptions JsonOptions = new()
    {
        PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower,
        DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull,
    };

    private sealed record EmbeddingsRequest(string[] Input, string Model, string InputType);

    private sealed record EmbeddingsResponse(List<EmbeddingData> Data);

    private sealed record EmbeddingData(float[] Embedding, int Index);
}
