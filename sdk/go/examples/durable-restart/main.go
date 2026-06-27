// durable-restart demonstrates how to persist run events to SQLite and
// replay them from the store after a simulated restart, without re-running
// the agent.
package main

import (
	"context"
	"fmt"
	"os"

	"ancora.io/sdk/ancora"
)

func main() {
	dbPath := ":memory:"

	rt, err := ancora.NewRuntime()
	if err != nil {
		fmt.Fprintf(os.Stderr, "runtime: %v\n", err)
		os.Exit(1)
	}
	defer rt.Free()

	store, err := ancora.OpenSqliteStore(dbPath)
	if err != nil {
		fmt.Fprintf(os.Stderr, "open store: %v\n", err)
		os.Exit(1)
	}
	defer store.Close()

	tr := ancora.NewStoringTransport(ancora.NewCgoTransport(rt), store)
	spec := ancora.NewAgentSpec("durable-agent", "llama3", "respond and let your events be persisted")
	ag := ancora.NewTransportAgent(tr, spec)

	// --- first run ---
	run, err := ag.Start(context.Background())
	if err != nil {
		fmt.Fprintf(os.Stderr, "start: %v\n", err)
		os.Exit(1)
	}
	runID := run.ID()
	fmt.Printf("first run: %s\n", runID)

	evs, err := run.DrainEvents(context.Background())
	if err != nil {
		fmt.Fprintf(os.Stderr, "drain: %v\n", err)
		os.Exit(1)
	}
	fmt.Printf("live events: %d\n", len(evs))

	// --- simulate restart: read events from the journal ---
	stored, err := store.EventsForRun(runID)
	if err != nil {
		fmt.Fprintf(os.Stderr, "replay: %v\n", err)
		os.Exit(1)
	}
	fmt.Printf("replayed %d event(s) from journal\n", len(stored))

	total, _ := store.RunCount()
	fmt.Printf("total runs in store: %d\n", total)

	fmt.Println("durable-restart done")
}
