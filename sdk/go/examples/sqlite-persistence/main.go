// sqlite-persistence runs an agent with SQLite event persistence.
// Run IDs and events are stored in example.db in the current directory.
package main

import (
	"context"
	"fmt"
	"os"

	"ancora.io/sdk/ancora"
)

func main() {
	rt, err := ancora.NewRuntime()
	if err != nil {
		fmt.Fprintf(os.Stderr, "runtime: %v\n", err)
		os.Exit(1)
	}
	defer rt.Free()

	store, err := ancora.OpenSqliteStore("example.db")
	if err != nil {
		fmt.Fprintf(os.Stderr, "open store: %v\n", err)
		os.Exit(1)
	}
	defer store.Close()

	tr := ancora.NewStoringTransport(ancora.NewCgoTransport(rt), store)
	spec := ancora.NewAgentSpec("persisted-agent", "llama3", "persist my events to sqlite")
	ag := ancora.NewTransportAgent(tr, spec)

	run, err := ag.Start(context.Background())
	if err != nil {
		fmt.Fprintf(os.Stderr, "start: %v\n", err)
		os.Exit(1)
	}
	fmt.Printf("run started: %s\n", run.ID())

	evs, err := run.DrainEvents(context.Background())
	if err != nil {
		fmt.Fprintf(os.Stderr, "drain: %v\n", err)
		os.Exit(1)
	}
	for _, ev := range evs {
		fmt.Printf("  live event: %s\n", ev)
	}

	stored, err := store.EventsForRun(run.ID())
	if err != nil {
		fmt.Fprintf(os.Stderr, "retrieve stored: %v\n", err)
		os.Exit(1)
	}
	fmt.Printf("stored %d event(s) for run %s\n", len(stored), run.ID())

	total, _ := store.RunCount()
	fmt.Printf("total runs in db: %d\n", total)
}
