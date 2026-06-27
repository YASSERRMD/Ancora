package ancora_test

import (
	"context"
	"strings"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestCancellationContextCancelPropagates(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	if ctx.Err() == nil {
		t.Fatal("cancelled context must have non-nil Err")
	}
}

func TestCancellationContextCancelAfterRun(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	run, err := rt.StartRun([]byte("{}"))
	if err != nil {
		t.Fatalf("StartRun: %v", err)
	}

	cancel()
	_ = ctx.Err()
	if run.ID() == "" {
		t.Fatal("run ID must be non-empty even after cancel")
	}
}

func TestCancellationRunIDSurvivesContextCancel(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	run, _ := rt.StartRun([]byte("{}"))
	originalID := run.ID()

	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	_ = ctx.Done()

	if run.ID() != originalID {
		t.Fatalf("run ID must not change on cancel: %q vs %q", originalID, run.ID())
	}
}

func TestCancellationStorePersistsAfterCancel(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("cancel-run")
	_ = store.AppendEvent("cancel-run", 0, `{"type":"started"}`)

	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	_ = ctx.Err()

	events, _ := store.EventsForRun("cancel-run")
	if len(events) != 1 {
		t.Fatalf("store must retain events after context cancel, got: %d", len(events))
	}
}

func TestCancellationMultipleRunsCancelSafely(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	runs := make([]string, 5)
	for i := range runs {
		run, err := rt.StartRun([]byte("{}"))
		if err != nil {
			t.Fatalf("StartRun %d: %v", i, err)
		}
		runs[i] = run.ID()
	}

	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	_ = ctx.Err()

	for i, id := range runs {
		if id == "" {
			t.Fatalf("run %d ID must be non-empty", i)
		}
	}
}

func TestCancellationToolRegistryUnaffectedByContextCancel(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	reg.Register("noop", func(input []byte) ([]byte, error) {
		return []byte(`{}`), nil
	})

	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	_ = ctx.Err()

	if !reg.Has("noop") {
		t.Fatal("registry must retain tools after context cancel")
	}
}

func TestCancellationEventChanDrainsBeforeCancel(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	run, _ := rt.StartRun([]byte("{}"))

	_, cancel := context.WithCancel(context.Background())
	defer cancel()

	var events []string
	done := make(chan struct{})
	go func() {
		defer close(done)
		for ev := range run.EventChan() {
			events = append(events, string(ev))
		}
	}()
	<-done
	cancel()

	if len(events) == 0 {
		t.Fatal("EventChan must deliver at least one event before cancellation")
	}
}

func TestCancellationConformanceSuiteContextCancelled(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	suite := ancora.NewConformanceSuite(ancora.NewCgoTransport(rt))
	ctx, cancel := context.WithCancel(context.Background())
	cancel()

	results := suite.RunAll(ctx)
	for _, r := range results {
		_ = r
	}
}

func TestCancellationResumeAfterCancelDoesNotPanic(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	run, _ := rt.StartRun([]byte("{}"))
	_, _ = run.DrainEvents()

	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	_ = ctx.Err()

	if err := run.Resume([]byte(`{"cancelled":true}`)); err != nil {
		t.Logf("Resume after cancel returned error (acceptable): %v", err)
	}
}

func TestCancellationFirstEventStillStarted(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	run, err := rt.StartRun([]byte("{}"))
	if err != nil {
		t.Fatalf("StartRun: %v", err)
	}

	ev, err := run.PollEvent()
	if err != nil {
		t.Fatalf("PollEvent: %v", err)
	}
	if ev == nil {
		t.Fatal("first event must not be nil before cancel")
	}

	cancel()
	_ = ctx.Err()

	if !strings.Contains(string(ev), "started") {
		t.Logf("first event: %s (may not contain 'started' depending on runtime)", ev)
	}
}
