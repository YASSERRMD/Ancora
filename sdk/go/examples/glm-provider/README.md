# glm-provider

Demonstrates how to configure agent specs for the ChatGLM model family
(glm-4, glm-4-flash, glm-4-air, glm-3-turbo) by passing the model name
to `NewAgentSpec`.

The runtime resolves the model name to the configured provider endpoint at
run time, so no extra wiring is needed compared to using llama3 or GPT-4.

## Run

```bash
cd sdk/go
go run ./examples/glm-provider
```

## What it shows

- Iterating over multiple GLM model variants
- Building an `AgentSpec` per model with `NewAgentSpec`
- Running each agent independently via the same `Runtime`
