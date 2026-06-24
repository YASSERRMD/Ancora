import * as path from 'path'
import * as fs from 'fs'

const npmDir = path.join(__dirname, '..', 'npm')

const PLATFORMS = [
  'linux-x64-gnu',
  'darwin-x64',
  'darwin-arm64',
  'win32-x64-msvc',
  'linux-arm64-gnu',
]

describe('platform package structure', () => {
  for (const platform of PLATFORMS) {
    describe(platform, () => {
      const pkgPath = path.join(npmDir, platform, 'package.json')
      const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf8')) as Record<string, unknown>

      it('has a name field starting with @ancora/', () => {
        expect(typeof pkg['name']).toBe('string')
        expect((pkg['name'] as string).startsWith('@ancora/')).toBe(true)
      })

      it('has version 0.1.0', () => {
        expect(pkg['version']).toBe('0.1.0')
      })

      it('has os field', () => {
        expect(Array.isArray(pkg['os'])).toBe(true)
        expect((pkg['os'] as string[]).length).toBeGreaterThan(0)
      })

      it('has cpu field', () => {
        expect(Array.isArray(pkg['cpu'])).toBe(true)
        expect((pkg['cpu'] as string[]).length).toBeGreaterThan(0)
      })

      it('has files field including .node file', () => {
        const files = pkg['files'] as string[]
        expect(Array.isArray(files)).toBe(true)
        expect(files.some(f => f.endsWith('.node'))).toBe(true)
      })

      it('has Apache-2.0 license', () => {
        expect(pkg['license']).toBe('Apache-2.0')
      })

      it('has a README.md', () => {
        const readmePath = path.join(npmDir, platform, 'README.md')
        expect(fs.existsSync(readmePath)).toBe(true)
      })
    })
  }

  it('main package has all platforms in optionalDependencies', () => {
    const mainPkg = JSON.parse(
      fs.readFileSync(path.join(__dirname, '..', 'package.json'), 'utf8')
    ) as { optionalDependencies: Record<string, string> }
    const opts = mainPkg.optionalDependencies
    expect(Object.keys(opts)).toHaveLength(PLATFORMS.length)
    for (const [name, version] of Object.entries(opts)) {
      expect(name.startsWith('@ancora/')).toBe(true)
      expect(version).toBe('0.1.0')
    }
  })
})
