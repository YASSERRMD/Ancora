import { z } from 'zod'
import { zodToInputSchema, buildSpec, defineTool } from 'ancora'

const searchSchema = z.object({
  query: z.string().describe('The search query'),
  limit: z.number().optional().describe('Maximum number of results'),
  filters: z.array(z.string()).optional().describe('Filter terms'),
  exact: z.boolean().optional().describe('Whether to use exact matching'),
})

const searchTool = defineTool({
  name: 'search',
  description: 'Search a knowledge base',
  schema: searchSchema,
  handler: async ({ query, limit = 10 }) => ({
    results: Array.from({ length: Math.min(limit, 3) }, (_, i) => `Result ${i + 1} for: ${query}`),
  }),
})

async function main() {
  const inputSchema = zodToInputSchema(searchSchema)
  console.log('Generated JSON Schema:')
  console.log(JSON.stringify(inputSchema, null, 2))

  console.log('\nTool spec:')
  console.log(JSON.stringify(searchTool.spec, null, 2))

  const spec = buildSpec('claude-3-5-sonnet', {
    instructions: 'Use the search tool to answer questions.',
    tools: [searchTool.spec],
  })
  console.log('\nAgent spec has', spec.tools.length, 'tool(s)')
}

main().catch(console.error)
