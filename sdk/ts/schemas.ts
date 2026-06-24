import { z } from 'zod'

export const ToolInputPropertySchema = z.object({
  type: z.string(),
  description: z.string().optional(),
})

export const ToolInputSchemaSchema = z.object({
  type: z.literal('object'),
  properties: z.record(ToolInputPropertySchema).default({}),
  required: z.array(z.string()).default([]),
})

export const ToolSpecSchema = z.object({
  name: z.string().min(1),
  description: z.string(),
  input_schema: ToolInputSchemaSchema,
})

export type ToolSpec = z.infer<typeof ToolSpecSchema>

export const AgentSpecSchema = z.object({
  model: z.string().min(1),
  instructions: z.string().default(''),
  tools: z.array(ToolSpecSchema).default([]),
  max_tokens: z.number().int().positive().optional(),
  temperature: z.number().min(0).max(2).optional(),
})

export type AgentSpec = z.infer<typeof AgentSpecSchema>

export const RunEventSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('started'), run_id: z.string(), spec: z.string() }),
  z.object({ kind: z.literal('token'), run_id: z.string(), text: z.string() }),
  z.object({ kind: z.literal('completed'), run_id: z.string() }),
  z.object({ kind: z.literal('resumed'), run_id: z.string(), decision: z.string() }),
  z.object({ kind: z.literal('tool_call'), run_id: z.string(), name: z.string(), input: z.string() }),
])

export type RunEvent = z.infer<typeof RunEventSchema>

export const ToolCallSchema = z.object({
  name: z.string().min(1),
  input: z.record(z.unknown()),
})

export type ToolCall = z.infer<typeof ToolCallSchema>
