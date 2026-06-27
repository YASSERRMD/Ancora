package ancora_test

import (
	"sync"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestConcurrentRunsHaveUniqueIDs(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	const numRuns = 20
	ids := make([]string, numRuns)
	var wg sync.WaitGroup
	var mu sync.Mutex

	for i := 0; i < numRuns; i++ {
		wg.Add(1)
		go func(idx int) {
			defer wg.Done()
			run, err := rt.StartRun([]byte("{}"))
			if err != nil {
				t.Errorf("StartRun %d: %v", idx, err)
				return
			}
			mu.Lock()
			ids[idx] = run.ID()
			mu.Unlock()
		}(i)
	}
	wg.Wait()

	seen := make(map[string]bool)
	for i, id := range ids {
		if id == "" {
			t.Errorf("run %d has empty ID", i)
			continue
		}
		if seen[id] {
			t.Fatalf("duplicate run ID: %q", id)
		}
		seen[id] = true
	}
}

func TestConcurrentRunsDoNotShareEvents(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	const numRuns = 5
	results := make([][]string, numRuns)
	var wg sync.WaitGroup

	for i := 0; i < numRuns; i++ {
		wg.Add(1)
		go func(idx int) {
			defer wg.Done()
			run, err := rt.StartRun([]byte("{}"))
			if err != nil {
				t.Errorf("StartRun %d: %v", idx, err)
				return
			}
			events, err := run.DrainEvents()
			if err != nil {
				t.Errorf("DrainEvents %d: %v", idx, err)
				return
			}
			results[idx] = events
		}(i)
	}
	wg.Wait()

	for i, events := range results {
		if len(events) == 0 {
			t.Errorf("run %d must have at least one event", i)
		}
	}
}

func TestConcurrentStoreWritesAreIsolated(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	const numRuns = 10
	var wg sync.WaitGroup
	for i := 0; i < numRuns; i++ {
		wg.Add(1)
		runID := "concurrent-run-" + string(rune('A'+i))
		go func(id string) {
			defer wg.Done()
			_ = store.RecordRun(id)
			_ = store.AppendEvent(id, 0, `{"type":"started"}`)
		}(runID)
	}
	wg.Wait()

	count, err := store.RunCount()
	if err != nil {
		t.Fatalf("RunCount: %v", err)
	}
	if count != numRuns {
		t.Fatalf("expected %d runs, got: %d", numRuns, count)
	}
}

func TestConcurrentRuntimeToolkitRegistrations(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tk := ancora.NewRuntimeToolkit(rt)

	const numTools = 10
	var wg sync.WaitGroup
	for i := 0; i < numTools; i++ {
		wg.Add(1)
		name := "tool-" + string(rune('A'+i))
		go func(toolName string) {
			defer wg.Done()
			tk.RegisterTool(toolName, func(input []byte) ([]byte, error) {
				return []byte(`{"ok":true}`), nil
			})
		}(name)
	}
	wg.Wait()

	if tk.Tools().Count() != numTools {
		t.Fatalf("expected %d tools, got: %d", numTools, tk.Tools().Count())
	}
}

func TestConcurrentRegistryInvocations(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	reg.Register("echo", func(input []byte) ([]byte, error) {
		return input, nil
	})

	const numCalls = 50
	var wg sync.WaitGroup
	errors := make(chan error, numCalls)

	for i := 0; i < numCalls; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			_, err := reg.Invoke("echo", []byte(`{"n":1}`))
			if err != nil {
				errors <- err
			}
		}()
	}
	wg.Wait()
	close(errors)

	for err := range errors {
		t.Fatalf("concurrent Invoke error: %v", err)
	}
}

func TestConcurrentStoreEventCounts(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("concurrent-events")

	const numEvents = 20
	var wg sync.WaitGroup
	for i := 0; i < numEvents; i++ {
		wg.Add(1)
		seq := i
		go func() {
			defer wg.Done()
			_ = store.AppendEvent("concurrent-events", seq, `{"n":1}`)
		}()
	}
	wg.Wait()

	count, err := store.EventCount("concurrent-events")
	if err != nil {
		t.Fatalf("EventCount: %v", err)
	}
	if count != numEvents {
		t.Fatalf("expected %d events, got: %d", numEvents, count)
	}
}

func TestConcurrentRunsEachHaveNonEmptyFirstEvent(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	const numRuns = 8
	var wg sync.WaitGroup
	errs := make(chan string, numRuns)

	for i := 0; i < numRuns; i++ {
		wg.Add(1)
		go func(idx int) {
			defer wg.Done()
			run, err := rt.StartRun([]byte("{}"))
			if err != nil {
				errs <- "StartRun: " + err.Error()
				return
			}
			ev, err := run.PollEvent()
			if err != nil {
				errs <- "PollEvent: " + err.Error()
				return
			}
			if ev == nil || len(ev) == 0 {
				errs <- "empty first event"
			}
		}(i)
	}
	wg.Wait()
	close(errs)
	for msg := range errs {
		t.Fatal(msg)
	}
}
