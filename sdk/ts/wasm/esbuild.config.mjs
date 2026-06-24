import { build } from 'esbuild'
import { fileURLToPath } from 'url'
import { dirname, join } from 'path'

const __dirname = dirname(fileURLToPath(import.meta.url))

await build({
  entryPoints: [join(__dirname, 'index.ts')],
  bundle: true,
  outfile: join(__dirname, '..', 'dist', 'wasm', 'ancora-wasm.js'),
  format: 'esm',
  platform: 'browser',
  target: ['es2020', 'chrome90', 'firefox90', 'safari14'],
  external: [],
  minify: false,
  sourcemap: true,
})

console.log('WASM bundle built.')
