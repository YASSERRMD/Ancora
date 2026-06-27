using System.Collections.Generic;
using System.Linq;
using Xunit;

namespace Ancora.Tests;

public sealed class InMemoryStore146
{
    private readonly Dictionary<string, string> _data = new();

    public void Set(string key, string value) => _data[key] = value;
    public string? Get(string key) => _data.TryGetValue(key, out var v) ? v : null;
    public bool Contains(string key) => _data.ContainsKey(key);
    public void Delete(string key) => _data.Remove(key);
    public int Count => _data.Count;
    public void Clear() => _data.Clear();
}

public class Phase146MemoryReadWriteTests
{
    [Fact]
    public void Store_Set_And_Get_Returns_Value()
    {
        var store = new InMemoryStore146();
        store.Set("key1", "value1");
        Assert.Equal("value1", store.Get("key1"));
    }

    [Fact]
    public void Store_Missing_Key_Returns_Null()
    {
        var store = new InMemoryStore146();
        Assert.Null(store.Get("missing"));
    }

    [Fact]
    public void Store_Overwrite_Key_Updates_Value()
    {
        var store = new InMemoryStore146();
        store.Set("k", "v1");
        store.Set("k", "v2");
        Assert.Equal("v2", store.Get("k"));
    }

    [Fact]
    public void Store_Contains_True_For_Existing_Key()
    {
        var store = new InMemoryStore146();
        store.Set("exists", "yes");
        Assert.True(store.Contains("exists"));
    }

    [Fact]
    public void Store_Contains_False_For_Missing_Key()
    {
        var store = new InMemoryStore146();
        Assert.False(store.Contains("ghost"));
    }

    [Fact]
    public void Store_Delete_Removes_Key()
    {
        var store = new InMemoryStore146();
        store.Set("tmp", "val");
        store.Delete("tmp");
        Assert.False(store.Contains("tmp"));
    }

    [Fact]
    public void Store_Count_Reflects_Entries()
    {
        var store = new InMemoryStore146();
        store.Set("a", "1");
        store.Set("b", "2");
        store.Set("c", "3");
        Assert.Equal(3, store.Count);
    }

    [Fact]
    public void Store_Clear_Empties_All_Entries()
    {
        var store = new InMemoryStore146();
        for (int i = 0; i < 10; i++) store.Set($"key-{i}", $"val-{i}");
        store.Clear();
        Assert.Equal(0, store.Count);
    }

    [Fact]
    public void Store_Five_Hundred_Ops_Succeed()
    {
        var store = new InMemoryStore146();
        for (int i = 0; i < 500; i++) store.Set($"k{i}", $"v{i}");
        for (int i = 0; i < 500; i++) Assert.Equal($"v{i}", store.Get($"k{i}"));
    }

    [Fact]
    public void Store_Keys_Are_Unique()
    {
        var store = new InMemoryStore146();
        for (int i = 0; i < 100; i++) store.Set($"unique-{i}", "x");
        Assert.Equal(100, store.Count);
    }
}
