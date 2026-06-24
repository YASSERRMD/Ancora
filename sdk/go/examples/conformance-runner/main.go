// conformance-runner runs all canonical conformance scenarios against the
// local CGO transport and prints a pass/fail summary.
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

	tr := ancora.NewCgoTransport(rt)
	suite := ancora.NewConformanceSuite(tr)

	results := suite.RunAll(context.Background())

	passed := 0
	failed := 0
	for _, r := range results {
		status := "PASS"
		if !r.Passed {
			status = "FAIL"
			failed++
			fmt.Printf("  [%s] %s: %s\n", status, r.ScenarioID, r.Reason)
		} else {
			passed++
			fmt.Printf("  [%s] %s\n", status, r.ScenarioID)
		}
	}
	fmt.Printf("\nconformance: %d passed, %d failed\n", passed, failed)
	if failed > 0 {
		os.Exit(1)
	}
}
