import { z } from 'zod'
import { defineTool, ToolRegistry } from '../tools'

const QDRANT_FIXTURE = [
  { id: 'qdrant-1', text: 'Qdrant is a vector database written in Rust.', score: 0.96 },
  { id: 'qdrant-2', text: 'Qdrant supports payload filtering alongside vector search.', score: 0.91 },
  { id: 'qdrant-3', text: 'Qdrant uses HNSW indexing for fast approximate search.', score: 0.85 },
]

const qdrantRetrieve = defineTool({
  name: 'qdrant_retrieve',
  description: 'Retrieve chunks from Qdrant fixture',
  schema: z.object({ query: z.string(), top_k: z.number().int().positive().default(3) }),
  handler: ({ top_k }) => JSON.stringify(QDRANT_FIXTURE.slice(0, top_k)),
})

describe('phase144 rag retrieval qdrant', () => {
  it('fixture has 3 chunks', () => {
    expect(QDRANT_FIXTURE).toHaveLength(3)
  })

  it('all chunks have id', () => {
    QDRANT_FIXTURE.forEach((c) => expect(c.id).toMatch(/^qdrant-/))
  })

  it('all chunks have text', () => {
    QDRANT_FIXTURE.forEach((c) => expect(c.text.length).toBeGreaterThan(0))
  })

  it('scores are descending', () => {
    for (let i = 1; i < QDRANT_FIXTURE.length; i++) {
      expect(QDRANT_FIXTURE[i].score).toBeLessThanOrEqual(QDRANT_FIXTURE[i - 1].score)
    }
  })

  it('defineTool wraps qdrant retriever', () => {
    expect(qdrantRetrieve.spec.name).toBe('qdrant_retrieve')
  })

  it('qdrant tool spec has input_schema', () => {
    expect(qdrantRetrieve.spec.input_schema.type).toBe('object')
  })

  it('qdrant tool registered in registry', () => {
    const reg = new ToolRegistry()
    reg.register(qdrantRetrieve)
    expect(reg.has('qdrant_retrieve')).toBe(true)
  })

  it('dispatch returns JSON with 3 chunks', async () => {
    const reg = new ToolRegistry()
    reg.register(qdrantRetrieve)
    const result = await reg.dispatch('qdrant_retrieve', { query: 'vector', top_k: 3 })
    const chunks = JSON.parse(result as string)
    expect(chunks).toHaveLength(3)
  })

  it('top_k limits results', async () => {
    const reg = new ToolRegistry()
    reg.register(qdrantRetrieve)
    const result = await reg.dispatch('qdrant_retrieve', { query: 'rust', top_k: 1 })
    const chunks = JSON.parse(result as string)
    expect(chunks).toHaveLength(1)
  })

  it('result chunks have id, text, score', async () => {
    const reg = new ToolRegistry()
    reg.register(qdrantRetrieve)
    const result = await reg.dispatch('qdrant_retrieve', { query: 'hnsw', top_k: 2 })
    const chunks = JSON.parse(result as string)
    chunks.forEach((c: { id: string; text: string; score: number }) => {
      expect(c.id).toBeDefined()
      expect(c.text).toBeDefined()
      expect(c.score).toBeGreaterThan(0)
    })
  })
})
