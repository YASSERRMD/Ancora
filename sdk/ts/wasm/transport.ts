export interface TransportOptions {
  baseUrl: string
  headers?: Record<string, string>
}

export class HttpTransport {
  private _baseUrl: string
  private _headers: Record<string, string>

  constructor(opts: TransportOptions) {
    this._baseUrl = opts.baseUrl.replace(/\/$/, '')
    this._headers = { 'Content-Type': 'application/json', ...opts.headers }
  }

  get baseUrl(): string {
    return this._baseUrl
  }

  async startRun(specBytes: Uint8Array): Promise<string> {
    const res = await fetch(`${this._baseUrl}/v1/runs`, {
      method: 'POST',
      headers: this._headers,
      body: specBytes,
    })
    if (!res.ok) throw new Error(`startRun failed: ${res.status}`)
    const data = (await res.json()) as { run_id: string }
    return data.run_id
  }

  async pollRun(runId: string): Promise<Uint8Array | null> {
    const res = await fetch(`${this._baseUrl}/v1/runs/${encodeURIComponent(runId)}/events`, {
      method: 'GET',
      headers: this._headers,
    })
    if (res.status === 204) return null
    if (!res.ok) throw new Error(`pollRun failed: ${res.status}`)
    const buf = await res.arrayBuffer()
    if (buf.byteLength === 0) return null
    return new Uint8Array(buf)
  }

  async resumeRun(runId: string, decision: Uint8Array): Promise<void> {
    const res = await fetch(`${this._baseUrl}/v1/runs/${encodeURIComponent(runId)}/resume`, {
      method: 'POST',
      headers: this._headers,
      body: decision,
    })
    if (!res.ok) throw new Error(`resumeRun failed: ${res.status}`)
  }
}
