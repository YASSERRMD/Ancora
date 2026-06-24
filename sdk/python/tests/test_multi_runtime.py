"""Tests for multiple concurrent Runtime instances."""

import ancora


def test_two_runtimes_independent():
    rt1 = ancora.Runtime()
    rt2 = ancora.Runtime()
    assert not rt1.is_freed
    assert not rt2.is_freed
    rt1.free()
    assert rt1.is_freed
    assert not rt2.is_freed
    rt2.free()


def test_nested_context_managers():
    with ancora.Runtime() as rt1:
        with ancora.Runtime() as rt2:
            assert not rt1.is_freed
            assert not rt2.is_freed
        assert rt2.is_freed
        assert not rt1.is_freed
    assert rt1.is_freed


def test_many_runtimes_created_and_freed():
    runtimes = [ancora.Runtime() for _ in range(10)]
    for rt in runtimes:
        assert not rt.is_freed
    for rt in runtimes:
        rt.free()
    for rt in runtimes:
        assert rt.is_freed
