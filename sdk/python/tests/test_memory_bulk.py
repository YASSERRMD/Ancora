"""Tests for MemoryStore.update and pop methods."""

from ancora.memory import MemoryStore


def test_update_writes_multiple_keys():
    mem = MemoryStore()
    mem.update({"a": 1, "b": 2, "c": 3})
    assert mem.read("a") == 1
    assert mem.read("b") == 2
    assert mem.read("c") == 3


def test_update_overwrites_existing():
    mem = MemoryStore()
    mem.write("x", "old")
    mem.update({"x": "new", "y": "fresh"})
    assert mem.read("x") == "new"
    assert mem.read("y") == "fresh"


def test_update_empty_mapping_is_noop():
    mem = MemoryStore()
    mem.write("k", "v")
    mem.update({})
    assert mem.read("k") == "v"
    assert len(mem) == 1


def test_pop_returns_value_and_removes_key():
    mem = MemoryStore()
    mem.write("k", 42)
    val = mem.pop("k")
    assert val == 42
    assert "k" not in mem


def test_pop_missing_returns_default():
    mem = MemoryStore()
    assert mem.pop("missing") is None


def test_pop_custom_default():
    mem = MemoryStore()
    assert mem.pop("x", default=99) == 99


def test_update_then_pop():
    mem = MemoryStore()
    mem.update({"a": 1, "b": 2})
    mem.pop("a")
    assert mem.keys == ["b"]
