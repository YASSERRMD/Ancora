"""In-process key-value memory store for Ancora agents."""

from __future__ import annotations

from typing import Any, Iterator, Optional


class MemoryStore:
    """Thread-safe in-memory key-value store for agent state.

    Values are arbitrary Python objects. Keys are strings. The store is
    scoped to the Python process -- it is not persisted to disk.

    Typical usage::

        from ancora.memory import MemoryStore

        mem = MemoryStore()
        mem.write("user", "Alice")
        name = mem.read("user")        # "Alice"
        mem.delete("user")
        mem.write("count", 0)
        mem.clear()
    """

    def __init__(self) -> None:
        self._store: dict[str, Any] = {}

    def read(self, key: str, default: Any = None) -> Any:
        """Return the value for *key*, or *default* if absent."""
        return self._store.get(key, default)

    def write(self, key: str, value: Any) -> None:
        """Store *value* under *key*, overwriting any existing value."""
        self._store[key] = value

    def delete(self, key: str) -> None:
        """Remove *key* from the store. No-op if key is absent."""
        self._store.pop(key, None)

    def clear(self) -> None:
        """Remove all keys from the store."""
        self._store.clear()

    @property
    def keys(self) -> list[str]:
        """Return a snapshot of all keys in insertion order."""
        return list(self._store.keys())

    @property
    def values(self) -> list[Any]:
        """Return a snapshot of all values in insertion order."""
        return list(self._store.values())

    def __len__(self) -> int:
        return len(self._store)

    def __contains__(self, key: str) -> bool:
        return key in self._store

    def __iter__(self) -> Iterator[str]:
        return iter(self._store)

    def __repr__(self) -> str:
        return f"MemoryStore(keys={self.keys!r})"
