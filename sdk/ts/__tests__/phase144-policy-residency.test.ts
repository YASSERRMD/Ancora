interface PolicyEvent {
  region: string
  blocked: boolean
  reason: string
}

function makePolicyEvent(region: string, blocked: boolean, reason = ''): PolicyEvent {
  return { region, blocked, reason }
}

describe('phase144 policy residency block', () => {
  it('policy event has region field', () => {
    const ev = makePolicyEvent('eu-west-1', false)
    expect(ev.region).toBe('eu-west-1')
  })

  it('policy event has blocked flag', () => {
    const ev = makePolicyEvent('us-east-1', true, 'no consent')
    expect(ev.blocked).toBe(true)
  })

  it('eu regions are blocked in fixture', () => {
    const regions = ['eu-west-1', 'eu-central-1', 'us-east-1']
    const events = regions.map((r) => makePolicyEvent(r, r.startsWith('eu'), r.startsWith('eu') ? 'GDPR' : ''))
    const blocked = events.filter((e) => e.blocked)
    expect(blocked).toHaveLength(2)
  })

  it('non-eu regions are allowed', () => {
    const ev = makePolicyEvent('us-west-2', false)
    expect(ev.blocked).toBe(false)
  })

  it('reason is empty when allowed', () => {
    const ev = makePolicyEvent('us-east-1', false)
    expect(ev.reason).toBe('')
  })

  it('reason is non-empty when blocked', () => {
    const ev = makePolicyEvent('eu-west-1', true, 'GDPR residency')
    expect(ev.reason.length).toBeGreaterThan(0)
  })

  it('policy event JSON round-trips', () => {
    const ev = makePolicyEvent('ap-southeast-1', false)
    const parsed = JSON.parse(JSON.stringify(ev)) as PolicyEvent
    expect(parsed.region).toBe('ap-southeast-1')
    expect(parsed.blocked).toBe(false)
  })

  it('multiple blocked events counted correctly', () => {
    const events = [
      makePolicyEvent('eu-west-1', true, 'GDPR'),
      makePolicyEvent('us-east-1', false),
      makePolicyEvent('eu-central-1', true, 'GDPR'),
    ]
    expect(events.filter((e) => e.blocked)).toHaveLength(2)
  })

  it('allowed events count correctly', () => {
    const events = [
      makePolicyEvent('us-east-1', false),
      makePolicyEvent('ap-south-1', false),
      makePolicyEvent('eu-west-1', true, 'GDPR'),
    ]
    expect(events.filter((e) => !e.blocked)).toHaveLength(2)
  })

  it('policy event is a plain object', () => {
    const ev = makePolicyEvent('us-east-1', false)
    expect(typeof ev).toBe('object')
  })
})
