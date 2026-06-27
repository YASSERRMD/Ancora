# Python SDK API Reference

## `Runtime`

```python
class Runtime:
    def __init__(
        self,
        transport: Transport | None = None,
    ) -> None: ...

    def run(
        self,
        spec: AgentSpec,
        prompt: str = "",
        run_id: str | None = None,
    ) -> RunResult: ...

    def start(
        self,
        spec: AgentSpec,
        prompt: str = "",
        run_id: str | None = None,
    ) -> RunHandle: ...

    def resume(self, run_id: str) -> RunHandle: ...

    def stream(
        self,
        spec: AgentSpec,
        prompt: str = "",
    ) -> Iterator[RunEvent]: ...

    async def stream_async(
        self,
        spec: AgentSpec,
        prompt: str = "",
    ) -> AsyncIterator[RunEvent]: ...

    def run_graph(
        self,
        graph: GraphSpec,
        prompt: str = "",
    ) -> GraphResult: ...
```

## `AgentSpec`

```python
@dataclass
class AgentSpec:
    model: str
    instructions: str
    tools: ToolRegistry | list[ToolSpec] | None = None
    max_tokens: int = 4096
    temperature: float = 0.7
    output_schema: type[BaseModel] | None = None
    policy: PolicySpec | None = None
    mcp_servers: list[str] = field(default_factory=list)
    model_url: str | None = None
```

## `ToolRegistry`

```python
class ToolRegistry:
    def tool(
        self,
        description: str,
        effect: EffectClass = EffectClass.NONE,
    ) -> Callable: ...

    def register(
        self,
        name: str,
        fn: Callable,
        description: str,
        effect: EffectClass = EffectClass.NONE,
    ) -> None: ...
```

## `ToolSpec`

```python
class ToolSpec:
    @staticmethod
    def from_callable(
        name: str,
        fn: Callable,
        description: str,
        effect: EffectClass = EffectClass.NONE,
        idempotency_key_template: str | None = None,
    ) -> ToolSpec: ...

    @staticmethod
    def schema_from_model(model: type[BaseModel]) -> dict: ...
```

## `PolicySpec`

```python
@dataclass
class PolicySpec:
    allow_regions: list[str] = field(default_factory=list)
    deny_providers: list[str] = field(default_factory=list)
    max_write_tools: int = 0
```

## `RunResult`

```python
@dataclass
class RunResult:
    run_id: str
    output: str
    usage: TokenUsage

    def parse(self, model: type[BaseModel]) -> BaseModel: ...
```

## `RunHandle`

```python
class RunHandle:
    def collect(self) -> RunResult: ...
    async def collect_async(self) -> RunResult: ...
    def run_until_pause(self) -> RunStatus: ...
    async def run_until_pause_async(self) -> RunStatus: ...
    def resume(self, payload: str) -> None: ...
    def resume_bytes(self, payload: bytes) -> None: ...
    async def resume_async(self, payload: str) -> None: ...
    def events(self) -> Iterator[RunEvent]: ...
```

## `SqliteStore` / `MemoryStore`

```python
class SqliteStore:
    def __init__(self, path: str) -> None: ...
    def has_run(self, run_id: str) -> bool: ...

class MemoryStore:
    def has_run(self, run_id: str) -> bool: ...
```

## `GraphSpec`

```python
@dataclass
class GraphSpec:
    nodes: list[GraphNode]
    edges: list[GraphEdge]

@dataclass
class GraphNode:
    id: str
    spec: AgentSpec

@dataclass
class GraphEdge:
    from_node: str
    to_node: str
```
