package ancora_test

import (
	"context"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestNewCgoTransportAgentReturnsNonNil(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	spec := ancora.NewAgentSpec("factory-agent", "llama3", "hi")
	ag := ancora.NewCgoTransportAgent(rt, spec)
	if ag == nil {
		t.Fatal("NewCgoTransportAgent returned nil")
	}
}

func TestNewCgoTransportAgentTransportIsCgoTransport(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	spec := ancora.NewAgentSpec("cgo-factory", "llama3", "hi")
	ag := ancora.NewCgoTransportAgent(rt, spec)
	if _, ok := ag.Transport().(*ancora.CgoTransport); !ok {
		t.Fatal("Transport must be a *CgoTransport")
	}
}

func TestNewCgoTransportAgentStartRun(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	spec := ancora.NewAgentSpec("factory-start", "llama3", "hello")
	ag := ancora.NewCgoTransportAgent(rt, spec)
	run, err := ag.Start(context.Background())
	if err != nil {
		t.Fatalf("Start: %v", err)
	}
	if run.ID() == "" {
		t.Fatal("Start returned empty run ID")
	}
}

func TestNewCgoTransportAgentSpecPreserved(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	spec := ancora.NewAgentSpec("spec-check", "llama3", "instructions")
	ag := ancora.NewCgoTransportAgent(rt, spec)
	if ag.Spec().GetName() != "spec-check" {
		t.Fatalf("Spec name mismatch: %q", ag.Spec().GetName())
	}
}
