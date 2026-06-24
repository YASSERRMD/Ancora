package ancora_test

import (
	"testing"

	"ancora.io/sdk/ancora"
)

func makeAgent(t *testing.T) (*ancora.Runtime, *ancora.Agent) {
	t.Helper()
	rt := mustRuntime(t)
	spec := ancora.NewAgentSpec("test", "llama3", "You are a test agent.")
	return rt, ancora.NewAgent(rt, spec)
}

func TestAgentStartReturnsNonNilRun(t *testing.T) {
	rt, ag := makeAgent(t)
	defer rt.Free()
	run, err := ag.Start()
	if err != nil {
		t.Fatalf("Agent.Start: %v", err)
	}
	if run == nil {
		t.Fatal("Agent.Start returned nil Run")
	}
}

func TestAgentStartRunHasNonEmptyID(t *testing.T) {
	rt, ag := makeAgent(t)
	defer rt.Free()
	run, _ := ag.Start()
	if run.ID() == "" {
		t.Fatal("Agent.Start returned Run with empty ID")
	}
}

func TestNewAgentReturnsNonNil(t *testing.T) {
	rt, ag := makeAgent(t)
	defer rt.Free()
	if ag == nil {
		t.Fatal("NewAgent returned nil")
	}
}
