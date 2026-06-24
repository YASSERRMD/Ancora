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
