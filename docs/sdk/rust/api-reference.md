# API Reference (Rust)

## `ancora-core`

### `Runtime`

```rust
pub struct Runtime { /* opaque */ }

impl Runtime {
    pub fn new() -> anyhow::Result<Self>;
    pub fn with_options(opts: RuntimeOptions) -> anyhow::Result<Self>;
    pub async fn run(&self, spec: &AgentSpec, prompt: &str) -> anyhow::Result<Run>;
    pub async fn run_graph(&self, graph: &GraphSpec, prompt: &str) -> anyhow::Result<Run>;
    pub async fn resume(&self, run_id: &str) -> anyhow::Result<Run>;
}
```

### `RuntimeOptions`

```rust
pub struct RuntimeOptions {
    pub model_url: Option<String>,
    pub transport: Option<Box<dyn Transport>>,
    pub http_timeout: Option<Duration>,
    pub tracer: Option<BoxedTracer>,
}
```

### `AgentSpec`

```rust
pub struct AgentSpec {
    pub model: String,
    pub instructions: String,
    pub tools: Vec<ToolSpec>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub policy: Option<PolicySpec>,
}

impl AgentSpec {
    pub fn builder() -> AgentSpecBuilder;
}
```

### `Run`

```rust
pub struct Run { /* opaque */ }

impl Run {
    pub async fn next(&mut self) -> anyhow::Result<Option<RunEvent>>;
    pub async fn resume(&mut self, input: &str) -> anyhow::Result<()>;
    pub fn run_id(&self) -> &str;
}
```

### `RunEvent`

```rust
pub enum RunEvent {
    Started   { run_id: String },
    Token     { token: String },
    ToolCall  { name: String, input: serde_json::Value },
    Completed { output: String, usage: Usage },
    Suspended { reason: String },
    Resumed,
}
```

### `ToolSpec`

```rust
pub struct ToolSpec {
    pub name: String,
    pub description: String,
    pub schema: ToolInputSchema,
    pub effect: EffectClass,
    pub handler: BoxedHandler,
}

impl ToolSpec {
    pub fn builder() -> ToolSpecBuilder;
}
```

### `ToolInputSchema`

```rust
pub struct ToolInputSchema {
    pub type_: String,
    pub properties: serde_json::Value,
    pub required: Vec<String>,
}
```

### `EffectClass`

```rust
pub enum EffectClass {
    None,
    Read,
    Write,
}
```

### `PolicySpec`

```rust
pub struct PolicySpec {
    pub allow_regions: Vec<String>,
    pub deny_providers: Vec<String>,
    pub max_write_tools: Option<u32>,
    pub require_encryption: bool,
}

impl PolicySpec {
    pub fn builder() -> PolicySpecBuilder;
}
```

### `MemoryStore`

```rust
pub struct MemoryStore { /* opaque */ }

impl MemoryStore {
    pub fn new() -> Self;
    pub fn add(&mut self, id: &str, text: &str);
    pub fn keyword_search(&self, query: &str, limit: usize) -> Vec<Passage>;
}
```

### `GraphSpec`

```rust
pub struct GraphSpec {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub entry: String,
}

pub struct GraphNode {
    pub id: String,
    pub spec: AgentSpec,
}

pub struct GraphEdge {
    pub from: String,
    pub to: String,
}
```

### `Usage`

```rust
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

impl Usage {
    pub fn cost_usd(&self) -> f64;
}
```

## See also

- [Quickstart](quickstart.md)
- [Tools](tools.md)
