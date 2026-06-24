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

func TestNewAgentReturnsNonNil(t *testing.T) {
	rt, ag := makeAgent(t)
	defer rt.Free()
	if ag == nil {
		t.Fatal("NewAgent returned nil")
	}
}
