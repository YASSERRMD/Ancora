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
