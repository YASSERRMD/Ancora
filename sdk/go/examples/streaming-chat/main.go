// streaming-chat consumes agent events one at a time via EventChan,
// printing each event as it arrives rather than waiting for all of them.
package main

import (
	"context"
	"fmt"
	"os"
	"time"

	"ancora.io/sdk/ancora"
)

func main() {
	rt, err := ancora.NewRuntime()
	if err != nil {
		fmt.Fprintf(os.Stderr, "runtime: %v\n", err)
		os.Exit(1)
	}
	defer rt.Free()

	spec := ancora.NewAgentSpec("chat-agent", "llama3", "respond in short bursts")
	ag := ancora.NewTransportAgent(ancora.NewCgoTransport(rt), spec)

	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	run, err := ag.Start(ctx)
	if err != nil {
		fmt.Fprintf(os.Stderr, "start: %v\n", err)
		os.Exit(1)
	}
	fmt.Printf("stream started: %s\n", run.ID())

	ch := run.EventChan(ctx)
	count := 0
	for ev := range ch {
		count++
		fmt.Printf("[%d] %s\n", count, ev)
	}
	fmt.Printf("stream done: %d event(s)\n", count)
}
