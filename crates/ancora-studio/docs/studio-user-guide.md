# Ancora Studio - User Guide

Ancora Studio is a local, offline-first developer tool for inspecting agent runs. It surfaces runs, traces, eval results, cost analytics, drift signals, and human feedback without requiring any network connection.

## Views

### Run List

The run list is the entry point. It shows all locally recorded runs. You can:
- Search by run label or ID
- Filter by status (completed, failed, running, cancelled)
- Filter by tag
- Sort by start time (newest first by default)

### Timeline

The timeline shows every step of a run as a horizontal swimlane. Each step shows:
- Kind (LLM call, tool call, checkpoint, user/assistant message, error)
- Duration bar (proportional to wall time)
- Token counts and cost (where available)
- Redacted steps are hidden automatically

### Step Inspector

Click any step in the timeline to open the step inspector. It shows:
- Full prompt text (if not redacted)
- Full response text (if not redacted)
- All tool calls with their input and output JSON
- Arbitrary metadata key-value pairs

### Trace Tree

The trace tree view renders the full span hierarchy of a run. Parent spans contain child spans. Hover over a span to see its attributes. Error spans are highlighted in red.

### Replay

The replay view lets you step through a run one step at a time:
- Play / Pause / Stop controls
- Step forward / Step back buttons
- Seek slider for direct navigation
- Playback speed (0.25x to 16x)

### Run Diff

Select two runs to compare them side by side. The diff view shows:
- Steps present in one run but not the other
- Steps whose prompts or responses changed
- Per-step cost delta
- Overall cost delta summary

### Eval Results

Eval results are shown per run. For each evaluation:
- Pass / Fail / Skip / Partial outcome
- Score (0.0 - 1.0)
- Optional reason string
- Overall pass rate and average score

### Cost Analytics

The cost view breaks down spending by:
- Per-step cost
- Per-model aggregate
- Total tokens in / out
- Most expensive step
- Average cost per step

### Drift View

The drift view tracks metric values over time across runs. It highlights regressions exceeding a configurable threshold percentage.

### Feedback

Human reviewers can leave thumbs-up/down ratings or numeric scores (1-5) on whole runs or individual steps. Comments and tags are supported. The feedback view shows average rating per run and surfaces all negative entries for review.

## Redaction

Redaction policies are enforced before any data reaches the UI. Three policy modes are available:
- `AlwaysRedact` - never shown regardless of viewer role
- `RequireRole(role)` - shown only to viewers with the specified role
- `NeverRedact` - always shown

Redacted fields appear as `[REDACTED]`.
