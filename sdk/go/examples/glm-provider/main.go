// glm-provider configures an agent spec for the ChatGLM model family
// and runs it through the standard transport, demonstrating provider
// selection by model name rather than by transport wiring.
package main

import (
	"context"
	"fmt"
	"os"

	"ancora.io/sdk/ancora"
)

// glmModels lists well-known GLM model identifiers.
var glmModels = []string{
	"glm-4",
	"glm-4-flash",
	"glm-4-air",
	"glm-3-turbo",
}

func runGlmAgent(ctx context.Context, rt *ancora.Runtime, model string) error {
	spec := ancora.NewAgentSpec(
		fmt.Sprintf("glm-%s-agent", model),
		model,
		"You are a helpful assistant powered by the GLM model family.",
	)
	ag := ancora.NewTransportAgent(ancora.NewCgoTransport(rt), spec)

	run, err := ag.Start(ctx)
	if err != nil {
		return fmt.Errorf("start: %w", err)
	}
	fmt.Printf("model=%s run=%s\n", model, run.ID())

	evs, err := run.DrainEvents(ctx)
	if err != nil {
		return fmt.Errorf("drain: %w", err)
	}
	fmt.Printf("  events: %d\n", len(evs))
	return nil
}

func main() {
	rt, err := ancora.NewRuntime()
	if err != nil {
		fmt.Fprintf(os.Stderr, "runtime: %v\n", err)
		os.Exit(1)
	}
	defer rt.Free()

	ctx := context.Background()

	// Demonstrate that any model name can be passed to NewAgentSpec.
	// In production the runtime resolves the model name to the configured
	// provider endpoint (e.g. Zhipu AI API or a local Ollama shim).
	for _, model := range glmModels {
		if err := runGlmAgent(ctx, rt, model); err != nil {
			fmt.Fprintf(os.Stderr, "glm %s: %v\n", model, err)
			os.Exit(1)
		}
	}

	fmt.Println("glm-provider done")
}
