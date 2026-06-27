package ancora_test

import (
	"testing"

	"ancora.io/sdk/ancora"
)

func BenchmarkFFIStartRun(b *testing.B) {
	rt := mustRuntimeB(b)
	defer rt.Free()

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		run, err := rt.StartRun([]byte("{}"))
		if err != nil {
			b.Fatalf("StartRun: %v", err)
		}
		_ = run.ID()
	}
}

func BenchmarkFFIPollEvent(b *testing.B) {
	rt := mustRuntimeB(b)
	defer rt.Free()

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		run, _ := rt.StartRun([]byte("{}"))
		ev, err := run.PollEvent()
		if err != nil {
			b.Fatalf("PollEvent: %v", err)
		}
		_ = ev
	}
}

func BenchmarkFFIDrainEvents(b *testing.B) {
	rt := mustRuntimeB(b)
	defer rt.Free()

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		run, _ := rt.StartRun([]byte("{}"))
		events, err := run.DrainEvents()
		if err != nil {
			b.Fatalf("DrainEvents: %v", err)
		}
		_ = events
	}
}

func BenchmarkFFIAgentSpecBuild(b *testing.B) {
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		spec := ancora.NewAgentSpecBuilder().
			WithName("bench-agent").
			WithModelID("llama3").
			WithInstructions("benchmark instructions").
			Build()
		_ = spec
	}
}

func BenchmarkFFIAgentSpecBuildBytes(b *testing.B) {
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_, err := ancora.NewAgentSpecBuilder().
			WithName("bench-agent").
			WithModelID("llama3").
			BuildBytes()
		if err != nil {
			b.Fatalf("BuildBytes: %v", err)
		}
	}
}

func BenchmarkFFIToolRegistryInvoke(b *testing.B) {
	reg := ancora.NewGoToolRegistry()
	reg.Register("bench-tool", func(input []byte) ([]byte, error) {
		return []byte(`{"ok":true}`), nil
	})

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		out, err := reg.Invoke("bench-tool", []byte(`{}`))
		if err != nil {
			b.Fatalf("Invoke: %v", err)
		}
		_ = out
	}
}

func BenchmarkFFIStoreAppendEvent(b *testing.B) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		b.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()
	_ = store.RecordRun("bench-run")

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_ = store.AppendEvent("bench-run", i, `{"type":"activity_recorded","key":"step"}`)
	}
}

func BenchmarkFFIStoreEventCount(b *testing.B) {
	store, _ := ancora.OpenSqliteStore(":memory:")
	defer store.Close()
	_ = store.RecordRun("bench-count")
	for i := 0; i < 100; i++ {
		_ = store.AppendEvent("bench-count", i, `{"n":1}`)
	}

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		c, _ := store.EventCount("bench-count")
		_ = c
	}
}

// mustRuntimeB is a benchmark-safe variant of mustRuntime.
func mustRuntimeB(b *testing.B) *ancora.Runtime {
	b.Helper()
	rt, err := ancora.NewRuntime()
	if err != nil {
		b.Fatalf("NewRuntime: %v", err)
	}
	return rt
}
