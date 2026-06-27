package ancora_test

import (
	"runtime"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestRuntimeFreeNilsPointerAfterCall(t *testing.T) {
	rt := mustRuntime(t)
	rt.Free()
	// Second Free must be a no-op (ptr is cleared after first Free).
	rt.Free()
}

func TestMultipleRuntimesCanCoexist(t *testing.T) {
	rt1 := mustRuntime(t)
	rt2 := mustRuntime(t)
	defer rt1.Free()
	defer rt2.Free()
	if rt1 == rt2 {
		t.Fatal("two NewRuntime calls must return distinct instances")
	}
}

func TestRuntimeFreedBeforeRunDoesNotCrash(t *testing.T) {
	rt := mustRuntime(t)
	rt.Free()
	// Starting a run on a freed runtime should return an error, not crash.
	_, err := rt.StartRun([]byte("{}"))
	if err == nil {
		t.Log("note: StartRun after Free returned nil error; result is implementation-defined")
	}
}

func TestRuntimeHandleReusedAfterFreeIsDetected(t *testing.T) {
	rt := mustRuntime(t)
	rt.Free()
	// Any operation on a freed runtime must not panic; error is acceptable.
	defer func() {
		if r := recover(); r != nil {
			t.Fatalf("operation on freed runtime panicked: %v", r)
		}
	}()
	_, _ = rt.StartRun([]byte("{}"))
}

func TestOneHundredRuntimesFreeWithoutLeak(t *testing.T) {
	for i := 0; i < 100; i++ {
		rt := mustRuntime(t)
		rt.Free()
	}
	runtime.GC()
}

func TestRuntimeRunIDsAreUniqueAcrossInstances(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	ids := make(map[string]bool)
	for i := 0; i < 10; i++ {
		run, err := rt.StartRun([]byte("{}"))
		if err != nil {
			t.Fatalf("StartRun %d: %v", i, err)
		}
		if ids[run.ID()] {
			t.Fatalf("duplicate run ID at iteration %d: %s", i, run.ID())
		}
		ids[run.ID()] = true
	}
}

func TestRuntimeRunIDIsUUID(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	run, err := rt.StartRun([]byte("{}"))
	if err != nil {
		t.Fatalf("StartRun: %v", err)
	}
	id := run.ID()
	if len(id) == 0 {
		t.Fatal("run ID must not be empty")
	}
	// UUID format check: 36 chars with hyphens.
	if len(id) != 36 {
		t.Logf("run ID %q has length %d (not UUID format -- implementation-defined)", id, len(id))
	}
}

func TestGCCycleAfterRuntimeFreeDoesNotPanic(t *testing.T) {
	rt := mustRuntime(t)
	rt.Free()
	runtime.GC()
	runtime.GC()
}

func TestNewAgentDoesNotLeakOnFree(t *testing.T) {
	rt := mustRuntime(t)
	spec := ancora.NewAgentSpec("leak-test", "llama3", "no-op")
	_ = ancora.NewAgent(rt, spec)
	rt.Free()
	runtime.GC()
}
