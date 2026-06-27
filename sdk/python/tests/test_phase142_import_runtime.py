"""Phase 142 task 1: import and runtime create."""

import pytest
import ancora


def test_import_ancora():
    assert ancora is not None


def test_import_version():
    assert ancora.version() != ""


def test_runtime_create_and_free():
    rt = ancora.Runtime()
    assert rt is not None
    rt.free()
    assert rt.is_freed


def test_runtime_context_manager():
    with ancora.Runtime() as rt:
        assert not rt.is_freed


def test_runtime_is_freed_after_context_manager():
    with ancora.Runtime() as rt:
        pass
    assert rt.is_freed


def test_runtime_multiple_instances():
    rt1 = ancora.Runtime()
    rt2 = ancora.Runtime()
    rt1.free()
    rt2.free()
    assert rt1.is_freed
    assert rt2.is_freed


def test_runtime_free_is_idempotent():
    rt = ancora.Runtime()
    rt.free()
    rt.free()
    assert rt.is_freed


def test_runtime_exports_expected_names():
    expected = ["Runtime", "Agent", "AgentSpec", "ToolSpec", "tool", "MemoryStore"]
    for name in expected:
        assert hasattr(ancora, name), f"ancora.{name} not found"


def test_runtime_version_is_string():
    v = ancora.version()
    assert isinstance(v, str)


def test_ancora_error_is_importable():
    assert ancora.AncorError is not None
