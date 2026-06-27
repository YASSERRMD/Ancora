jest.mock('../ancora.node', () => ({}), { virtual: true })
jest.mock('../../ancora.node', () => ({}), { virtual: true })

import { Agent, buildSpec, collectEvents, tokenText } from '../../index'
import { Runtime } from '../../index'
import { RunEvent } from '../../schemas'

function makeOfflineRuntime(agentTokens: Record<string, string[]>): Runtime {
  let counter = 0
  const runs = new Map<string, RunEvent[]>()
  const agentNames = Object.keys(agentTokens)
  return {
    startRun(spec: string | Uint8Array): string {
      const idx = counter % agentNames.length
      const name = agentNames[idx] ?? 'default'
      const id = `run-${name}-${++counter}`
      const s = typeof spec === 'string' ? spec : new TextDecoder().decode(spec)
      const tokens = agentTokens[name] ?? []
      runs.set(id, [
        { kind: 'started', run_id: id, spec: s },
        ...tokens.map(t => ({ kind: 'token' as const, run_id: id, text: t })),
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

describe('multi-agent-verifier example smoke test', () => {
  it('runs two agents concurrently and collects events from both', async () => {
    const rt = makeOfflineRuntime({
      'primary': ['I am the primary agent.'],
      'verifier': ['Verified: primary output is correct.'],
    })
    const primaryAgent = new Agent(rt)
    const verifierAgent = new Agent(rt)

    const primarySpec = buildSpec('claude', { instructions: 'You are the primary agent.' })
    const verifierSpec = buildSpec('claude', { instructions: 'Verify the primary agent output.' })

    const [primaryEvents, verifierEvents] = await Promise.all([
      collectEvents(primaryAgent.run(primarySpec)),
      collectEvents(verifierAgent.run(verifierSpec)),
    ])

    expect(primaryEvents.some(e => e.kind === 'started')).toBe(true)
    expect(verifierEvents.some(e => e.kind === 'started')).toBe(true)
    expect(primaryEvents.some(e => e.kind === 'completed')).toBe(true)
    expect(verifierEvents.some(e => e.kind === 'completed')).toBe(true)

    primaryAgent.free()
    verifierAgent.free()
  })

  it('primary and verifier produce distinct run IDs', async () => {
    const rt = makeOfflineRuntime({
      'primary': ['answer'],
      'verifier': ['ok'],
    })
    const primary = new Agent(rt)
    const verifier = new Agent(rt)

    const ph = primary.run(buildSpec('model'))
    const vh = verifier.run(buildSpec('model'))

    expect(ph.runId).not.toBe(vh.runId)

    await collectEvents(ph)
    await collectEvents(vh)
    primary.free()
    verifier.free()
  })

  it('token text from each agent is independent', async () => {
    const rt = makeOfflineRuntime({
      'primary': ['primary-response'],
      'verifier': ['verifier-response'],
    })
    const primary = new Agent(rt)
    const verifier = new Agent(rt)

    const [pEvents, vEvents] = await Promise.all([
      collectEvents(primary.run(buildSpec('model'))),
      collectEvents(verifier.run(buildSpec('model'))),
    ])

    expect(tokenText(pEvents)).toBe('primary-response')
    expect(tokenText(vEvents)).toBe('verifier-response')
    primary.free()
    verifier.free()
  })
})
