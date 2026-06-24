import { Agent, buildSpec, tokenText } from 'ancora'
import { makeOfflineRuntime } from './offline-runtime'

async function runOnce(spec: ReturnType<typeof buildSpec>): Promise<string> {
  const rt = makeOfflineRuntime(['The', ' answer', ' is', ' 42.'])
  const agent = new Agent(rt)
  const handle = agent.run(spec)
  const events = []
  for await (const ev of handle) {
    events.push(ev)
  }
  agent.free()
  return tokenText(events)
}

async function main() {
  const spec = buildSpec('claude-3-5-sonnet', {
    instructions: 'Answer in one sentence.',
    maxTokens: 512,
  })

  const text = await runOnce(spec)
  console.log('Result:', text)
}

main().catch(console.error)
