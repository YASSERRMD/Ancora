# human-in-loop

Demonstrates the human-in-the-loop pattern: start a run, suspend, receive a
decision from stdin, resume, and run to completion.

## Run

```bash
# Non-interactive (decision defaults to "approved")
go run ./examples/human-in-loop < /dev/null

# Interactive
go run ./examples/human-in-loop
# Enter decision (approved/rejected): approved
```

## What it shows

- Pre-suspension event drain with `TransportRun.DrainEvents`
- Resuming a suspended run with `TransportRun.Resume`
- Post-resume event drain to confirm the resumed event appeared
