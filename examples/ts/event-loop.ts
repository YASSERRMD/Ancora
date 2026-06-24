import { Agent, buildSpec } from 'ancora'
import { makeOfflineRuntime } from './offline-runtime'

async function main() {
  const spec = buildSpec('claude-3-5-sonnet', { maxTokens: 1024 })
  const rt = makeOfflineRuntime(['Processing', '...', ' done!'])
  const agent = new Agent(rt)
  const handle = agent.run(spec)

  let tokenCount = 0
  let started = false

  for await (const ev of handle) {
    switch (ev.kind) {
      case 'started':
        started = true
        console.log(`Run started: ${ev.run_id}`)
        break
      case 'token':
        tokenCount++
        process.stdout.write(ev.text)
        break
      case 'tool_call':
        console.log(`\nTool call: ${ev.name}`)
        handle.resume(JSON.stringify({ result: 'mock' }))
        break
      case 'completed':
        process.stdout.write('\n')
        console.log(`Run completed. Tokens: ${tokenCount}`)
        break
    }
  }

  if (!started) console.error('Run never started')
  agent.free()
}

main().catch(console.error)
