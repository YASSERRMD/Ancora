import { z } from 'zod'
import { defineTool, ToolRegistry } from '../tools'

const VALID_TOKEN = 'valid-token-ts-e2e'
const ERR_UNAUTHORIZED = 'unauthorized'

const secureResource = defineTool({
  name: 'secure_resource',
  description: 'Requires auth token',
  schema: z.object({ token: z.string() }),
  handler: ({ token }) => {
    if (token !== VALID_TOKEN) throw new Error(ERR_UNAUTHORIZED)
    return JSON.stringify({ data: 'secret-value' })
  },
})

describe('phase145 unauthenticated mcp refused', () => {
  it('valid token returns data', () => {
    const reg = new ToolRegistry()
    reg.register(secureResource)
    const result = JSON.parse(reg.dispatch('secure_resource', { token: VALID_TOKEN }) as string)
    expect(result.data).toBe('secret-value')
  })

  it('invalid token throws unauthorized', () => {
    const reg = new ToolRegistry()
    reg.register(secureResource)
    expect(() => reg.dispatch('secure_resource', { token: 'wrong' })).toThrow(ERR_UNAUTHORIZED)
  })

  it('empty token throws unauthorized', () => {
    const reg = new ToolRegistry()
    reg.register(secureResource)
    expect(() => reg.dispatch('secure_resource', { token: '' })).toThrow(ERR_UNAUTHORIZED)
  })

  it('error message is unauthorized not secret-value', () => {
    const reg = new ToolRegistry()
    reg.register(secureResource)
    let msg = ''
    try { reg.dispatch('secure_resource', { token: 'bad' }) } catch (err) { msg = (err as Error).message }
    expect(msg).toBe(ERR_UNAUTHORIZED)
    expect(msg).not.toContain('secret-value')
  })

  it('multiple invalid tokens all throw', () => {
    const reg = new ToolRegistry()
    reg.register(secureResource)
    const bad = ['', 'wrong', 'hacked', "' OR 1=1"]
    bad.forEach((t) => {
      expect(() => reg.dispatch('secure_resource', { token: t })).toThrow()
    })
  })

  it('valid dispatch does not throw', () => {
    const reg = new ToolRegistry()
    reg.register(secureResource)
    expect(() => reg.dispatch('secure_resource', { token: VALID_TOKEN })).not.toThrow()
  })

  it('error does not corrupt registry', () => {
    const reg = new ToolRegistry()
    reg.register(secureResource)
    try { reg.dispatch('secure_resource', { token: 'bad' }) } catch (_) {}
    expect(reg.get('secure_resource')).toBeDefined()
  })

  it('tool spec name matches handler name', () => {
    expect(secureResource.spec.name).toBe('secure_resource')
  })

  it('ERR_UNAUTHORIZED sentinel value is unauthorized', () => {
    expect(ERR_UNAUTHORIZED).toBe('unauthorized')
  })

  it('dispatch missing tool still throws', () => {
    const reg = new ToolRegistry()
    expect(() => reg.dispatch('missing_tool', {})).toThrow()
  })
})
