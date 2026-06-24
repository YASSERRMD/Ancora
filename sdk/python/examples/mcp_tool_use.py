"""MCP-style tool use example.

Demonstrates registering multiple tools (as if they were MCP tools) with an
agent and dispatching tool calls by name. Runs fully offline.

Usage::

    python -m examples.mcp_tool_use
"""

import asyncio
import json
from typing import Optional

import ancora
from ancora.models import EffectClass
from ancora.tools import ToolRegistry, tool


@tool(effect_class=EffectClass.READ)
def search_web(query: str, limit: int = 5) -> list:
    """Search the web for a query. Returns a list of result titles."""
    return [f"Result {i + 1} for '{query}'" for i in range(limit)]


@tool(effect_class=EffectClass.READ)
def get_weather(location: str, units: str = "celsius") -> str:
    """Get current weather for a location."""
    return f"Weather in {location}: 22 {units}, partly cloudy"


@tool(effect_class=EffectClass.WRITE)
def send_email(to: str, subject: str, body: str) -> bool:
    """Send an email (simulated -- no actual email sent)."""
    print(f"  [email] to={to!r} subject={subject!r}")
    return True


@tool(effect_class=EffectClass.PURE)
def calculate(expression: str) -> float:
    """Evaluate a simple arithmetic expression."""
    allowed = set("0123456789 +-*/().")
    if not all(c in allowed for c in expression):
        raise ValueError(f"Unsafe expression: {expression!r}")
    return float(eval(expression))  # noqa: S307


async def main() -> None:
    registry = ToolRegistry()
    for t in [search_web, get_weather, send_email, calculate]:
        registry.register(t)

    print(f"registered tools: {registry.names}")

    results = registry.dispatch("search_web", json.dumps({"query": "Ancora", "limit": 3}))
    print(f"search: {results}")

    weather = registry.dispatch("get_weather", json.dumps({"location": "London"}))
    print(f"weather: {weather}")

    total = registry.dispatch("calculate", json.dumps({"expression": "2 + 2 * 3"}))
    print(f"calculate: {total}")

    rt = ancora.Runtime()
    spec = ancora.AgentSpec(
        name="tool-use-agent",
        model_id="local-model",
        tools=registry.all_specs(),
    )
    agent = ancora.Agent(rt, spec, registry=registry)
    run = await agent.run()
    await run.drain_events()
    print(f"agent spec tools: {[t.name for t in agent.spec.tools]}")

    rt.free()


if __name__ == "__main__":
    asyncio.run(main())
