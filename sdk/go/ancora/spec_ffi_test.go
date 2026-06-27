package ancora_test

import (
	"testing"

	"ancora.io/sdk/ancora"
	"google.golang.org/protobuf/proto"
)

func TestSpecBytesDecodeToSameSpec(t *testing.T) {
	spec := ancora.NewAgentSpec("ffi-agent", "claude-3", "You are helpful.")
	b, err := proto.Marshal(spec)
	if err != nil {
		t.Fatalf("proto.Marshal: %v", err)
	}
	var decoded ancora.AgentSpec
	if err := proto.Unmarshal(b, &decoded); err != nil {
		t.Fatalf("proto.Unmarshal: %v", err)
	}
	if decoded.GetName() != "ffi-agent" {
		t.Fatalf("name mismatch: %q", decoded.GetName())
	}
	if decoded.GetModelId() != "claude-3" {
		t.Fatalf("model_id mismatch: %q", decoded.GetModelId())
	}
	if decoded.GetInstructions() != "You are helpful." {
		t.Fatalf("instructions mismatch: %q", decoded.GetInstructions())
	}
}

func TestSpecWithMaxStepsRoundTrips(t *testing.T) {
	spec := ancora.NewAgentSpecBuilder().
		WithName("bounded").
		WithModelID("gpt-4o").
		WithMaxSteps(7).
		Build()
	if spec.GetMaxSteps() != 7 {
		t.Fatalf("expected max_steps=7, got: %d", spec.GetMaxSteps())
	}
	b, _ := proto.Marshal(spec)
	var decoded ancora.AgentSpec
	_ = proto.Unmarshal(b, &decoded)
	if decoded.GetMaxSteps() != 7 {
		t.Fatalf("max_steps not preserved through FFI round trip")
	}
}

func TestSpecWithToolsRoundTrips(t *testing.T) {
	tool := ancora.NewToolSpec("search", "searches the web")
	spec := ancora.NewAgentSpecBuilder().
		WithName("searcher").
		WithModelID("gpt-4o").
		WithTool(tool).
		Build()
	b, _ := proto.Marshal(spec)
	var decoded ancora.AgentSpec
	_ = proto.Unmarshal(b, &decoded)
	if len(decoded.GetTools()) != 1 {
		t.Fatalf("expected 1 tool after round trip, got: %d", len(decoded.GetTools()))
	}
	if decoded.GetTools()[0].GetName() != "search" {
		t.Fatalf("tool name mismatch: %q", decoded.GetTools()[0].GetName())
	}
}

func TestSpecWithUnicodeInstructionsRoundTrips(t *testing.T) {
	instructions := "You are a helpful assistant. 你好世界!"
	spec := ancora.NewAgentSpec("uni-agent", "gpt-4o", instructions)
	b, _ := proto.Marshal(spec)
	var decoded ancora.AgentSpec
	_ = proto.Unmarshal(b, &decoded)
	if decoded.GetInstructions() != instructions {
		t.Fatalf("unicode instructions not preserved: %q", decoded.GetInstructions())
	}
}

func TestSpecBuildBytesPassedToStartRunSucceeds(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	b, err := ancora.NewAgentSpecBuilder().
		WithName("startup-agent").
		WithModelID("llama3").
		BuildBytes()
	if err != nil {
		t.Fatalf("BuildBytes: %v", err)
	}
	run, err := rt.StartRun(b)
	if err != nil {
		t.Fatalf("StartRun with spec bytes: %v", err)
	}
	if run.ID() == "" {
		t.Fatal("run ID must be non-empty")
	}
}

func TestEmptySpecBytesStartRunDoesNotPanic(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	_, err := rt.StartRun([]byte("{}"))
	if err != nil {
		t.Logf("StartRun with empty spec returned error (acceptable): %v", err)
	}
}

func TestSpecWithInstructionsRoundTrips(t *testing.T) {
	instructions := "Be concise and accurate."
	spec := ancora.NewAgentSpecBuilder().
		WithName("inst-agent").
		WithModelID("gpt-4o").
		WithInstructions(instructions).
		Build()
	b, _ := proto.Marshal(spec)
	var decoded ancora.AgentSpec
	_ = proto.Unmarshal(b, &decoded)
	if decoded.GetInstructions() != instructions {
		t.Fatalf("instructions not preserved: %q", decoded.GetInstructions())
	}
}

func TestToolSpecDescriptionPreserved(t *testing.T) {
	tool := ancora.NewToolSpec("calc", "performs arithmetic operations")
	if tool.GetDescription() != "performs arithmetic operations" {
		t.Fatalf("description mismatch: %q", tool.GetDescription())
	}
}
