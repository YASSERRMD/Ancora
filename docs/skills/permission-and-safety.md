# Permission and Safety Notes

## Skill Scopes

| Scope | Allowed effects |
|---|---|
| ReadOnly | No side effects |
| LocalWrite | In-process writes only |
| Unrestricted | Not permitted for sub-agents |

Sub-agent nodes reject `Unrestricted` scope at invocation time.

## JIT Scope Bounding

Loading a skill via `JitLoader::load_on_demand` is idempotent. A skill is
loaded only once regardless of how many times it is requested, preventing
unbounded context growth.

## Journaling and Replay

Every skill invocation is recorded in `SkillJournal`. Replay produces the
exact same (name, version) sequence, enabling deterministic audit.
