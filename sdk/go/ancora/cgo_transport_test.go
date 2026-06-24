package ancora_test

import (
	"context"
	"testing"

	"ancora.io/sdk/ancora"
)

func mustCgoTransport(t *testing.T) (*ancora.CgoTransport, *ancora.Runtime) {
	t.Helper()
	rt := mustRuntime(t)
	return ancora.NewCgoTransport(rt), rt
}

func TestCgoTransportStartRunReturnsNonEmptyID(t *testing.T) {
	tr, rt := mustCgoTransport(t)
	defer rt.Free()
	spec := ancora.NewAgentSpec("cgo-agent", "llama3", "hello")
	b, err := ancora.NewAgentSpecBuilder().
		WithName(spec.GetName()).
		WithModelID(spec.GetModelId()).
		WithInstructions(spec.GetInstructions()).
		BuildBytes()
	if err != nil {
		t.Fatalf("BuildBytes: %v", err)
	}
	id, err := tr.StartRun(context.Background(), b)
	if err != nil {
		t.Fatalf("StartRun: %v", err)
	}
	if id == "" {
		t.Fatal("StartRun returned empty run ID")
	}
}

func TestCgoTransportPollRunReturnsBytes(t *testing.T) {
	tr, rt := mustCgoTransport(t)
	defer rt.Free()
	b, _ := ancora.NewAgentSpecBuilder().
		WithName("poll-agent").
		WithModelID("llama3").
		WithInstructions("say hi").
		BuildBytes()
	id, err := tr.StartRun(context.Background(), b)
	if err != nil {
		t.Fatalf("StartRun: %v", err)
	}
	ev, err := tr.PollRun(context.Background(), id)
	if err != nil {
		t.Fatalf("PollRun: %v", err)
	}
	_ = ev
}

func TestCgoTransportPollRunEmptyQueueReturnsNil(t *testing.T) {
	tr, rt := mustCgoTransport(t)
	defer rt.Free()
	b, _ := ancora.NewAgentSpecBuilder().
		WithName("drain-agent").
		WithModelID("llama3").
		WithInstructions("noop").
		BuildBytes()
	id, _ := tr.StartRun(context.Background(), b)
	for {
		ev, err := tr.PollRun(context.Background(), id)
		if err != nil {
			t.Fatalf("PollRun error: %v", err)
		}
		if ev == nil {
			break
		}
	}
}

func TestCgoTransportResumeRunAcceptsDecision(t *testing.T) {
	tr, rt := mustCgoTransport(t)
	defer rt.Free()
	b, _ := ancora.NewAgentSpecBuilder().
		WithName("resume-agent").
		WithModelID("llama3").
		WithInstructions("wait").
		BuildBytes()
	id, _ := tr.StartRun(context.Background(), b)
	err := tr.ResumeRun(context.Background(), id, []byte(`{"answer":"yes"}`))
	if err != nil {
		t.Fatalf("ResumeRun: %v", err)
	}
}

func TestCgoTransportImplementsTransportInterface(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	var _ ancora.Transport = ancora.NewCgoTransport(rt)
}

func TestNewCgoTransportReturnsNonNil(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tr := ancora.NewCgoTransport(rt)
	if tr == nil {
		t.Fatal("NewCgoTransport returned nil")
	}
}

func TestCgoTransportMultipleStartRunsHaveDifferentIDs(t *testing.T) {
	tr, rt := mustCgoTransport(t)
	defer rt.Free()
	b, _ := ancora.NewAgentSpecBuilder().
		WithName("multi-agent").
		WithModelID("llama3").
		WithInstructions("hi").
		BuildBytes()
	id1, _ := tr.StartRun(context.Background(), b)
	id2, _ := tr.StartRun(context.Background(), b)
	if id1 == id2 {
		t.Fatalf("expected distinct run IDs, both were %q", id1)
	}
}
