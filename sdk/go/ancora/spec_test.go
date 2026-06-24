package ancora_test

import (
	"testing"

	"ancora.io/sdk/ancora"
	"google.golang.org/protobuf/proto"
)

func TestAgentSpecBuilderSetsName(t *testing.T) {
	spec := ancora.NewAgentSpecBuilder().WithName("my-agent").Build()
	if spec.GetName() != "my-agent" {
		t.Fatalf("expected name 'my-agent', got: %q", spec.GetName())
	}
}

func TestAgentSpecBuildBytesNonEmpty(t *testing.T) {
	b := ancora.NewAgentSpecBuilder().WithName("agent").WithModelID("llama3")
	bytes, err := b.BuildBytes()
	if err != nil {
		t.Fatalf("BuildBytes: %v", err)
	}
	if len(bytes) == 0 {
		t.Fatal("BuildBytes returned empty bytes")
	}
}

func TestAgentSpecBuilderSetsModelID(t *testing.T) {
	spec := ancora.NewAgentSpecBuilder().WithModelID("gpt-4o").Build()
	if spec.GetModelId() != "gpt-4o" {
		t.Fatalf("expected model id 'gpt-4o', got: %q", spec.GetModelId())
	}
}

func TestRoundTripSpecThroughFFI(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	spec := ancora.NewAgentSpecBuilder().
		WithName("ffi-agent").
		WithModelID("gpt-4o").
		WithInstructions("You are a test agent.").
		WithMaxSteps(5)
	bytes, err := spec.BuildBytes()
	if err != nil {
		t.Fatalf("BuildBytes: %v", err)
	}
	run, err := rt.StartRun(bytes)
	if err != nil {
		t.Fatalf("StartRun with spec bytes: %v", err)
	}
	if run.ID() == "" {
		t.Fatal("StartRun returned empty run ID")
	}
}

func TestToolSpecBuilderSetsName(t *testing.T) {
	tool := ancora.NewToolSpecBuilder().WithToolName("search").Build()
	if tool.GetName() != "search" {
		t.Fatalf("expected tool name 'search', got: %q", tool.GetName())
	}
}

func TestConvenienceNewAgentSpecSetsAllFields(t *testing.T) {
	spec := ancora.NewAgentSpec("a", "m", "i")
	if spec.GetName() != "a" || spec.GetModelId() != "m" || spec.GetInstructions() != "i" {
		t.Fatalf("NewAgentSpec fields wrong: %v", spec)
	}
}

func TestAgentSpecWithToolAttachesTool(t *testing.T) {
	tool := ancora.NewToolSpec("calc", "does math")
	spec := ancora.NewAgentSpecBuilder().WithTool(tool).Build()
	if len(spec.GetTools()) != 1 {
		t.Fatalf("expected 1 tool, got: %d", len(spec.GetTools()))
	}
	if spec.GetTools()[0].GetName() != "calc" {
		t.Fatalf("wrong tool name: %q", spec.GetTools()[0].GetName())
	}
}

func TestAgentSpecRoundTrip(t *testing.T) {
	original := ancora.NewAgentSpec("test-agent", "gpt-4o", "You are helpful.")
	bytes, err := proto.Marshal(original)
	if err != nil {
		t.Fatalf("proto.Marshal: %v", err)
	}
	var restored ancora.AgentSpec
	if err := proto.Unmarshal(bytes, &restored); err != nil {
		t.Fatalf("proto.Unmarshal: %v", err)
	}
	if restored.GetName() != "test-agent" {
		t.Fatalf("round-trip: name mismatch: %q", restored.GetName())
	}
	if restored.GetModelId() != "gpt-4o" {
		t.Fatalf("round-trip: model_id mismatch: %q", restored.GetModelId())
	}
}
