import { Agent, buildSpec, collectEvents, tokenText } from 'ancora'
import { makeOfflineRuntime } from './offline-runtime'

async function main() {
  const spec = buildSpec('claude-3-5-sonnet', {
    instructions: 'You are a helpful assistant. Answer concisely.',
    maxTokens: 1024,
    temperature: 0.7,
  })

  const rt = makeOfflineRuntime(['The', ' answer', ' is', ' 42.'])
  const agent = new Agent(rt)
  const handle = agent.run(spec)

  console.log('Running single agent...')
  const events = await collectEvents(handle)
  const text = tokenText(events)
  console.log('Response:', text)
  console.log('Total events:', events.length)

  agent.free()
}

main().catch(console.error)
