// cost-otel wraps an agent run in lightweight span tracking to record
// event counts, run duration, and token budget -- mirroring the data an
// OpenTelemetry exporter would emit without requiring a live collector.
package main

import (
	"context"
	"fmt"
	"os"
	"time"

	"ancora.io/sdk/ancora"
)

// span is a minimal stand-in for an OTEL span.
type span struct {
	name      string
	startedAt time.Time
	attrs     map[string]any
}

func startSpan(name string) *span {
	return &span{name: name, startedAt: time.Now(), attrs: make(map[string]any)}
}

func (s *span) set(key string, val any) { s.attrs[key] = val }

func (s *span) end() {
	dur := time.Since(s.startedAt)
	fmt.Printf("span: %s  duration=%s", s.name, dur.Round(time.Millisecond))
	for k, v := range s.attrs {
		fmt.Printf("  %s=%v", k, v)
	}
	fmt.Println()
}

func main() {
	rt, err := ancora.NewRuntime()
	if err != nil {
		fmt.Fprintf(os.Stderr, "runtime: %v\n", err)
		os.Exit(1)
	}
	defer rt.Free()

	spec := ancora.NewAgentSpec("cost-agent", "llama3", "respond concisely")
	ag := ancora.NewTransportAgent(ancora.NewCgoTransport(rt), spec)

	rootSpan := startSpan("agent.run")
	defer rootSpan.end()

	run, err := ag.Start(context.Background())
	if err != nil {
		fmt.Fprintf(os.Stderr, "start: %v\n", err)
		os.Exit(1)
	}
	rootSpan.set("run.id", run.ID())
	fmt.Printf("run started: %s\n", run.ID())

	drainSpan := startSpan("agent.drain_events")
	evs, err := run.DrainEvents(context.Background())
	drainSpan.set("event.count", len(evs))
	drainSpan.end()

	if err != nil {
		fmt.Fprintf(os.Stderr, "drain: %v\n", err)
		os.Exit(1)
	}

	rootSpan.set("event.count", len(evs))
	// Token estimation: events carry raw bytes; 4 bytes ~ 1 token is a
	// common heuristic used when no usage header is available.
	totalBytes := 0
	for _, ev := range evs {
		totalBytes += len(ev)
	}
	estimatedTokens := totalBytes / 4
	rootSpan.set("tokens.estimated", estimatedTokens)

	fmt.Printf("events: %d  estimated_tokens: %d\n", len(evs), estimatedTokens)

	// Emit a summary span that aggregates the per-run metrics.
	summarySpan := startSpan("agent.summary")
	summarySpan.set("total.events", len(evs))
	summarySpan.set("total.bytes", totalBytes)
	summarySpan.set("tokens.estimated", estimatedTokens)
	summarySpan.end()

	fmt.Println("cost-otel done")
}
