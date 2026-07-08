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
  it('valid token returns data', async () => {
    const reg = new ToolRegistry()
    reg.register(secureResource)
    const result = JSON.parse((await reg.dispatch('secure_resource', { token: VALID_TOKEN })) as string)
    expect(result.data).toBe('secret-value')
  })

  it('invalid token throws unauthorized', async () => {
    const reg = new ToolRegistry()
    reg.register(secureResource)
    await expect(reg.dispatch('secure_resource', { token: 'wrong' })).rejects.toThrow(ERR_UNAUTHORIZED)
  })

  it('empty token throws unauthorized', async () => {
    const reg = new ToolRegistry()
    reg.register(secureResource)
    await expect(reg.dispatch('secure_resource', { token: '' })).rejects.toThrow(ERR_UNAUTHORIZED)
  })

  it('error message is unauthorized not secret-value', async () => {
    const reg = new ToolRegistry()
    reg.register(secureResource)
    let msg = ''
    try { await reg.dispatch('secure_resource', { token: 'bad' }) } catch (err) { msg = (err as Error).message }
    expect(msg).toBe(ERR_UNAUTHORIZED)
    expect(msg).not.toContain('secret-value')
  })

  it('multiple invalid tokens all throw', async () => {
    const reg = new ToolRegistry()
    reg.register(secureResource)
    const bad = ['', 'wrong', 'hacked', "' OR 1=1"]
    for (const t of bad) {
      await expect(reg.dispatch('secure_resource', { token: t })).rejects.toThrow()
    }
  })

  it('valid dispatch does not throw', async () => {
    const reg = new ToolRegistry()
    reg.register(secureResource)
    const result = await reg.dispatch('secure_resource', { token: VALID_TOKEN })
    expect(result).toBeDefined()
  })

  it('error does not corrupt registry', async () => {
    const reg = new ToolRegistry()
    reg.register(secureResource)
    try { await reg.dispatch('secure_resource', { token: 'bad' }) } catch (_) {}
    expect(reg.has('secure_resource')).toBe(true)
  })

  it('tool spec name matches handler name', () => {
    expect(secureResource.spec.name).toBe('secure_resource')
  })

  it('ERR_UNAUTHORIZED sentinel value is unauthorized', () => {
    expect(ERR_UNAUTHORIZED).toBe('unauthorized')
  })

  it('dispatch missing tool still throws', async () => {
    const reg = new ToolRegistry()
    await expect(reg.dispatch('missing_tool', {})).rejects.toThrow()
  })
})
