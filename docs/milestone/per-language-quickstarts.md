# Per-language Advanced Quickstarts

## Rust (canonical)

```toml
# Cargo.toml
[dependencies]
ancora-preset = { path = "crates/ancora-preset" }
ancora-ageval = { path = "crates/ancora-ageval" }
```

```rust
use ancora_preset::{assemble, research_assistant};
use ancora_ageval::{PlanningMetric, ReflectionMetric};

fn main() {
    // Assemble a preset
    let spec = assemble(&research_assistant()).expect("preset valid");
    println!("Agent: {}", spec.agent_id);

    // Evaluate behavior
    let q = PlanningMetric::score(&["a".into(), "b".into()], &["a".into()]);
    println!("Planning quality: {q}");
}
```

## Go

```bash
cd sdk/go
go run ./examples/advanced-parity/
```

Verify all 10 lines print `ok`:

```
ok   planning_3of4 = 0.750000
ok   reflection_grew = 1.000000
...
```

## Python (reference)

```python
def planning_score(expected, matched):
    return 1.0 if not expected else len(matched) / len(expected)

def routing_score(quality, cost, max_cost):
    if max_cost == 0:
        return quality
    return (quality + (1 - cost / max_cost)) / 2

# Validate against canonical values
assert abs(planning_score([1,2,3,4], [1,2,3]) - 0.75) < 1e-9
assert abs(routing_score(0.9, 300, 1000) - 0.8) < 1e-9
```

## TypeScript

```typescript
function planningScore(expected: number, matched: number): number {
  if (expected === 0) return 1.0;
  return matched / expected;
}

function routingScore(quality: number, cost: number, maxCost: number): number {
  if (maxCost === 0) return quality;
  return (quality + (1 - cost / maxCost)) / 2;
}

console.assert(Math.abs(planningScore(4, 3) - 0.75) < 1e-9);
console.assert(Math.abs(routingScore(0.9, 300, 1000) - 0.8) < 1e-9);
```

## .NET

```csharp
static double PlanningScore(int expected, int matched) =>
    expected == 0 ? 1.0 : (double)matched / expected;

static double RoutingScore(double quality, int cost, int maxCost) {
    if (maxCost == 0) return quality;
    return (quality + (1.0 - (double)cost / maxCost)) / 2.0;
}

Debug.Assert(Math.Abs(PlanningScore(4, 3) - 0.75) < 1e-9);
Debug.Assert(Math.Abs(RoutingScore(0.9, 300, 1000) - 0.8) < 1e-9);
```

## Java

```java
static double planningScore(int expected, int matched) {
    return expected == 0 ? 1.0 : (double) matched / expected;
}

static double routingScore(double quality, int cost, int maxCost) {
    if (maxCost == 0) return quality;
    return (quality + (1.0 - (double) cost / maxCost)) / 2.0;
}

assert Math.abs(planningScore(4, 3) - 0.75) < 1e-9;
assert Math.abs(routingScore(0.9, 300, 1000) - 0.8) < 1e-9;
```
