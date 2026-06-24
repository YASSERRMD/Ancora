import { z } from 'zod'
import { ToolSpec } from './schemas'
import { zodToInputSchema } from './zod-to-schema'

export interface ToolDef<T = unknown> {
  spec: ToolSpec
  handler: (input: T) => unknown | Promise<unknown>
}

export function defineTool<S extends z.ZodObject<z.ZodRawShape>>(opts: {
  name: string
  description: string
  schema: S
  handler: (input: z.infer<S>) => unknown | Promise<unknown>
}): ToolDef<z.infer<S>> {
  const input_schema = zodToInputSchema(opts.schema)
  return {
    spec: { name: opts.name, description: opts.description, input_schema },
    handler: opts.handler,
  }
}

export class ToolRegistry {
  private _tools: Map<string, ToolDef> = new Map()

  register(tool: ToolDef): this {
    this._tools.set(tool.spec.name, tool)
    return this
  }

  has(name: string): boolean {
    return this._tools.has(name)
  }

  get names(): string[] {
    return Array.from(this._tools.keys())
  }

  get specs(): ToolSpec[] {
    return Array.from(this._tools.values()).map((t) => t.spec)
  }

  async dispatch(name: string, input: unknown): Promise<unknown> {
    const tool = this._tools.get(name)
    if (!tool) throw new Error(`Tool not found: ${name}`)
    const result = tool.handler(input)
    return result instanceof Promise ? await result : result
  }
}
