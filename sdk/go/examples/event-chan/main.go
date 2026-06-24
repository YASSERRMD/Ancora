// event-chan demonstrates consuming agent events through a Go channel
// instead of the drain-all DrainEvents approach.
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

	spec := ancora.NewAgentSpec("chan-agent", "llama3", "stream your events")
	ag := ancora.NewTransportAgent(ancora.NewCgoTransport(rt), spec)

	run, err := ag.Start(context.Background())
	if err != nil {
		fmt.Fprintf(os.Stderr, "start: %v\n", err)
		os.Exit(1)
	}
	fmt.Printf("run started: %s\n", run.ID())

	ch := run.EventChan(context.Background())
	count := 0
	for ev := range ch {
		count++
		fmt.Printf("  event[%d]: %s\n", count, ev)
	}
	fmt.Printf("received %d event(s)\n", count)
}
