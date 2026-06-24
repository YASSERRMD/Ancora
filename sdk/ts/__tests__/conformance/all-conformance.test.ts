import { parseEvent, encodeSpec, decodeSpec, buildSpec, validateSpec } from '../../wire'
import { tokenText, collectEvents } from '../../helpers'
import { z } from 'zod'
import { defineTool, ToolRegistry } from '../../tools'
import { ToolBridge, RunHandleLike, ToolBridgeEvent } from '../../tool-bridge'
import { RunEvent } from '../../schemas'

function makeHandle(events: RunEvent[]): RunHandleLike {
  const q = [...events]
  return {
    runId: 'all-conf',
    async *[Symbol.asyncIterator]() { for (const e of q) yield e },
    resume() {},
  }
}

describe('all conformance scenarios', () => {
  describe('spec round-trip', () => {
    it('buildSpec -> encodeSpec -> decodeSpec -> model matches', () => {
      const spec = buildSpec('claude-3-5-sonnet', { maxTokens: 2048 })
      expect(decodeSpec(encodeSpec(spec)).model).toBe('claude-3-5-sonnet')
    })
  })

  describe('event parsing', () => {
    it.each([
      ['started', '{"kind":"started","run_id":"r","spec":"{}"}'],
      ['token', '{"kind":"token","run_id":"r","text":"t"}'],
      ['completed', '{"kind":"completed","run_id":"r"}'],
      ['resumed', '{"kind":"resumed","run_id":"r","decision":"{}"}'],
      ['tool_call', '{"kind":"tool_call","run_id":"r","name":"fn","input":"{}"}'],
    ])('parses %s event', (_kind, json) => {
      expect(() => parseEvent(json)).not.toThrow()
    })
  })

  describe('tokenText', () => {
    it('returns empty string for no token events', () => {
      expect(tokenText([{ kind: 'completed', run_id: 'r' }])).toBe('')
    })

    it('concatenates multiple tokens', () => {
      expect(tokenText([
        { kind: 'token', run_id: 'r', text: 'a' },
        { kind: 'token', run_id: 'r', text: 'b' },
        { kind: 'token', run_id: 'r', text: 'c' },
      ])).toBe('abc')
    })
  })

  describe('validateSpec', () => {
    it('ok: true for valid spec', () => {
      expect(validateSpec({ model: 'x' }).ok).toBe(true)
    })

    it('ok: false for missing model', () => {
      expect(validateSpec({}).ok).toBe(false)
    })
  })

  describe('tool pipeline', () => {
    it('defineTool + ToolRegistry + ToolBridge pipeline', async () => {
      const sq = defineTool({
        name: 'square',
        description: 'square a number',
        schema: z.object({ n: z.number() }),
        handler: ({ n }) => n * n,
      })
      const bridge = new ToolBridge(new ToolRegistry().register(sq) as ToolRegistry)
      const events: ToolBridgeEvent[] = []
      for await (const ev of bridge.run(makeHandle([
        { kind: 'tool_call', run_id: 'p', name: 'square', input: '{"n":9}' },
        { kind: 'completed', run_id: 'p' },
      ]))) {
        events.push(ev)
      }
      const tr = events.find(e => e.kind === 'tool_result') as { kind: 'tool_result'; result: unknown } | undefined
      expect(tr).toBeDefined()
      expect(tr?.result).toBe(81)
    })
  })

  describe('collectEvents', () => {
    it('collects all events from an async iterable', async () => {
      const handle = makeHandle([
        { kind: 'started', run_id: 'c', spec: '{}' },
        { kind: 'token', run_id: 'c', text: 'x' },
        { kind: 'completed', run_id: 'c' },
      ])
      const all = await collectEvents(handle)
      expect(all).toHaveLength(3)
    })
  })
})
