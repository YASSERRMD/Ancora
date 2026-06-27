# Observability and Cost (TypeScript)

## In-process token tracking

```ts
import { Runtime, buildSpec } from 'ancora'

const rt = new Runtime()
const spec = buildSpec({ model: 'llama3', instructions: 'Answer.' })

let totalTokens = 0
for await (const event of rt.stream(spec, 'What is 2+2?')) {
  if (event.type === 'token') {
    totalTokens += Math.ceil(event.token.length / 4)
  }
}
console.log('Estimated tokens:', totalTokens)
```

## Per-run usage from completed event

```ts
for await (const event of rt.stream(spec, 'What is 2+2?')) {
  if (event.type === 'completed') {
    console.log('Input tokens: ', event.usage.inputTokens)
    console.log('Output tokens:', event.usage.outputTokens)
  }
}
```

## OpenTelemetry export

```bash
npm install @opentelemetry/sdk-node @opentelemetry/exporter-trace-otlp-grpc
```

```ts
import { NodeSDK } from '@opentelemetry/sdk-node'
import { OTLPTraceExporter } from '@opentelemetry/exporter-trace-otlp-grpc'
import { trace } from '@opentelemetry/api'

const sdk = new NodeSDK({ traceExporter: new OTLPTraceExporter({ url: 'http://localhost:4317' }) })
sdk.start()

const tracer = trace.getTracer('ancora')

const span = tracer.startSpan('agent-run')
const result = await rt.run(spec, 'What is 2+2?')
span.setAttribute('ancora.model', spec.model)
span.setAttribute('ancora.output_tokens', result.usage.outputTokens)
span.end()
```

## See also

- [Durability](durability.md)
- [Policy](policy.md)
