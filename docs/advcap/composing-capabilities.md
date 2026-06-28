# Composing Advanced Capabilities

Advanced Ancora capabilities are designed to compose without coupling. Each crate
exposes independent APIs that share only primitive types (u64 ticks, strings).

## Common Composition Patterns

### Planning + Reflection

Generate a plan with `fan_out`, then measure improvement with `ReflectionMetric`:

```rust
let tasks = fan_out(orchestrator, inputs, tick)?;
let initial = tasks.len();
let final_tasks = refine_tasks(&tasks);
let score = ReflectionMetric::score(initial, final_tasks.len());
```

### Routing + Guardrails

Route inputs using `RoutingMetric`, then gate on `GuardrailPolicy`:

```rust
let route_score = RoutingMetric::score(quality, cost, max_cost);
let outcome = guardrail_policy.check_input(input, &mut journal, tick);
```

### Long-horizon + Checkpointing + Coordination

Persist state across ticks with ancora-lh while coordinating via ancora-coord:

```rust
let mut run = BackgroundRun::new("agent-1", tick);
let mut ck = Checkpoint::new("agent-1", tick);

loop {
    // Execute one tick of work
    let winner = ContractNet::assign(&bids);
    ck.set("last_winner", &winner.agent_id);
    run.apply_effect(&format!("assigned:{}", winner.task_id));
    if deadline.check(now).is_err() { break; }
}
```

### Reasoning + Citations + Eval

Run structured reasoning, track evidence, then score with ageval:

```rust
let steps = StepDecomposer::decompose(claims);
for step in steps.iter_mut() {
    let fc = FactChecker::check(&step.claim, |c| tool.lookup(c));
    if fc.grounded { citation_store.add(&step.claim, fc.source); }
    StepVerifier::verify(step, |c| fc.grounded);
}
let score = ReasoningMetric::score(verified_count, steps.len());
report.add_score(MetricScore::new("reasoning", score));
```

### Tool Synthesis + Skills + Memory

Synthesize tools dynamically, load skills on demand, consolidate memory:

```rust
let spec = registry.get("search-web").unwrap();
let result = SandboxRunner::execute(spec, &json!({"query": "Rust async"}));

let mut jit = JitLoader::new();
jit.load_on_demand(&mut skill_registry, descriptor);

let job = ConsolidationJob::new(items, salience_scorer, policy);
job.run();
```

## Anti-patterns

- Do not share mutable state between crates directly; use the journal/blackboard APIs
- Do not mix abstract ticks with wall-clock time; keep all time as u64 ticks
- Do not call external network services from any capability implementation
