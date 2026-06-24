import { Agent, buildSpec } from 'ancora'
import { makeOfflineRuntime } from './offline-runtime'

async function main() {
  const spec = buildSpec('claude-3-5-sonnet', {
    instructions: 'You are a helpful chat assistant.',
    maxTokens: 2048,
  })

  const tokens = ['Hello', '!', ' How', ' can', ' I', ' help', ' you', ' today', '?']
  const rt = makeOfflineRuntime(tokens)
  const agent = new Agent(rt)
  const handle = agent.run(spec)

  process.stdout.write('Assistant: ')

  for await (const ev of handle) {
    if (ev.kind === 'token') {
      process.stdout.write(ev.text)
    } else if (ev.kind === 'completed') {
      process.stdout.write('\n')
    }
  }

  agent.free()
}

main().catch(console.error)
