# TypeScript SDK API Reference

## `Runtime`

```ts
class Runtime {
  constructor(options?: { transport?: Transport; logLevel?: string }): Runtime

  run(spec: AgentSpec, prompt?: string, options?: RunOptions): Promise<RunResult>
  start(spec: AgentSpec, prompt?: string, options?: RunOptions): RunHandle
  resume(runId: string): RunHandle
  stream(spec: AgentSpec, prompt?: string, options?: StreamOptions): AsyncIterable<RunEvent>
  runGraph(graph: GraphSpec, prompt?: string): Promise<GraphResult>
  startGraph(graph: GraphSpec, prompt?: string): GraphHandle
}
```

## `buildSpec`

```ts
function buildSpec(options: {
  model: string
  instructions: string
  tools?: ToolRegistry | ToolSpec[]
  maxTokens?: number
  temperature?: number
  outputSchema?: ZodType
  policy?: PolicySpec
  mcpServers?: string[]
  modelUrl?: string
}): AgentSpec
```

**Note**: use `instructions:` (not `systemPrompt:`).

## `ToolRegistry`

```ts
class ToolRegistry {
  register<I extends ZodType>(options: {
    name: string
    description: string
    input: I
    fn: (input: z.infer<I>) => unknown | Promise<unknown>
    effect?: EffectClass
    idempotencyKeyTemplate?: string
  }): void
}
```

## `PolicySpec`

```ts
interface PolicySpec {
  allowRegions?: string[]
  denyProviders?: string[]
  maxWriteTools?: number
}
```

## `RunResult`

```ts
interface RunResult {
  runId: string
  output: string
  usage: TokenUsage
  parse<T>(schema: ZodType<T>): T
}
```

## `RunHandle`

```ts
class RunHandle {
  readonly runId: string
  readonly status: RunStatus
  readonly pauseReason: string | undefined

  collect(): Promise<RunResult>
  runUntilPause(): Promise<void>
  resume(payload: string): Promise<void>
  resumeBytes(payload: Buffer): Promise<void>
  events(): AsyncIterable<RunEvent>
}
```

## `RunEvent` union

```ts
type RunEvent =
  | { type: 'started'; runId: string }
  | { type: 'token'; token: string }
  | { type: 'tool_call'; name: string; input: unknown }
  | { type: 'completed'; output: string; usage: TokenUsage }
  | { type: 'resumed'; runId: string }
  | { type: 'error'; message: string }
```

## `SqliteStore` / `MemoryStore`

```ts
class SqliteStore { constructor(path: string) }
class MemoryStore { hasRun(runId: string): boolean }
```

## `buildGraph`

```ts
function buildGraph(options: {
  nodes: Array<{ id: string; model: string; instructions: string; tools?: ToolRegistry }>
  edges: Array<{ from: string; to: string }>
}): GraphSpec
```
