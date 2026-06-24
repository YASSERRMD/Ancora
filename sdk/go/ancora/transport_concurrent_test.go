package ancora_test

import (
	"context"
	"sync"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestTransportAgentConcurrentStartsReturnUniqueIDs(t *testing.T) {
	tr := newFakeGRPCTransport(t)
	spec := ancora.NewAgentSpec("concurrent-agent", "llama3", "hi")
	ag := ancora.NewTransportAgent(tr, spec)

	const n = 20
	ids := make([]string, n)
	var wg sync.WaitGroup
	wg.Add(n)
	for i := 0; i < n; i++ {
		i := i
		go func() {
			defer wg.Done()
			run, err := ag.Start(context.Background())
			if err != nil {
				t.Errorf("Start[%d]: %v", i, err)
				return
			}
			ids[i] = run.ID()
		}()
	}
	wg.Wait()

	seen := make(map[string]bool, n)
	for _, id := range ids {
		if id == "" {
			t.Fatal("empty run ID in concurrent start")
		}
		if seen[id] {
			t.Fatalf("duplicate run ID %q in concurrent starts", id)
		}
		seen[id] = true
	}
}

func TestCgoTransportConcurrentStartsReturnUniqueIDs(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tr := ancora.NewCgoTransport(rt)
	spec := ancora.NewAgentSpec("cgo-concurrent", "llama3", "hi")
	ag := ancora.NewTransportAgent(tr, spec)

	const n = 10
	ids := make([]string, n)
	var wg sync.WaitGroup
	wg.Add(n)
	for i := 0; i < n; i++ {
		i := i
		go func() {
			defer wg.Done()
			run, err := ag.Start(context.Background())
			if err != nil {
				t.Errorf("Start[%d]: %v", i, err)
				return
			}
			ids[i] = run.ID()
		}()
	}
	wg.Wait()

	seen := make(map[string]bool, n)
	for _, id := range ids {
		if id == "" {
			t.Fatal("empty run ID in concurrent cgo start")
		}
		if seen[id] {
			t.Fatalf("duplicate run ID %q in concurrent cgo starts", id)
		}
		seen[id] = true
	}
}
