import { encodeSpec, decodeSpec } from '../wire'
import { AgentSpec } from '../schemas'

const SPEC: AgentSpec = {
  model: 'test-model',
  instructions: 'Use UTF-8: éàü',
  tools: [],
}

describe('wire encoding properties', () => {
  it('encodeSpec output is valid UTF-8', () => {
    const buf = encodeSpec(SPEC)
    expect(() => buf.toString('utf8')).not.toThrow()
  })

  it('multi-byte characters survive round-trip', () => {
    const decoded = decodeSpec(encodeSpec(SPEC))
    expect(decoded.instructions).toBe(SPEC.instructions)
  })

  it('optional fields absent in output when not set', () => {
    const spec: AgentSpec = { model: 'gpt-4', instructions: '', tools: [] }
    const json = JSON.parse(encodeSpec(spec).toString('utf8'))
    expect(json.max_tokens).toBeUndefined()
    expect(json.temperature).toBeUndefined()
  })

  it('optional fields present in output when set', () => {
    const spec: AgentSpec = { model: 'gpt-4', instructions: '', tools: [], max_tokens: 256, temperature: 0.3 }
    const json = JSON.parse(encodeSpec(spec).toString('utf8'))
    expect(json.max_tokens).toBe(256)
    expect(json.temperature).toBeCloseTo(0.3)
  })

  it('decodeSpec accepts Uint8Array', () => {
    const buf = encodeSpec(SPEC)
    const uint8 = new Uint8Array(buf.buffer, buf.byteOffset, buf.byteLength)
    const decoded = decodeSpec(uint8)
    expect(decoded.model).toBe(SPEC.model)
  })
})
