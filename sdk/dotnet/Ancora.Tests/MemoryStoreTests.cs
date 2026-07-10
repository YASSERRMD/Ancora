using System.Collections.Generic;
using Ancora;
using Xunit;

namespace Ancora.Tests;

/// <summary>
/// Exercises the memory/vector-store surface added to close an
/// Attestra-readiness gap: pgvector was previously Rust-only, with no C ABI
/// functions and no .NET wrapper at all. These use the default runtime's
/// zero-dependency in-memory store (no Postgres needed to run this suite),
/// but exercise the exact same native call path a pgvector-configured
/// runtime uses. Like <c>Phase148SdkSurfaceTests</c>, these do not catch
/// <see cref="System.DllNotFoundException"/> defensively: CI builds the
/// native library before running tests, so a failure here is a real
/// regression.
/// </summary>
public class MemoryStoreTests
{
    [Fact]
    public void CreateCollection_Then_Query_Finds_Closest_Point()
    {
        using var runtime = new Runtime();
        runtime.CreateCollection("docs", dimensions: 2);
        runtime.Upsert("docs", new[]
        {
            new VectorPoint(1, new[] { 1.0f, 0.0f }, new Dictionary<string, object?> { ["text"] = "alpha" }),
            new VectorPoint(2, new[] { 0.0f, 1.0f }, new Dictionary<string, object?> { ["text"] = "beta" }),
        });

        var results = runtime.Query("docs", new[] { 1.0f, 0.0f }, topK: 1);

        var top = Assert.Single(results);
        Assert.Equal(1UL, top.Id);
        Assert.Equal("alpha", top.Payload["text"].GetString());
    }

    [Fact]
    public void Delete_Removes_Point_From_Query_Results()
    {
        using var runtime = new Runtime();
        runtime.CreateCollection("docs", dimensions: 2);
        runtime.Upsert("docs", new[]
        {
            new VectorPoint(1, new[] { 1.0f, 0.0f }),
            new VectorPoint(2, new[] { 0.9f, 0.1f }),
        });

        runtime.Delete("docs", new ulong[] { 1 });
        var results = runtime.Query("docs", new[] { 1.0f, 0.0f }, topK: 10);

        Assert.DoesNotContain(results, p => p.Id == 1);
        Assert.Contains(results, p => p.Id == 2);
    }

    [Fact]
    public void DropCollection_Then_Query_Throws()
    {
        using var runtime = new Runtime();
        runtime.CreateCollection("temp", dimensions: 2);
        runtime.DropCollection("temp");

        Assert.Throws<AncorException>(() => runtime.Query("temp", new[] { 1.0f, 0.0f }));
    }

    [Fact]
    public void Query_Respects_Score_Threshold()
    {
        using var runtime = new Runtime();
        runtime.CreateCollection("docs", dimensions: 2);
        runtime.Upsert("docs", new[]
        {
            new VectorPoint(1, new[] { 1.0f, 0.0f }), // cosine score 1.0 against itself
            new VectorPoint(2, new[] { 0.0f, 1.0f }), // orthogonal, cosine score 0.0
        });

        var results = runtime.Query("docs", new[] { 1.0f, 0.0f }, topK: 10, scoreThreshold: 0.5);

        Assert.Single(results);
        Assert.Equal(1UL, results[0].Id);
    }

    [Fact]
    public void MemoryConfig_Constructor_Falls_Back_To_In_Memory_Store_When_Unreachable()
    {
        // An unreachable pgvector URL falls back to the in-memory store
        // rather than throwing, matching ProviderConfig's error-tolerant
        // ancora_runtime_new_with_config behavior.
        using var runtime = new Runtime(new MemoryConfig("postgres://nobody@127.0.0.1:1/nope"));
        runtime.CreateCollection("docs", dimensions: 2);
        runtime.Upsert("docs", new[] { new VectorPoint(1, new[] { 1.0f, 0.0f }) });

        var results = runtime.Query("docs", new[] { 1.0f, 0.0f }, topK: 1);
        Assert.Single(results);
    }

    [Fact]
    public void Runtime_With_Provider_And_Memory_Both_Configured_Works()
    {
        using var runtime = new Runtime(
            new ProviderConfig(BaseUrl: "http://127.0.0.1:1"),
            new MemoryConfig("postgres://nobody@127.0.0.1:1/nope"));
        runtime.CreateCollection("docs", dimensions: 2);
        runtime.Upsert("docs", new[] { new VectorPoint(1, new[] { 1.0f, 0.0f }) });

        var results = runtime.Query("docs", new[] { 1.0f, 0.0f }, topK: 1);
        Assert.Single(results);
    }
}
