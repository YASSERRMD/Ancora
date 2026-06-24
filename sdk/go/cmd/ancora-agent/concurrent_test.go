package main_test

import (
	"os/exec"
	"strings"
	"sync"
	"testing"
)

func TestSingleBinaryConcurrentRunsHaveUniqueIDs(t *testing.T) {
	bin := buildBinary(t)
	const n = 4
	ids := make([]string, n)
	var wg sync.WaitGroup
	wg.Add(n)
	for i := 0; i < n; i++ {
		i := i
		go func() {
			defer wg.Done()
			out, err := exec.Command(bin,
				"-name=conc-agent",
				"-model=llama3",
				"-instructions=hi",
			).Output()
			if err != nil {
				t.Errorf("run %d: %v", i, err)
				return
			}
			line := strings.SplitN(strings.TrimSpace(string(out)), "\n", 2)[0]
			ids[i] = strings.TrimPrefix(line, "run_id=")
		}()
	}
	wg.Wait()

	seen := make(map[string]bool)
	for _, id := range ids {
		if id == "" {
			t.Fatal("empty run ID in concurrent binary invocations")
		}
		if seen[id] {
			t.Fatalf("duplicate run ID %q in concurrent invocations", id)
		}
		seen[id] = true
	}
}
