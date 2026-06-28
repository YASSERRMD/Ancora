// advanced-parity prints canonical values for all 7 advanced metrics so that
// implementations in other languages can validate numeric parity.
package main

import (
	"fmt"
	"math"
)

// PlanningScore returns matched / expected (1.0 if expected is 0).
func PlanningScore(expected, matched int) float64 {
	if expected == 0 {
		return 1.0
	}
	return float64(matched) / float64(expected)
}

// ReflectionScore returns 1.0 if after grew, 0.5 if changed but shorter, 0.0 if same.
func ReflectionScore(before, after string) float64 {
	if before == after {
		return 0.0
	}
	if len(after) > len(before) {
		return 1.0
	}
	return 0.5
}

// RoutingScore returns (quality + (1 - cost/maxCost)) / 2.
func RoutingScore(quality float64, cost, maxCost int) float64 {
	if maxCost == 0 {
		return quality
	}
	efficiency := 1.0 - float64(cost)/float64(maxCost)
	return (quality + efficiency) / 2.0
}

// CoordinationScore returns completed / assigned.
func CoordinationScore(assigned, completed int) float64 {
	if assigned == 0 {
		return 1.0
	}
	return float64(completed) / float64(assigned)
}

// GuardrailScore returns triggered / total.
func GuardrailScore(triggered, total int) float64 {
	if total == 0 {
		return 1.0
	}
	return float64(triggered) / float64(total)
}

// ReasoningScore returns verified / total.
func ReasoningScore(verified, total int) float64 {
	if total == 0 {
		return 1.0
	}
	return float64(verified) / float64(total)
}

// MemoryScore returns retained / total.
func MemoryScore(retained, total int) float64 {
	if total == 0 {
		return 1.0
	}
	return float64(retained) / float64(total)
}

func approxEqual(a, b float64) bool {
	return math.Abs(a-b) < 1e-9
}

func check(name string, got, want float64) {
	if !approxEqual(got, want) {
		fmt.Printf("FAIL %s: got %.6f want %.6f\n", name, got, want)
	} else {
		fmt.Printf("ok   %s = %.6f\n", name, got)
	}
}

func main() {
	fmt.Println("=== Ancora Advanced Parity (Go) ===")

	check("planning_3of4",      PlanningScore(4, 3),                 0.75)
	check("reflection_grew",    ReflectionScore("short", "longer answer"), 1.0)
	check("reflection_shrink",  ReflectionScore("longer text here", "short"), 0.5)
	check("reflection_same",    ReflectionScore("x", "x"),           0.0)
	check("routing_0.9_300",    RoutingScore(0.9, 300, 1000),        0.8)
	check("routing_no_cost",    RoutingScore(0.85, 0, 1000),         0.925)
	check("coordination_3of3",  CoordinationScore(3, 3),             1.0)
	check("guardrail_1of2",     GuardrailScore(1, 2),                0.5)
	check("reasoning_4of5",     ReasoningScore(4, 5),                0.8)
	check("memory_9of10",       MemoryScore(9, 10),                  0.9)

	fmt.Println("=== done ===")
}
