import { validateSpec, buildSpec } from 'ancora'

function trySpec(raw: unknown): void {
  const result = validateSpec(raw)
  if (result.ok) {
    console.log('Valid spec:', result.spec.model)
  } else {
    console.log('Invalid spec errors:', result.errors)
  }
}

async function main() {
  trySpec({ model: 'claude-3-5-sonnet', max_tokens: 1024, temperature: 0.7 })
  trySpec({ instructions: 'missing model field' })
  trySpec({ model: 'test', temperature: 3.0 })
  const spec = buildSpec('claude-3-5-sonnet', { maxTokens: 2048 })
  console.log('Built spec:', JSON.stringify(spec, null, 2))
}

main().catch(console.error)
