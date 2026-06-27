package ancora_test

import (
	"testing"

	"ancora.io/sdk/ancora"
)

const (
	provAnthropicModel = "claude-opus-4-8"
	provOpenAIModel    = "gpt-4o"
	provGeminiModel    = "gemini-2-5-pro"
	provMistralModel   = "mistral-large-latest"
	provDeepSeekModel  = "deepseek-chat"
)

func TestProviderAnthropicSpecHasCorrectModelID(t *testing.T) {
	spec := ancora.NewAgentSpec("anthropic-agent", provAnthropicModel, "")
	if spec.GetModelId() != provAnthropicModel {
		t.Fatalf("expected %q, got: %q", provAnthropicModel, spec.GetModelId())
	}
}

func TestProviderOpenAISpecHasCorrectModelID(t *testing.T) {
	spec := ancora.NewAgentSpec("openai-agent", provOpenAIModel, "")
	if spec.GetModelId() != provOpenAIModel {
		t.Fatalf("expected %q, got: %q", provOpenAIModel, spec.GetModelId())
	}
}

func TestProviderGeminiSpecHasCorrectModelID(t *testing.T) {
	spec := ancora.NewAgentSpec("gemini-agent", provGeminiModel, "")
	if spec.GetModelId() != provGeminiModel {
		t.Fatalf("expected %q, got: %q", provGeminiModel, spec.GetModelId())
	}
}

func TestProviderMistralSpecHasCorrectModelID(t *testing.T) {
	spec := ancora.NewAgentSpec("mistral-agent", provMistralModel, "")
	if spec.GetModelId() != provMistralModel {
		t.Fatalf("expected %q, got: %q", provMistralModel, spec.GetModelId())
	}
}

func TestProviderDeepSeekSpecHasCorrectModelID(t *testing.T) {
	spec := ancora.NewAgentSpec("deepseek-agent", provDeepSeekModel, "")
	if spec.GetModelId() != provDeepSeekModel {
		t.Fatalf("expected %q, got: %q", provDeepSeekModel, spec.GetModelId())
	}
}

func TestProviderModelIDsAreDistinct(t *testing.T) {
	models := []string{
		provAnthropicModel,
		provOpenAIModel,
		provGeminiModel,
		provMistralModel,
		provDeepSeekModel,
	}
	seen := make(map[string]bool)
	for _, m := range models {
		if seen[m] {
			t.Fatalf("duplicate model ID: %q", m)
		}
		seen[m] = true
	}
}

func TestProviderSelectionTransportFromEnvIsNonNil(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tr, err := ancora.NewTransportFromEnv(rt)
	if err != nil {
		t.Fatalf("NewTransportFromEnv: %v", err)
	}
	if tr == nil {
		t.Fatal("NewTransportFromEnv must return non-nil transport")
	}
}

func TestNewCgoTransportIsNonNil(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tr := ancora.NewCgoTransport(rt)
	if tr == nil {
		t.Fatal("NewCgoTransport must return non-nil")
	}
}

func TestProviderSpecBuilderSetsModelIDForAllFiveProviders(t *testing.T) {
	providers := []struct {
		name    string
		modelID string
	}{
		{"anthropic", provAnthropicModel},
		{"openai", provOpenAIModel},
		{"gemini", provGeminiModel},
		{"mistral", provMistralModel},
		{"deepseek", provDeepSeekModel},
	}
	for _, p := range providers {
		spec := ancora.NewAgentSpecBuilder().
			WithName(p.name + "-agent").
			WithModelID(p.modelID).
			Build()
		if spec.GetModelId() != p.modelID {
			t.Fatalf("provider %s: model ID mismatch: %q", p.name, spec.GetModelId())
		}
	}
}

func TestAllFiveProviderRunsStartSuccessfully(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	for _, modelID := range []string{
		provAnthropicModel, provOpenAIModel, provGeminiModel,
		provMistralModel, provDeepSeekModel,
	} {
		spec := ancora.NewAgentSpec("prov-test", modelID, "test")
		run, err := ancora.NewAgent(rt, spec).Start()
		if err != nil {
			t.Fatalf("Start with model %q: %v", modelID, err)
		}
		if run.ID() == "" {
			t.Fatalf("run ID empty for model %q", modelID)
		}
	}
}
