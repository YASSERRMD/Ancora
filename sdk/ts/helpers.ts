import { RunEvent } from './schemas'

export async function collectEvents(
  iter: AsyncIterable<RunEvent>
): Promise<RunEvent[]> {
  const events: RunEvent[] = []
  for await (const ev of iter) {
    events.push(ev)
  }
  return events
}

export function tokenText(events: RunEvent[]): string {
  return events
    .filter((e): e is Extract<RunEvent, { kind: 'token' }> => e.kind === 'token')
    .map((e) => e.text)
    .join('')
}
