# Authoring Skills and Sub-Agents

ancora-skills provides a discoverable, versioned, scoped, composable registry
of skills and sub-agents.

## Skill Descriptor

```rust
let skill = SkillDescriptor::new(
    "search",
    1,                     // version
    "Keyword search over a corpus",
    vec!["retrieval"],     // capability tags
    SkillScope::ReadOnly,
);
```

## Sub-Agent Node

```rust
let node = SubAgentNode::new("n1", "agent-search", json!({"query": "rust async"}));
let result = node.invoke(&SkillScope::ReadOnly)?;
```

## Skill Composition

```rust
let crew = Crew::new("research", vec!["search", "summarize"]);
let skills = crew.resolve(&registry)?;
```

## JIT Loading

```rust
let mut loader = JitLoader::new();
loader.load_on_demand(&mut registry, skill)?;
```

Only load skills when they are actually needed to minimize context overhead.
