import { z } from 'zod'
import { defineTool, ToolRegistry } from '../tools'

interface McpCall { tool: string; args: Record<string, unknown> }
interface McpResult { tool: string; output: string; is_error: boolean }

function makeMcpCall(tool: string, args: Record<string, unknown>): McpCall {
  return { tool, args }
}

function makeMcpResult(tool: string, output: string, is_error = false): McpResult {
  return { tool, output, is_error }
}

const mcpReadFile = defineTool({
  name: 'mcp_read_file',
  description: 'Read file via MCP',
  schema: z.object({ path: z.string() }),
  handler: ({ path }) => JSON.stringify({ path, content: 'fixture-content', size: 15 }),
})

const mcpListDir = defineTool({
  name: 'mcp_list_dir',
  description: 'List directory via MCP',
  schema: z.object({ path: z.string() }),
  handler: ({ path }) => JSON.stringify({ path, entries: ['a.txt', 'b.txt'] }),
})

describe('phase144 mcp tool use', () => {
  it('mcp call has tool field', () => {
    const call = makeMcpCall('read_file', { path: '/etc/hosts' })
    expect(call.tool).toBe('read_file')
  })

  it('mcp result has is_error false on success', () => {
    const result = makeMcpResult('read_file', 'content')
    expect(result.is_error).toBe(false)
  })

  it('mcp result is_error true on error', () => {
    const result = makeMcpResult('delete', '', true)
    expect(result.is_error).toBe(true)
  })

  it('mcp read_file tool registered', () => {
    const reg = new ToolRegistry()
    reg.register(mcpReadFile)
    expect(reg.get('mcp_read_file')).toBeDefined()
  })

  it('mcp read_file dispatch returns JSON', () => {
    const reg = new ToolRegistry()
    reg.register(mcpReadFile)
    const result = reg.dispatch('mcp_read_file', { path: '/tmp/test.txt' })
    const parsed = JSON.parse(result as string)
    expect(parsed.content).toBe('fixture-content')
  })

  it('mcp list_dir dispatch returns entries', () => {
    const reg = new ToolRegistry()
    reg.register(mcpListDir)
    const result = reg.dispatch('mcp_list_dir', { path: '/tmp' })
    const parsed = JSON.parse(result as string)
    expect(parsed.entries).toHaveLength(2)
  })

  it('both mcp tools registered together', () => {
    const reg = new ToolRegistry()
    reg.register(mcpReadFile)
    reg.register(mcpListDir)
    expect(reg.get('mcp_read_file')).toBeDefined()
    expect(reg.get('mcp_list_dir')).toBeDefined()
  })

  it('mcp call JSON round-trips', () => {
    const call = makeMcpCall('search', { query: 'rust' })
    const parsed = JSON.parse(JSON.stringify(call)) as McpCall
    expect(parsed.tool).toBe('search')
    expect(parsed.args.query).toBe('rust')
  })

  it('mcp result output non-empty on success', () => {
    const result = makeMcpResult('list', '[\"a\",\"b\"]')
    expect(result.output.length).toBeGreaterThan(0)
  })

  it('mcp tool spec has description', () => {
    expect(mcpReadFile.spec.description).toBe('Read file via MCP')
  })
})
