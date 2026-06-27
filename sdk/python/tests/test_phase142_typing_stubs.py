"""Phase 142 task 19: typing stubs present and correct."""

import inspect
import ancora
from ancora.models import AgentSpec, ToolSpec
from ancora.tools import tool, ToolRegistry
from ancora.memory import MemoryStore


def test_runtime_class_is_exported():
    assert hasattr(ancora, "Runtime")


def test_agent_class_is_exported():
    assert hasattr(ancora, "Agent")


def test_agent_spec_builder_is_exported():
    assert hasattr(ancora, "AgentSpecBuilder")


def test_ancor_error_is_exported():
    assert hasattr(ancora, "AncorError")


def test_runtime_init_signature_has_no_required_args():
    sig = inspect.signature(ancora.Runtime.__init__)
    params = {
        k: v for k, v in sig.parameters.items()
        if k != "self" and v.default is inspect.Parameter.empty
    }
    assert len(params) == 0


def test_agent_spec_model_id_field_exists():
    spec = AgentSpec(name="test", model_id="llama3")
    assert hasattr(spec, "model_id")


def test_tool_spec_name_field_exists():
    ts = ToolSpec(name="my-tool", description="desc")
    assert hasattr(ts, "name")


def test_tool_registry_has_register_method():
    reg = ToolRegistry()
    assert callable(getattr(reg, "register", None))


def test_tool_registry_has_dispatch_method():
    reg = ToolRegistry()
    assert callable(getattr(reg, "dispatch", None))


def test_memory_store_has_write_and_read():
    mem = MemoryStore()
    assert callable(getattr(mem, "write", None))
    assert callable(getattr(mem, "read", None))


def test_tool_decorator_callable():
    assert callable(tool)


def test_agentspec_builder_has_build():
    builder = ancora.AgentSpecBuilder()
    assert callable(getattr(builder, "build", None))
