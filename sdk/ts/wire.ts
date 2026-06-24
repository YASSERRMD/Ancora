import { AgentSpec, AgentSpecSchema, RunEvent, RunEventSchema, ToolSpec } from './schemas'

export function encodeSpec(spec: AgentSpec): Buffer {
  return Buffer.from(JSON.stringify(spec), 'utf8')
}

export function decodeSpec(bytes: Buffer | Uint8Array): AgentSpec {
  const raw = JSON.parse(Buffer.from(bytes).toString('utf8'))
  return AgentSpecSchema.parse(raw)
}

export function parseEvent(bytes: Buffer | string): RunEvent {
  const raw = typeof bytes === 'string' ? JSON.parse(bytes) : JSON.parse(bytes.toString('utf8'))
  return RunEventSchema.parse(raw)
}

export function validateSpec(raw: unknown): { ok: true; spec: AgentSpec } | { ok: false; errors: string[] } {
  const result = AgentSpecSchema.safeParse(raw)
  if (result.success) return { ok: true, spec: result.data }
  return { ok: false, errors: result.error.issues.map((i) => `${i.path.join('.')}: ${i.message}`) }
}

export function buildSpec(
  model: string,
  options: { instructions?: string; tools?: ToolSpec[]; maxTokens?: number; temperature?: number } = {}
): AgentSpec {
  return AgentSpecSchema.parse({
    model,
    instructions: options.instructions,
    tools: options.tools,
    max_tokens: options.maxTokens,
    temperature: options.temperature,
  })
}
