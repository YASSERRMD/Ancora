package ancora_test

import (
	"context"
	"encoding/json"
	"strings"
	"testing"

	"ancora.io/sdk/ancora"
)

// chineseMockResponse simulates a response from a Chinese LLM provider mock.
type chineseMockResponse struct {
	ModelID  string `json:"model_id"`
	Text     string `json:"text"`
	Provider string `json:"provider"`
}

const (
	glmModel      = "glm-4-plus"
	qwenModel     = "qwen-turbo"
	deepSeekModel = "deepseek-chat"
)

func fixtureChineseProvider(input []byte) ([]byte, error) {
	resp := chineseMockResponse{
		ModelID:  glmModel,
		Text:     "This is a fixture response from the Chinese provider mock.",
		Provider: "zhipu",
	}
	return json.Marshal(resp)
}

func TestE2EChineseProviderGLMModelIDIsSet(t *testing.T) {
	spec := ancora.NewAgentSpecBuilder().
		WithName("glm-agent").
		WithModelID(glmModel).
		Build()
	if spec.GetModelId() != glmModel {
		t.Fatalf("model ID mismatch: %q", spec.GetModelId())
	}
}

func TestE2EChineseProviderQwenModelIDIsSet(t *testing.T) {
	spec := ancora.NewAgentSpecBuilder().
		WithName("qwen-agent").
		WithModelID(qwenModel).
		Build()
	if spec.GetModelId() != qwenModel {
		t.Fatalf("model ID mismatch: %q", spec.GetModelId())
	}
}

func TestE2EChineseProviderDeepSeekModelIDIsSet(t *testing.T) {
	spec := ancora.NewAgentSpecBuilder().
		WithName("deepseek-agent").
		WithModelID(deepSeekModel).
		Build()
	if spec.GetModelId() != deepSeekModel {
		t.Fatalf("model ID mismatch: %q", spec.GetModelId())
	}
}

func TestE2EChineseProviderMockToolReturnsResponse(t *testing.T) {
	out, err := fixtureChineseProvider([]byte(`{}`))
	if err != nil {
		t.Fatalf("fixtureChineseProvider: %v", err)
	}
	var resp chineseMockResponse
	if err := json.Unmarshal(out, &resp); err != nil {
		t.Fatalf("Unmarshal: %v", err)
	}
	if resp.Provider != "zhipu" {
		t.Fatalf("provider mismatch: %q", resp.Provider)
	}
}

func TestE2EChineseProviderMockResponseContainsText(t *testing.T) {
	out, _ := fixtureChineseProvider(nil)
	if !strings.Contains(string(out), "fixture response") {
		t.Fatalf("mock response must contain 'fixture response', got: %s", out)
	}
}

func TestE2EChineseProviderGLMRunStartsSuccessfully(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	spec := ancora.NewAgentSpecBuilder().
		WithName("glm-test").
		WithModelID(glmModel).
		Build()

	run, err := ancora.NewAgent(rt, spec).Start()
	if err != nil {
		t.Fatalf("Start: %v", err)
	}
	if run.ID() == "" {
		t.Fatal("run ID must be non-empty")
	}
}

func TestE2EChineseProviderQwenRunStartsSuccessfully(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	spec := ancora.NewAgentSpecBuilder().
		WithName("qwen-test").
		WithModelID(qwenModel).
		Build()

	run, err := ancora.NewAgent(rt, spec).Start()
	if err != nil {
		t.Fatalf("Start: %v", err)
	}
	if run.ID() == "" {
		t.Fatal("run ID must be non-empty")
	}
}

func TestE2EChineseProviderMockToolRegistered(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	reg.Register("chinese-llm", fixtureChineseProvider)
	if !reg.Has("chinese-llm") {
		t.Fatal("chinese-llm must be registered")
	}
}

func TestE2EChineseProviderStoringTransportRecordsRun(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	tr := ancora.NewStoringTransport(ancora.NewCgoTransport(rt), store)
	builder := ancora.NewAgentSpecBuilder().
		WithName("glm-agent").
		WithModelID(glmModel)

	b, err := builder.BuildBytes()
	if err != nil {
		t.Fatalf("BuildBytes: %v", err)
	}

	runID, err := tr.StartRun(context.Background(), b)
	if err != nil {
		t.Fatalf("StartRun: %v", err)
	}

	has, _ := store.HasRun(runID)
	if !has {
		t.Fatalf("store must record Chinese provider run %q", runID)
	}
}

func TestE2EChineseProviderThreeModelsAreDistinct(t *testing.T) {
	models := []string{glmModel, qwenModel, deepSeekModel}
	seen := make(map[string]bool)
	for _, m := range models {
		if seen[m] {
			t.Fatalf("duplicate model ID: %q", m)
		}
		seen[m] = true
	}
}
