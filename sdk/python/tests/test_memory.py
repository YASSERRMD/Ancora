"""Tests for MemoryStore read, write, delete, and clear."""

import pytest

from ancora.memory import MemoryStore


def test_write_and_read():
    mem = MemoryStore()
    mem.write("key", "value")
    assert mem.read("key") == "value"


def test_read_missing_returns_default():
    mem = MemoryStore()
    assert mem.read("missing") is None


def test_read_custom_default():
    mem = MemoryStore()
    assert mem.read("x", default=42) == 42


def test_overwrite_existing():
    mem = MemoryStore()
    mem.write("k", 1)
    mem.write("k", 2)
    assert mem.read("k") == 2


def test_delete_removes_key():
    mem = MemoryStore()
    mem.write("k", "v")
    mem.delete("k")
    assert "k" not in mem


def test_delete_missing_is_noop():
    mem = MemoryStore()
    mem.delete("never_added")


def test_clear_empties_store():
    mem = MemoryStore()
    mem.write("a", 1)
    mem.write("b", 2)
    mem.clear()
    assert len(mem) == 0
    assert mem.keys == []


def test_keys_property_order():
    mem = MemoryStore()
    mem.write("z", 3)
    mem.write("a", 1)
    mem.write("m", 2)
    assert mem.keys == ["z", "a", "m"]


def test_values_property():
    mem = MemoryStore()
    mem.write("a", 10)
    mem.write("b", 20)
    assert mem.values == [10, 20]


def test_len():
    mem = MemoryStore()
    assert len(mem) == 0
    mem.write("x", 1)
    assert len(mem) == 1
    mem.write("y", 2)
    assert len(mem) == 2
    mem.delete("x")
    assert len(mem) == 1


def test_contains():
    mem = MemoryStore()
    mem.write("present", True)
    assert "present" in mem
    assert "absent" not in mem


def test_iter():
    mem = MemoryStore()
    mem.write("a", 1)
    mem.write("b", 2)
    assert list(mem) == ["a", "b"]


def test_repr():
    mem = MemoryStore()
    mem.write("x", 1)
    assert "x" in repr(mem)


def test_stores_arbitrary_objects():
    mem = MemoryStore()
    obj = {"nested": [1, 2, 3]}
    mem.write("data", obj)
    assert mem.read("data") is obj
