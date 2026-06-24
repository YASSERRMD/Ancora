import { RunEvent } from '../schemas'
import { parseEvent } from '../wire'

export function parseSseLine(line: string): RunEvent | null {
  const dataPrefix = 'data: '
  if (!line.startsWith(dataPrefix)) return null
  const payload = line.slice(dataPrefix.length).trim()
  if (!payload || payload === '[DONE]') return null
  try {
    return parseEvent(payload)
  } catch {
    return null
  }
}

export function* parseSseChunk(chunk: string): Iterable<RunEvent> {
  for (const line of chunk.split('\n')) {
    const ev = parseSseLine(line)
    if (ev !== null) yield ev
  }
}
