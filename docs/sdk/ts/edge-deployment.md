# WASM and Edge Deployment (TypeScript)

## Node.js single-binary (pkg)

Bundle the N-API addon and model endpoint into a single executable:

```bash
npm install -g @vercel/ncc
ncc build src/agent.ts -o dist/
```

Distribute `dist/` alongside `libancora_ffi.so`. Set `LD_LIBRARY_PATH` at
runtime:

```bash
LD_LIBRARY_PATH=./dist ./dist/agent
```

## Docker

```dockerfile
FROM node:20-slim

RUN apt-get update && apt-get install -y libgcc-s1 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY package*.json ./
RUN npm ci --omit=dev

COPY dist/ ./dist/
COPY libancora_ffi.so ./

ENV ANCORA_MODEL_URL=http://ollama:11434
ENV LD_LIBRARY_PATH=/app

CMD ["node", "dist/agent.js"]
```

## Deno (WASM backend)

When targeting Deno, Ancora uses a WASM backend instead of N-API:

```bash
deno run --allow-net --allow-env --allow-read agent.ts
```

The WASM backend does not require a native library or a Rust toolchain on
the target system.

## Air-gapped edge

1. Build the package with the pre-built binary included:
   ```bash
   npm pack ancora
   ```
2. Copy the `.tgz` and Ollama model weights to the air-gapped host.
3. Install offline:
   ```bash
   npm install --offline ancora-*.tgz
   ```
4. Start Ollama and set `ANCORA_MODEL_URL`.

## Cloudflare Workers (planned)

Worker support via WASM backend is on the roadmap. Track progress in the
repository issue tracker.

## See also

- [Install](install.md)
- [Deployment models concept](../../concepts/deployment-models.md)
