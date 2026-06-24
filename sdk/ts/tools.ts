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
