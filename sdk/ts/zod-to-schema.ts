import { z } from 'zod'
import { ToolInputSchemaSchema } from './schemas'

type InputSchema = {
  type: 'object'
  properties: Record<string, { type: string; description?: string }>
  required: string[]
}

export function zodToInputSchema(schema: z.ZodObject<z.ZodRawShape>): InputSchema {
  const shape = schema.shape
  const properties: Record<string, { type: string; description?: string }> = {}
  const required: string[] = []

  for (const [key, field] of Object.entries(shape)) {
    const typeName = getZodTypeName(field as z.ZodTypeAny)
    const description = getZodDescription(field as z.ZodTypeAny)
    properties[key] = description ? { type: typeName, description } : { type: typeName }

    if (!isOptional(field as z.ZodTypeAny)) {
      required.push(key)
    }
  }

  return ToolInputSchemaSchema.parse({ type: 'object', properties, required })
}

function getZodTypeName(field: z.ZodTypeAny): string {
  if (field instanceof z.ZodString) return 'string'
  if (field instanceof z.ZodNumber) return 'number'
  if (field instanceof z.ZodBoolean) return 'boolean'
  if (field instanceof z.ZodArray) return 'array'
  if (field instanceof z.ZodObject) return 'object'
  if (field instanceof z.ZodOptional) return getZodTypeName(field.unwrap())
  if (field instanceof z.ZodNullable) return getZodTypeName(field.unwrap())
  return 'string'
}

function getZodDescription(field: z.ZodTypeAny): string | undefined {
  return field.description
}

function isOptional(field: z.ZodTypeAny): boolean {
  return field instanceof z.ZodOptional || field instanceof z.ZodNullable
}
