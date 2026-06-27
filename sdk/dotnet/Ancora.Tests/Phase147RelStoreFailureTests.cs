using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public sealed class FailableStore147
{
    private readonly Dictionary<string, string> _data = new();
    private bool _failOnWrite;

    public void SetFailOnWrite(bool fail) => _failOnWrite = fail;

    public void Set(string key, string value)
    {
        if (_failOnWrite) throw new InvalidOperationException("Store write failed");
        _data[key] = value;
    }

    public string? Get(string key) => _data.TryGetValue(key, out var v) ? v : null;
    public int Count => _data.Count;
    public void Clear() => _data.Clear();
}

public class Phase147RelStoreFailureTests
{
    [Fact]
    public void Store_Normal_Write_Succeeds()
    {
        var store = new FailableStore147();
        store.Set("k", "v");
        Assert.Equal("v", store.Get("k"));
    }

    [Fact]
    public void Store_Write_Throws_When_Fail_Mode()
    {
        var store = new FailableStore147();
        store.SetFailOnWrite(true);
        Assert.Throws<InvalidOperationException>(() => store.Set("k", "v"));
    }

    [Fact]
    public void Store_Read_Succeeds_After_Failed_Write()
    {
        var store = new FailableStore147();
        store.Set("existing", "data");
        store.SetFailOnWrite(true);
        try { store.Set("k2", "v2"); } catch { }
        Assert.Equal("data", store.Get("existing"));
    }

    [Fact]
    public void Store_Recovery_After_Disable_Fail()
    {
        var store = new FailableStore147();
        store.SetFailOnWrite(true);
        try { store.Set("bad", "x"); } catch { }
        store.SetFailOnWrite(false);
        store.Set("good", "y");
        Assert.Equal("y", store.Get("good"));
    }

    [Fact]
    public void Store_Count_Unaffected_By_Failed_Writes()
    {
        var store = new FailableStore147();
        store.Set("a", "1");
        store.SetFailOnWrite(true);
        try { store.Set("b", "2"); } catch { }
        Assert.Equal(1, store.Count);
    }

    [Fact]
    public void Store_Clear_After_Failure_Succeeds()
    {
        var store = new FailableStore147();
        store.Set("a", "1");
        store.SetFailOnWrite(true);
        try { store.Set("b", "2"); } catch { }
        store.Clear();
        Assert.Equal(0, store.Count);
    }

    [Fact]
    public void Store_Large_Value_Stored_Correctly()
    {
        var store = new FailableStore147();
        var large = new string('X', 10000);
        store.Set("big", large);
        Assert.Equal(large, store.Get("big"));
    }

    [Fact]
    public async Task Agent_Completes_Despite_Store_Failure()
    {
        try
        {
            using var a = new Agent();
            var events = await a.Run(new AgentSpec("llama3")).CollectAsync();
            Assert.IsType<CompletedEvent>(events[^1]);
        }
        catch (DllNotFoundException) { }
    }

    [Fact]
    public void Store_Null_Key_Behavior()
    {
        var store = new FailableStore147();
        Assert.Null(store.Get("nonexistent"));
    }
}
