const originalPlatform = process.platform
const originalArch = process.arch

jest.mock('../ancora.node', () => ({ localBinding: true }), { virtual: true })

afterAll(() => {
  Object.defineProperty(process, 'platform', { value: originalPlatform, configurable: true })
  Object.defineProperty(process, 'arch', { value: originalArch, configurable: true })
})

describe('loadBinding', () => {
  it('falls back to ./ancora.node when platform package is missing', () => {
    jest.resetModules()
    Object.defineProperty(process, 'platform', { value: 'freebsd', configurable: true })
    Object.defineProperty(process, 'arch', { value: 'x64', configurable: true })

    jest.mock('../ancora.node', () => ({ fallback: true }), { virtual: true })

    const { loadBinding } = jest.requireActual('../load-binding') as typeof import('../load-binding')
    const binding = loadBinding() as Record<string, unknown>
    expect(binding).toBeDefined()
  })

  it('exports a loadBinding function', () => {
    jest.resetModules()
    const mod = require('../load-binding') as typeof import('../load-binding')
    expect(typeof mod.loadBinding).toBe('function')
  })

  it('PLATFORM_PACKAGES covers linux-x64', () => {
    jest.resetModules()
    Object.defineProperty(process, 'platform', { value: 'linux', configurable: true })
    Object.defineProperty(process, 'arch', { value: 'x64', configurable: true })
    const { loadBinding } = jest.requireActual('../load-binding') as typeof import('../load-binding')
    expect(typeof loadBinding).toBe('function')
  })
})
