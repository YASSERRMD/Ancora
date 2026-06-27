jest.mock('../ancora.node', () => ({}), { virtual: true })
jest.mock('../../ancora.node', () => ({}), { virtual: true })

import { Agent, buildSpec, collectEvents, tokenText } from '../../index'
import { Runtime } from '../../index'
import { RunEvent } from '../../schemas'

// Minimal offline corpus -- stands in for a real Qdrant collection.
interface Passage {
  id: string
  text: string
  source: string
}

function keywordRetrieve(corpus: Passage[], query: string, topK: number): Passage[] {
  const words = query.toLowerCase().split(/\s+/)
  const scored = corpus.map(p => {
    const lower = p.text.toLowerCase()
    const score = words.filter(w => lower.includes(w)).length
    return { passage: p, score }
  })
  return scored
    .filter(s => s.score > 0)
    .sort((a, b) => b.score - a.score)
    .slice(0, topK)
    .map(s => s.passage)
}

function makeRagRuntime(contextPassages: Passage[]): Runtime {
  let counter = 0
  const runs = new Map<string, RunEvent[]>()
  return {
    startRun(spec: string | Uint8Array): string {
      const id = `rag-${++counter}`
      const s = typeof spec === 'string' ? spec : new TextDecoder().decode(spec)
      const context = contextPassages.map(p => p.text).join('\n')
      runs.set(id, [
        { kind: 'started', run_id: id, spec: s },
        { kind: 'token', run_id: id, text: `Context: ${context.slice(0, 40)}` },
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

const corpus: Passage[] = [
  { id: '1', text: 'Ancora is a multi-backend agent runtime for Rust and Go.', source: 'docs/overview.md' },
  { id: '2', text: 'Supported backends include pgvector, qdrant, weaviate, and lancedb.', source: 'docs/backends.md' },
  { id: '3', text: 'The embedders module provides offline hash-based embedders.', source: 'docs/embeddings.md' },
  { id: '4', text: 'Qdrant is a vector search engine with REST and gRPC APIs.', source: 'docs/qdrant.md' },
]

describe('rag-qdrant example smoke test', () => {
  it('keywordRetrieve returns relevant passages', () => {
    const hits = keywordRetrieve(corpus, 'qdrant backends', 3)
    expect(hits.length).toBeGreaterThanOrEqual(1)
    expect(hits.some(h => h.source.includes('backends') || h.source.includes('qdrant'))).toBe(true)
  })

  it('keywordRetrieve respects topK limit', () => {
    const hits = keywordRetrieve(corpus, 'ancora qdrant backends embedders', 2)
    expect(hits.length).toBeLessThanOrEqual(2)
  })

  it('keywordRetrieve returns empty for unrelated query', () => {
    const hits = keywordRetrieve(corpus, 'zxqyuv unrelated', 3)
    expect(hits).toEqual([])
  })

  it('agent runs with injected RAG context', async () => {
    const query = 'what backends does ancora support'
    const hits = keywordRetrieve(corpus, query, 3)
    const context = hits.map(h => `[${h.source}] ${h.text}`).join('\n---\n')
    const rt = makeRagRuntime(hits)
    const agent = new Agent(rt)
    const spec = buildSpec('claude', {
      instructions: `Use this context to answer:\n${context}`,
    })
    const events = await collectEvents(agent.run(spec))
    expect(events.some(e => e.kind === 'started')).toBe(true)
    expect(events.some(e => e.kind === 'completed')).toBe(true)
    agent.free()
  })

  it('retrieved token includes context prefix', async () => {
    const hits = keywordRetrieve(corpus, 'qdrant vector search', 2)
    const rt = makeRagRuntime(hits)
    const agent = new Agent(rt)
    const events = await collectEvents(agent.run(buildSpec('claude')))
    const text = tokenText(events)
    expect(text.startsWith('Context:')).toBe(true)
    agent.free()
  })
})
