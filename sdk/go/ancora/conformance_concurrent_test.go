package ancora_test

import (
	"context"
	"sync"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestConformanceConcurrentRunsHaveUniqueIDs(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	spec := ancora.NewAgentSpec("conc-conf", "mock", "")
	ag := ancora.NewAgent(rt, spec)

	const n = 8
	ids := make([]string, n)
	var wg sync.WaitGroup
	wg.Add(n)
	for i := 0; i < n; i++ {
		i := i
		go func() {
			defer wg.Done()
			run, err := ag.Start()
			if err != nil {
				t.Errorf("Start[%d]: %v", i, err)
				return
			}
			ids[i] = run.ID()
		}()
	}
	wg.Wait()

	seen := make(map[string]bool)
	for i, id := range ids {
		if id == "" {
			t.Fatalf("ids[%d] is empty", i)
		}
		if seen[id] {
			t.Fatalf("duplicate run ID %q at index %d", id, i)
		}
		seen[id] = true
	}
}

func TestConformanceConcurrentSuiteRunsReturnFourEach(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tr := ancora.NewCgoTransport(rt)

	const n = 3
	results := make([][]ancora.ConformanceResult, n)
	var wg sync.WaitGroup
	wg.Add(n)
	for i := 0; i < n; i++ {
		i := i
		go func() {
			defer wg.Done()
			results[i] = ancora.NewConformanceSuite(tr).RunAll(context.Background())
		}()
	}
	wg.Wait()

	for i, r := range results {
		if len(r) != 4 {
			t.Errorf("concurrent suite run %d: expected 4 results, got %d", i, len(r))
		}
	}
}
