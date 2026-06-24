package ancora_test

import (
	"context"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestTransportAgentNewReturnsNonNil(t *testing.T) {
	tr := newFakeGRPCTransport(t)
	spec := ancora.NewAgentSpec("ta-agent", "llama3", "hi")
	ag := ancora.NewTransportAgent(tr, spec)
	if ag == nil {
		t.Fatal("NewTransportAgent returned nil")
	}
}

func TestTransportAgentSpecAccessor(t *testing.T) {
	tr := newFakeGRPCTransport(t)
	spec := ancora.NewAgentSpec("spec-agent", "llama3", "hi")
	ag := ancora.NewTransportAgent(tr, spec)
	if ag.Spec().GetName() != "spec-agent" {
		t.Fatalf("Spec().GetName() = %q, want spec-agent", ag.Spec().GetName())
	}
}

func TestTransportAgentTransportAccessor(t *testing.T) {
	tr := newFakeGRPCTransport(t)
	spec := ancora.NewAgentSpec("acc-agent", "llama3", "hi")
	ag := ancora.NewTransportAgent(tr, spec)
	if ag.Transport() != tr {
		t.Fatal("Transport() must return the transport passed to NewTransportAgent")
	}
}

func TestTransportAgentStartReturnsNonNilRun(t *testing.T) {
	tr := newFakeGRPCTransport(t)
	spec := ancora.NewAgentSpec("start-agent", "llama3", "hi")
	ag := ancora.NewTransportAgent(tr, spec)
	run, err := ag.Start(context.Background())
	if err != nil {
		t.Fatalf("Start: %v", err)
	}
	if run == nil {
		t.Fatal("Start returned nil TransportRun")
	}
}

func TestTransportRunIDIsNonEmpty(t *testing.T) {
	tr := newFakeGRPCTransport(t)
	spec := ancora.NewAgentSpec("id-agent", "llama3", "hi")
	run, _ := ancora.NewTransportAgent(tr, spec).Start(context.Background())
	if run.ID() == "" {
		t.Fatal("TransportRun.ID() must not be empty")
	}
}

func TestTransportRunPollEventReturnsNilWhenEmpty(t *testing.T) {
	tr := newFakeGRPCTransport(t)
	spec := ancora.NewAgentSpec("poll-agent", "llama3", "hi")
	run, _ := ancora.NewTransportAgent(tr, spec).Start(context.Background())
	ev, err := run.PollEvent(context.Background())
	if err != nil {
		t.Fatalf("PollEvent: %v", err)
	}
	if ev != nil {
		t.Fatalf("expected nil event, got: %s", ev)
	}
}

func TestTransportRunResumeSucceeds(t *testing.T) {
	tr := newFakeGRPCTransport(t)
	spec := ancora.NewAgentSpec("resume-agent", "llama3", "hi")
	run, _ := ancora.NewTransportAgent(tr, spec).Start(context.Background())
	err := run.Resume(context.Background(), []byte(`{"answer":"go"}`))
	if err != nil {
		t.Fatalf("Resume: %v", err)
	}
}

func TestTransportAgentTwoStartsReturnDifferentIDs(t *testing.T) {
	tr := newFakeGRPCTransport(t)
	spec := ancora.NewAgentSpec("two-agent", "llama3", "hi")
	ag := ancora.NewTransportAgent(tr, spec)
	r1, _ := ag.Start(context.Background())
	r2, _ := ag.Start(context.Background())
	if r1.ID() == r2.ID() {
		t.Fatalf("expected distinct run IDs, both %q", r1.ID())
	}
}

func TestTransportAgentCgoBackendStartRun(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	cgoTr := ancora.NewCgoTransport(rt)
	spec := ancora.NewAgentSpec("cgo-ta", "llama3", "hello")
	ag := ancora.NewTransportAgent(cgoTr, spec)
	run, err := ag.Start(context.Background())
	if err != nil {
		t.Fatalf("Start via CgoTransport: %v", err)
	}
	if run.ID() == "" {
		t.Fatal("expected non-empty run ID via CgoTransport")
	}
}

func TestTransportRunIDUnchangedAfterPoll(t *testing.T) {
	tr := newFakeGRPCTransport(t)
	spec := ancora.NewAgentSpec("stable-id", "llama3", "hi")
	run, _ := ancora.NewTransportAgent(tr, spec).Start(context.Background())
	before := run.ID()
	run.PollEvent(context.Background())
	if run.ID() != before {
		t.Fatalf("ID changed after PollEvent: %q -> %q", before, run.ID())
	}
}
