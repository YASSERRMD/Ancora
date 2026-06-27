"""Phase 142 task 18: error normalization."""

import pytest
import ancora
from ancora.models import AgentSpec


def test_ancor_error_ok_is_zero():
    assert ancora.AncorError.ErrOk == 0


def test_ancor_error_internal_is_nonzero():
    assert ancora.AncorError.ErrInternal != ancora.AncorError.ErrOk


def test_ancor_error_not_found_is_nonzero():
    assert ancora.AncorError.ErrNotFound != ancora.AncorError.ErrOk


def test_ancor_error_invalid_arg_is_nonzero():
    assert ancora.AncorError.ErrInvalidArg != ancora.AncorError.ErrOk


def test_ancor_error_codes_are_distinct():
    codes = [
        ancora.AncorError.ErrOk,
        ancora.AncorError.ErrInternal,
        ancora.AncorError.ErrNotFound,
        ancora.AncorError.ErrInvalidArg,
    ]
    assert len(set(codes)) == len(codes)


def test_ancor_error_ok_is_int():
    assert isinstance(ancora.AncorError.ErrOk, int)


def test_ancor_error_internal_is_int():
    assert isinstance(ancora.AncorError.ErrInternal, int)


def test_agentspec_invalid_name_raises():
    with pytest.raises(Exception):
        AgentSpec(name="", model_id="llama3")


def test_agentspec_invalid_model_raises():
    with pytest.raises(Exception):
        AgentSpec(name="agent", model_id="")


@pytest.mark.asyncio
async def test_freed_runtime_run_raises_error():
    rt = ancora.Runtime()
    rt.free()
    spec = AgentSpec(name="freed-rt", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    with pytest.raises(Exception):
        await agent.run()
