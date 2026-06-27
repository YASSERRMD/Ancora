# human_in_loop

Starts a run, drains pre-resume events, reads a decision from stdin,
then resumes the run and drains post-resume events.
Runs fully offline; pass empty stdin for non-interactive use.

## Run

```bash
cd sdk/python

# Non-interactive -- decision defaults to empty bytes
python -m examples.human_in_loop < /dev/null

# Interactive
python -m examples.human_in_loop
```

## What it shows

- Pausing a run at a human-in-the-loop gate with `run.resume()`
- Reading a decision from stdin
- Continuing after the decision and collecting the remaining events
