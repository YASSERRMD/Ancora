package ancora_test

import (
	"fmt"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestE2ELongRunStabilityOneHundredRuntimes(t *testing.T) {
	const numRuntimes = 100
	for i := 0; i < numRuntimes; i++ {
		rt := mustRuntime(t)
		rt.Free()
	}
}

func TestE2ELongRunStabilityFiveHundredStoreOperations(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	const numRuns = 50
	const eventsPerRun = 10
	for i := 0; i < numRuns; i++ {
		id := fmt.Sprintf("stability-run-%03d", i)
		_ = store.RecordRun(id)
		for j := 0; j < eventsPerRun; j++ {
			_ = store.AppendEvent(id, j, `{"type":"activity_recorded"}`)
		}
	}

	count, err := store.RunCount()
	if err != nil {
		t.Fatalf("RunCount: %v", err)
	}
	if count != numRuns {
		t.Fatalf("expected %d runs, got: %d", numRuns, count)
	}
}

func TestE2ELongRunStabilityEventCountIsCorrectForAllRuns(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	const numRuns = 20
	const eventsPerRun = 15
	for i := 0; i < numRuns; i++ {
		id := fmt.Sprintf("ec-run-%03d", i)
		_ = store.RecordRun(id)
		for j := 0; j < eventsPerRun; j++ {
			_ = store.AppendEvent(id, j, `{"n":1}`)
		}
	}

	for i := 0; i < numRuns; i++ {
		id := fmt.Sprintf("ec-run-%03d", i)
		c, err := store.EventCount(id)
		if err != nil {
			t.Fatalf("EventCount %s: %v", id, err)
		}
		if c != eventsPerRun {
			t.Fatalf("run %s expected %d events, got: %d", id, eventsPerRun, c)
		}
	}
}

func TestE2ELongRunStabilityToolRegistryWithManyTools(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	const numTools = 200
	for i := 0; i < numTools; i++ {
		name := fmt.Sprintf("tool-%04d", i)
		reg.Register(name, func(input []byte) ([]byte, error) {
			return []byte(`{"ok":true}`), nil
		})
	}
	if reg.Count() != numTools {
		t.Fatalf("expected %d tools, got: %d", numTools, reg.Count())
	}
}

func TestE2ELongRunStabilityToolkitAccumulatesTools(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tk := ancora.NewRuntimeToolkit(rt)

	const numTools = 50
	for i := 0; i < numTools; i++ {
		name := fmt.Sprintf("bulk-tool-%03d", i)
		tk.RegisterTool(name, func(input []byte) ([]byte, error) {
			return []byte(`{}`), nil
		})
	}

	if tk.Tools().Count() != numTools {
		t.Fatalf("toolkit expected %d tools, got: %d", numTools, tk.Tools().Count())
	}
}

func TestE2ELongRunStabilityListRunsAfterManyRecords(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	const numRuns = 30
	for i := 0; i < numRuns; i++ {
		_ = store.RecordRun(fmt.Sprintf("list-run-%03d", i))
	}

	ids, err := store.ListRuns()
	if err != nil {
		t.Fatalf("ListRuns: %v", err)
	}
	if len(ids) < numRuns {
		t.Fatalf("expected at least %d run IDs, got: %d", numRuns, len(ids))
	}
}

func TestE2ELongRunStabilityDeleteAllRuns(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	const numRuns = 15
	ids := make([]string, numRuns)
	for i := range ids {
		ids[i] = fmt.Sprintf("del-run-%03d", i)
		_ = store.RecordRun(ids[i])
	}
	for _, id := range ids {
		_ = store.DeleteRun(id)
	}

	count, _ := store.RunCount()
	if count != 0 {
		t.Fatalf("expected 0 runs after deleting all, got: %d", count)
	}
}
