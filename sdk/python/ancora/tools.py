"""Decorator-based tool registration for Ancora agents."""

from __future__ import annotations

import functools
import inspect
import json
from typing import Any, Callable, Dict, Optional

from ancora.models import EffectClass, ToolSpec
from ancora.schema import params_to_schema


class Tool:
    """A registered tool wrapping a Python callable.

    Holds the :class:`~ancora.models.ToolSpec` derived from the function
    signature and delegates calls to the original function.
    """

    def __init__(self, fn: Callable[..., Any], spec: ToolSpec) -> None:
        self._fn = fn
        self._spec = spec
        functools.update_wrapper(self, fn)

    @property
    def spec(self) -> ToolSpec:
        """Return the ToolSpec for this tool."""
        return self._spec

    @property
    def name(self) -> str:
        """Return the tool name."""
        return self._spec.name

    def __call__(self, *args: Any, **kwargs: Any) -> Any:
        return self._fn(*args, **kwargs)

    def call_with_json(self, args_json: str) -> Any:
        """Invoke the tool from a JSON args string."""
        parsed = json.loads(args_json) if args_json else {}
        return self._fn(**parsed)

    def call_with_kwargs(self, **kwargs: Any) -> Any:
        """Invoke the tool with explicit keyword arguments."""
        return self._fn(**kwargs)

    def __repr__(self) -> str:
        return f"Tool(name={self.name!r})"


class ToolRegistry:
    """Registry of decorated tools keyed by name."""

    def __init__(self) -> None:
        self._tools: Dict[str, Tool] = {}

    def register(self, tool: Tool) -> None:
        """Add a tool to the registry."""
        self._tools[tool.name] = tool

    def get(self, name: str) -> Optional[Tool]:
        """Return the tool for name, or None."""
        return self._tools.get(name)

    def dispatch(self, name: str, args_json: str = "{}") -> Any:
        """Dispatch a tool call by name with JSON args."""
        t = self._tools.get(name)
        if t is None:
            raise KeyError(f"No tool registered with name {name!r}")
        return t.call_with_json(args_json)

    def all_specs(self) -> list[ToolSpec]:
        """Return all tool specs in registration order."""
        return [t.spec for t in self._tools.values()]

    @property
    def names(self) -> list[str]:
        """Return all registered tool names in registration order."""
        return list(self._tools.keys())

    async def adispatch(self, name: str, args_json: str = "{}") -> Any:
        """Async dispatch a tool call. Supports both sync and async callables."""
        import asyncio
        t = self._tools.get(name)
        if t is None:
            raise KeyError(f"No tool registered with name {name!r}")
        result = t.call_with_json(args_json)
        if asyncio.iscoroutine(result):
            return await result
        return result

    def __len__(self) -> int:
        return len(self._tools)

    def __contains__(self, name: str) -> bool:
        return name in self._tools


def tool(
    fn: Optional[Callable[..., Any]] = None,
    *,
    name: Optional[str] = None,
    description: Optional[str] = None,
    effect_class: EffectClass = EffectClass.UNSPECIFIED,
) -> Any:
    """Decorator that registers a function as an Ancora tool.

    Generates a :class:`~ancora.models.ToolSpec` from the function's name,
    docstring, and type-annotated parameters.

    Example::

        from ancora.tools import tool

        @tool
        def search(query: str) -> str:
            \"\"\"Search the web for a query.\"\"\"
            return f"results for {query}"

        print(search.spec.name)         # "search"
        print(search.spec.description)  # "Search the web for a query."
    """
    def decorator(f: Callable[..., Any]) -> Tool:
        tool_name = name or f.__name__
        tool_description = description or (inspect.getdoc(f) or "")
        sig = inspect.signature(f)
        input_schema = params_to_schema(sig)
        spec = ToolSpec(
            name=tool_name,
            description=tool_description,
            input_schema_json=json.dumps(input_schema),
            effect_class=effect_class,
        )
        return Tool(f, spec)

    if fn is not None:
        return decorator(fn)
    return decorator
