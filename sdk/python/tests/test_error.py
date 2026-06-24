"""Tests for ancora.AncorError."""

import pytest
import ancora


def test_ancor_error_is_exception():
    assert issubclass(ancora.AncorError, Exception)


def test_ancor_error_can_be_raised():
    with pytest.raises(ancora.AncorError):
        raise ancora.AncorError("test error")


def test_ancor_error_message():
    with pytest.raises(ancora.AncorError) as exc_info:
        raise ancora.AncorError("something went wrong")
    assert "something went wrong" in str(exc_info.value)


def test_ancor_error_caught_as_exception():
    try:
        raise ancora.AncorError("oops")
    except Exception as e:
        assert "oops" in str(e)
    else:
        pytest.fail("AncorError not caught as Exception")
