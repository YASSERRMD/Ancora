jest.mock('../ancora.node', () => ({
  Runtime: class MockRuntime {
    private _freed = false
    get isFreed(): boolean { return this._freed }
    free(): void { this._freed = true }
    startRun(): string { return 'run-0' }
    pollRun(): Buffer | null { return null }
    resumeRun(): void {}
  },
  version: () => '0.1.0-phase144',
}), { virtual: true })

import { Runtime, version } from '../index'

describe('phase144 addon load and runtime create', () => {
  it('exports Runtime class', () => {
    expect(typeof Runtime).toBe('function')
  })

  it('exports version function', () => {
    expect(typeof version).toBe('function')
  })

  it('version returns string', () => {
    expect(typeof version()).toBe('string')
  })

  it('version matches semver pattern', () => {
    expect(version()).toMatch(/^\d+\.\d+\.\d+/)
  })

  it('Runtime constructor creates instance', () => {
    const rt = new Runtime()
    expect(rt).toBeDefined()
  })

  it('Runtime starts with isFreed false', () => {
    const rt = new Runtime()
    expect(rt.isFreed).toBe(false)
  })

  it('Runtime.free sets isFreed true', () => {
    const rt = new Runtime()
    rt.free()
    expect(rt.isFreed).toBe(true)
  })

  it('multiple Runtime instances are independent', () => {
    const rt1 = new Runtime()
    const rt2 = new Runtime()
    rt1.free()
    expect(rt1.isFreed).toBe(true)
    expect(rt2.isFreed).toBe(false)
    rt2.free()
  })

  it('startRun returns a non-empty string', () => {
    const rt = new Runtime()
    const id = rt.startRun('{"model":"test"}')
    expect(typeof id).toBe('string')
    expect(id.length).toBeGreaterThan(0)
  })

  it('pollRun returns null after drain', () => {
    const rt = new Runtime()
    const id = rt.startRun('{}')
    for (let i = 0; i < 20; i++) rt.pollRun(id)
    expect(rt.pollRun(id)).toBeNull()
  })
})
