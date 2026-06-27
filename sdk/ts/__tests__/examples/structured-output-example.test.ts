jest.mock('../ancora.node', () => ({}), { virtual: true })
jest.mock('../../ancora.node', () => ({}), { virtual: true })

import { z } from 'zod'
import { Agent, buildSpec, collectEvents, tokenText } from '../../index'
import { Runtime } from '../../index'
import { RunEvent } from '../../schemas'
import { zodToInputSchema } from '../../zod-to-schema'

// The schema that defines the expected agent output shape.
const AnalysisResultSchema = z.object({
  summary: z.string().describe('One-sentence summary of the analysis'),
  topics: z.array(z.string()).describe('List of main topics identified'),
  confidence: z.number().describe('Confidence score between 0.0 and 1.0'),
  action_item: z.string().describe('Recommended next action'),
})

type AnalysisResult = z.infer<typeof AnalysisResultSchema>

function makeOfflineRuntime(responseJson: string): Runtime {
  let counter = 0
  const runs = new Map<string, RunEvent[]>()
  return {
    startRun(spec: string | Uint8Array): string {
      const id = `struct-${++counter}`
      const s = typeof spec === 'string' ? spec : new TextDecoder().decode(spec)
      runs.set(id, [
        { kind: 'started', run_id: id, spec: s },
        { kind: 'token', run_id: id, text: responseJson },
        { kind: 'completed', run_id: id },
      ])
      return id
    },
    pollRun(id: string): string | null {
      const q = runs.get(id)
      if (!q || q.length === 0) return null
      return JSON.stringify(q.shift())
    },
    resumeRun() {},
    free() {},
    get isFreed() { return false },
  } as unknown as Runtime
}

describe('structured-output example smoke test', () => {
  const sampleResult: AnalysisResult = {
    summary: 'Ancora is a multi-backend agent runtime.',
    topics: ['agents', 'backends', 'embeddings'],
    confidence: 0.92,
    action_item: 'Review the pgvector backend configuration.',
  }

  it('zodToInputSchema extracts all required fields', () => {
    const schema = z.object({
      label: z.string().describe('Primary label'),
      score: z.number().describe('Score 0-1'),
    })
    const inputSchema = zodToInputSchema(schema)
    expect(inputSchema.required).toContain('label')
    expect(inputSchema.required).toContain('score')
    expect(inputSchema.properties['label'].type).toBe('string')
    expect(inputSchema.properties['score'].type).toBe('number')
  })

  it('zodToInputSchema preserves field descriptions', () => {
    const schema = z.object({
      summary: z.string().describe('A short description'),
    })
    const inputSchema = zodToInputSchema(schema)
    expect(inputSchema.properties['summary'].description).toBe('A short description')
  })

  it('agent runs and returns structured JSON token', async () => {
    const jsonResponse = JSON.stringify(sampleResult)
    const rt = makeOfflineRuntime(jsonResponse)
    const agent = new Agent(rt)
    const spec = buildSpec('claude-3-5-sonnet', {
      instructions: `Respond with JSON matching the AnalysisResult schema.`,
    })
    const events = await collectEvents(agent.run(spec))
    const text = tokenText(events)
    const parsed = JSON.parse(text) as AnalysisResult
    expect(parsed.summary).toBe(sampleResult.summary)
    expect(parsed.confidence).toBeCloseTo(0.92, 2)
    agent.free()
  })

  it('zod schema validates a conforming object', () => {
    const result = AnalysisResultSchema.safeParse(sampleResult)
    expect(result.success).toBe(true)
  })

  it('zod schema rejects object missing required field', () => {
    const bad = { summary: 'ok', confidence: 0.5 }
    const result = AnalysisResultSchema.safeParse(bad)
    expect(result.success).toBe(false)
  })

  it('AnalysisResultSchema has four required properties', () => {
    const keys = Object.keys(AnalysisResultSchema.shape)
    expect(keys).toContain('summary')
    expect(keys).toContain('topics')
    expect(keys).toContain('confidence')
    expect(keys).toContain('action_item')
  })

  it('zodToInputSchema marks all non-optional fields as required', () => {
    const schema = z.object({
      required_field: z.string(),
      optional_field: z.string().optional(),
    })
    const inputSchema = zodToInputSchema(schema)
    expect(inputSchema.required).toContain('required_field')
    expect(inputSchema.required).not.toContain('optional_field')
  })
})
