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
