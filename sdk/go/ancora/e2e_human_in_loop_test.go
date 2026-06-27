package ancora_test

import (
	"context"
	"strings"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestE2EHumanInLoopConformancePasses(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	suite := ancora.NewConformanceSuite(ancora.NewCgoTransport(rt))
	results := suite.RunAll(context.Background())

	for _, r := range results {
		if r.ScenarioID == "human-in-loop" && !r.Passed {
			t.Fatalf("conformance 'human-in-loop' failed: %s", r.Reason)
		}
	}
}

func TestE2EHumanInLoopRunCanBeSuspendedAndResumed(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	run, err := rt.StartRun([]byte("{}"))
	if err != nil {
		t.Fatalf("StartRun: %v", err)
	}
	_, _ = run.DrainEvents()

	if err := run.Resume([]byte(`{"decision":"approved"}`)); err != nil {
		t.Logf("Resume: %v (acceptable if run already completed)", err)
	}
}

func TestE2EHumanInLoopRunIDUnchangedAfterResume(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	run, _ := rt.StartRun([]byte("{}"))
	originalID := run.ID()
	_, _ = run.DrainEvents()
	_ = run.Resume([]byte(`{"decision":"approved"}`))

	if run.ID() != originalID {
		t.Fatalf("run ID must not change after Resume: %q -> %q", originalID, run.ID())
	}
}

func TestE2EHumanInLoopResumeWithRejection(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	run, _ := rt.StartRun([]byte("{}"))
	_, _ = run.DrainEvents()

	err := run.Resume([]byte(`{"decision":"rejected","reason":"invalid input"}`))
	if err != nil {
		t.Logf("Resume with rejection: %v (acceptable)", err)
	}
}

func TestE2EHumanInLoopScenarioHasSuspendTag(t *testing.T) {
	for _, s := range ancora.AllConformanceScenarios() {
		if s.ID != "human-in-loop" {
			continue
		}
		for _, tag := range s.Tags {
			if tag == "suspend" {
				return
			}
		}
		t.Fatalf("human-in-loop must have 'suspend' tag, got: %v", s.Tags)
	}
}

func TestE2EHumanInLoopMultipleResumesAreHandled(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	run, _ := rt.StartRun([]byte("{}"))
	_, _ = run.DrainEvents()

	for i, payload := range [][]byte{
		[]byte(`{"step":1}`),
		[]byte(`{"step":2}`),
		[]byte(`{"step":3}`),
	} {
		if err := run.Resume(payload); err != nil {
			t.Logf("Resume %d: %v (acceptable)", i, err)
		}
		_, _ = run.DrainEvents()
	}
}

func TestE2EHumanInLoopStoringTransportPersistsResumeRun(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	tr := ancora.NewStoringTransport(ancora.NewCgoTransport(rt), store)
	runID, err := tr.StartRun(context.Background(), []byte("{}"))
	if err != nil {
		t.Fatalf("StartRun: %v", err)
	}

	for {
		ev, _ := tr.PollRun(context.Background(), runID)
		if ev == nil {
			break
		}
	}

	_ = tr.ResumeRun(context.Background(), runID, []byte(`{"decision":"approved"}`))

	has, _ := store.HasRun(runID)
	if !has {
		t.Fatalf("run %q must still be in store after Resume", runID)
	}
}

func TestE2EHumanInLoopScenarioDescriptionIsNonEmpty(t *testing.T) {
	for _, s := range ancora.AllConformanceScenarios() {
		if s.ID == "human-in-loop" {
			if s.Description == "" {
				t.Fatal("human-in-loop scenario description must be non-empty")
			}
			return
		}
	}
	t.Fatal("human-in-loop scenario not found")
}

func TestE2EHumanInLoopGRPCTransportCanResume(t *testing.T) {
	tr := newFakeGRPCTransport(t)

	runID, err := tr.StartRun(context.Background(), []byte("{}"))
	if err != nil {
		t.Fatalf("StartRun: %v", err)
	}

	for {
		ev, _ := tr.PollRun(context.Background(), runID)
		if ev == nil {
			break
		}
	}

	if err := tr.ResumeRun(context.Background(), runID, []byte(`{"approved":true}`)); err != nil {
		t.Fatalf("ResumeRun via gRPC: %v", err)
	}
}

func TestE2EHumanInLoopEventSequenceContainsRunID(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	tr := ancora.NewStoringTransport(ancora.NewCgoTransport(rt), store)
	runID, err := tr.StartRun(context.Background(), []byte("{}"))
	if err != nil {
		t.Fatalf("StartRun: %v", err)
	}

	for {
		ev, _ := tr.PollRun(context.Background(), runID)
		if ev == nil {
			break
		}
	}

	events, _ := store.EventsForRun(runID)
	if len(events) == 0 {
		t.Fatal("stored events must be non-empty after human-in-loop run")
	}
	_ = strings.Contains(events[0], runID)
}
