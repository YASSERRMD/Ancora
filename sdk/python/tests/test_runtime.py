"""Tests for ancora.Runtime."""

import ancora


def test_import():
    assert ancora is not None


def test_runtime_create():
    rt = ancora.Runtime()
    assert rt is not None
    rt.free()


def test_runtime_not_freed_after_create():
    rt = ancora.Runtime()
    assert not rt.is_freed
    rt.free()


def test_runtime_free():
    rt = ancora.Runtime()
    rt.free()
    assert rt.is_freed


def test_runtime_free_idempotent():
    rt = ancora.Runtime()
    rt.free()
    rt.free()
    assert rt.is_freed


def test_runtime_repr_active():
    rt = ancora.Runtime()
    assert repr(rt) == "Runtime(active)"
    rt.free()


def test_runtime_repr_freed():
    rt = ancora.Runtime()
    rt.free()
    assert repr(rt) == "Runtime(freed)"


def test_runtime_context_manager():
    with ancora.Runtime() as rt:
        assert not rt.is_freed
    assert rt.is_freed


def test_runtime_context_manager_repr():
    with ancora.Runtime() as rt:
        assert repr(rt) == "Runtime(active)"
    assert repr(rt) == "Runtime(freed)"


def test_version_returns_string():
    v = ancora.version()
    assert isinstance(v, str)
    assert len(v) > 0


def test_version_semver_format():
    v = ancora.version()
    parts = v.split(".")
    assert len(parts) == 3
    for part in parts:
        assert part.isdigit()


def test_ancor_error_is_exception():
    assert issubclass(ancora.AncorError, Exception)
