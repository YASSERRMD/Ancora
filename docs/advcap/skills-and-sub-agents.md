# Skills and Sub-agents

ancora-skills provides a versioned registry of reusable agent skills, JIT loading,
sub-agent invocation with permission scoping, and a skill invocation journal.

## Skill Descriptor

```rust
use ancora_skills::{SkillDescriptor, SkillScope};

let skill = SkillDescriptor {
    name: "search".into(),
    version: 1,
    description: "web search".into(),
    capability_tags: vec!["search".into(), "retrieval".into()],
    input_schema: r#"{"type":"object"}"#.into(),
    permission_scope: SkillScope::ReadOnly,
};
```

## Registry and JIT Loading

```rust
use ancora_skills::{SkillRegistry, JitLoader};

let mut registry = SkillRegistry::default();
let mut loader = JitLoader::new();
loader.load_on_demand(&mut registry, skill).unwrap();

// Idempotent: calling again with same name does nothing
loader.load_on_demand(&mut registry, duplicate).unwrap();
assert_eq!(loader.loaded_count(), 1);
```

## Sub-agent Invocation

```rust
use ancora_skills::{SubAgentNode, SkillScope};

let node = SubAgentNode { node_id: "node-1".into(), agent_id: "summarizer".into(), input: "text".into() };
let result = node.invoke(SkillScope::ReadOnly); // Ok
let blocked = node.invoke(SkillScope::Unrestricted); // Err: blocked for sub-agents
```

## Permission Scopes

| Scope | Sub-agent allowed |
|---|---|
| `ReadOnly` | Yes |
| `LocalWrite` | Yes |
| `Unrestricted` | No |

## Crew Composition

```rust
use ancora_skills::Crew;
let crew = Crew { name: "search-crew".into(), skill_names: vec!["search".into()] };
let skills = crew.resolve(&registry)?;
```
