# Policy and Data Residency (TypeScript)

## Configuration

```ts
import { buildSpec, PolicySpec } from 'ancora'

const spec = buildSpec({
  model: 'claude-3-5-haiku-20241022',
  instructions: 'Answer.',
  policy: {
    allowRegions: ['us-east-1', 'eu-west-1'],
    denyProviders: ['openai-gpt4-global'],
    maxWriteTools: 3,
  } satisfies PolicySpec,
})
```

## Capping write-tool calls

```ts
const spec = buildSpec({
  model: 'llama3',
  instructions: 'Modify files as needed.',
  policy: { maxWriteTools: 2 },
})
```

If the agent tries to call a third write tool, the run fails with a
`PolicyViolationError`.

## Catching policy violations

```ts
import { PolicyViolationError } from 'ancora'

try {
  const result = await rt.run(spec, 'Overwrite all config files.')
  console.log(result.output)
} catch (e) {
  if (e instanceof PolicyViolationError) {
    console.error('Policy blocked:', e.message)
  } else {
    throw e
  }
}
```

## Audit trail

Policy checks are journalled as `ActivityRecorded` events with
`activity_kind = "policy_check"`. They appear in the journal and are
replayed correctly.

## See also

- [Providers](providers.md)
- [Observability](observability.md)
- [Policy concept](../../concepts/policy-and-data-sovereignty.md)
