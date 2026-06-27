"""Phase 142 task 10: memory read write."""

import pytest
from ancora.memory import MemoryStore


def test_memory_store_write_and_read():
    mem = MemoryStore()
    mem.write("key1", "value1")
    assert mem.read("key1") == "value1"


def test_memory_store_read_missing_returns_default():
    mem = MemoryStore()
    assert mem.read("nonexistent") is None


def test_memory_store_read_with_custom_default():
    mem = MemoryStore()
    assert mem.read("missing", default="fallback") == "fallback"


def test_memory_store_delete():
    mem = MemoryStore()
    mem.write("del-key", 42)
    mem.delete("del-key")
    assert mem.read("del-key") is None


def test_memory_store_delete_nonexistent_is_noop():
    mem = MemoryStore()
    mem.delete("never-written")


def test_memory_store_clear():
    mem = MemoryStore()
    mem.write("a", 1)
    mem.write("b", 2)
    mem.clear()
    assert mem.read("a") is None
    assert mem.read("b") is None


def test_memory_store_update():
    mem = MemoryStore()
    mem.update({"x": 10, "y": 20})
    assert mem.read("x") == 10
    assert mem.read("y") == 20


def test_memory_store_overwrite():
    mem = MemoryStore()
    mem.write("k", "first")
    mem.write("k", "second")
    assert mem.read("k") == "second"


def test_memory_store_keys():
    mem = MemoryStore()
    mem.write("one", 1)
    mem.write("two", 2)
    assert "one" in mem.keys
    assert "two" in mem.keys


def test_memory_store_values():
    mem = MemoryStore()
    mem.write("a", 100)
    mem.write("b", 200)
    assert 100 in mem.values
    assert 200 in mem.values


def test_memory_store_pop():
    mem = MemoryStore()
    mem.write("pop-key", "pop-value")
    result = mem.pop("pop-key")
    assert result == "pop-value"
    assert mem.read("pop-key") is None


def test_memory_store_pop_missing_returns_default():
    mem = MemoryStore()
    result = mem.pop("missing", "default")
    assert result == "default"


def test_memory_store_stores_any_python_object():
    mem = MemoryStore()
    mem.write("list", [1, 2, 3])
    mem.write("dict", {"a": 1})
    mem.write("none", None)
    assert mem.read("list") == [1, 2, 3]
    assert mem.read("dict") == {"a": 1}
    assert mem.read("none") is None
