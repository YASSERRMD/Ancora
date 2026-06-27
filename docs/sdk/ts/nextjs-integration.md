# Next.js Integration (TypeScript)

Use Ancora in a Next.js API route or Server Action.

## API route (App Router)

```ts
// app/api/ask/route.ts
import { NextRequest, NextResponse } from 'next/server'
import { Runtime, buildSpec } from 'ancora'

const rt = new Runtime()
const spec = buildSpec({ model: 'llama3', instructions: 'Answer concisely.' })

export async function POST(req: NextRequest) {
  const { prompt } = await req.json()
  const result = await rt.run(spec, prompt)
  return NextResponse.json({ output: result.output })
}
```

## Streaming API route

```ts
// app/api/stream/route.ts
import { NextRequest } from 'next/server'
import { Runtime, buildSpec } from 'ancora'

const rt = new Runtime()
const spec = buildSpec({ model: 'llama3', instructions: 'Answer.' })

export async function POST(req: NextRequest) {
  const { prompt } = await req.json()

  const stream = new ReadableStream({
    async start(controller) {
      for await (const event of rt.stream(spec, prompt)) {
        if (event.type === 'token') {
          controller.enqueue(new TextEncoder().encode(event.token))
        } else if (event.type === 'completed') {
          controller.close()
        }
      }
    },
  })

  return new Response(stream, {
    headers: { 'Content-Type': 'text/plain; charset=utf-8' },
  })
}
```

## Server Actions

```ts
// app/actions.ts
'use server'

import { Runtime, buildSpec } from 'ancora'

const rt = new Runtime()
const spec = buildSpec({ model: 'llama3', instructions: 'Answer.' })

export async function ask(prompt: string): Promise<string> {
  const result = await rt.run(spec, prompt)
  return result.output
}
```

## Important: singleton Runtime

Create `Runtime` at module level, not inside a request handler. Module-level
singletons in Next.js are reused across requests in the same process.

## See also

- [Streaming](streaming.md)
- [Configuration](configuration.md)
