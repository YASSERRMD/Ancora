// human-in-loop demonstrates a run that suspends for human approval,
// receives a decision, and continues to completion.
package main

import (
	"bufio"
	"context"
	"fmt"
	"os"
	"strings"

	"ancora.io/sdk/ancora"
)

func main() {
	rt, err := ancora.NewRuntime()
	if err != nil {
		fmt.Fprintf(os.Stderr, "runtime: %v\n", err)
		os.Exit(1)
	}
	defer rt.Free()
	tr := ancora.NewCgoTransport(rt)

	spec := ancora.NewAgentSpec("hitl-agent", "llama3", "ask the human for approval before proceeding")
	ag := ancora.NewTransportAgent(tr, spec)

	run, err := ag.Start(context.Background())
	if err != nil {
		fmt.Fprintf(os.Stderr, "start: %v\n", err)
		os.Exit(1)
	}
	fmt.Printf("run started: %s\n", run.ID())

	preEvs, err := run.DrainEvents(context.Background())
	if err != nil {
		fmt.Fprintf(os.Stderr, "drain pre-resume: %v\n", err)
		os.Exit(1)
	}
	for _, ev := range preEvs {
		fmt.Printf("  event: %s\n", ev)
	}

	decision := promptDecision(os.Stdin)
	fmt.Printf("resuming with decision: %q\n", decision)

	if err := run.Resume(context.Background(), []byte(decision)); err != nil {
		fmt.Fprintf(os.Stderr, "resume: %v\n", err)
		os.Exit(1)
	}

	postEvs, err := run.DrainEvents(context.Background())
	if err != nil {
		fmt.Fprintf(os.Stderr, "drain post-resume: %v\n", err)
		os.Exit(1)
	}
	for _, ev := range postEvs {
		fmt.Printf("  event: %s\n", ev)
	}
	fmt.Println("done")
}

// promptDecision reads a decision from r. In non-interactive mode it returns "approved".
func promptDecision(r *os.File) string {
	info, err := r.Stat()
	if err != nil || (info.Mode()&os.ModeCharDevice) == 0 {
		return "approved"
	}
	fmt.Print("Enter decision (approved/rejected): ")
	scanner := bufio.NewScanner(r)
	if scanner.Scan() {
		if d := strings.TrimSpace(scanner.Text()); d != "" {
			return d
		}
	}
	return "approved"
}
