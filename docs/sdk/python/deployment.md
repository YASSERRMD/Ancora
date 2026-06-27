# Packaging and Deployment (Python)

## Docker

```dockerfile
FROM python:3.12-slim

# Install native dependencies
RUN apt-get update && apt-get install -y libgcc-s1 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY . .

ENV ANCORA_MODEL_URL=http://ollama:11434

CMD ["python", "agent.py"]
```

## Air-gapped deployment

1. Build a wheel with the pre-built native library:
   ```bash
   pip wheel ancora --no-deps -w ./dist/
   ```
2. Copy `dist/` and the model weights to the air-gapped host.
3. Install from local wheel:
   ```bash
   pip install --no-index --find-links ./dist/ ancora
   ```
4. Start Ollama and pull the model on the air-gapped host:
   ```bash
   ollama pull llama3
   ```

## uWSGI / Gunicorn

Ancora's `Runtime` is not fork-safe. Create one `Runtime` per worker process:

```python
# worker_init hook
from ancora import Runtime

_rt = None

def post_fork(server, worker):
    global _rt
    _rt = Runtime()
```

## Serverless (AWS Lambda)

Initialise `Runtime` outside the handler to reuse across warm invocations:

```python
from ancora import Runtime, AgentSpec

_rt = Runtime()
_spec = AgentSpec(model="llama3", instructions="Answer.")

def handler(event, context):
    result = _rt.run(_spec, event["prompt"])
    return {"output": result.output}
```

The native library must be bundled in the deployment package. See the
[Lambda deployment guide](../guides/deployment-lambda.md) for layer setup.

## See also

- [Install](install.md)
- [Edge deployment concept](../../concepts/deployment-models.md)
