import { AgentSpec, AgentSpecSchema, RunEvent, RunEventSchema } from './schemas'

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
