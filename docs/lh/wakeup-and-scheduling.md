# Wakeup and Scheduling Patterns

## Scheduled Wakeup

Use `ScheduledWakeup` when the next action time is known ahead of time.
The run sleeps and is checked against `should_fire(now)` on each tick.

## Event-Driven Wakeup

Use `EventWakeup` when an external system (data pipeline, webhook, timer)
triggers the run. Inject a signal into `SignalQueue` and process it on the
next agent tick.

## Combining Wakeups

A run can have both a scheduled wakeup (maximum sleep time) and an event
wakeup (early exit) simultaneously. Check both on each tick and wake on
whichever fires first.

## Signal Injection

External signals are injected via `SignalQueue::inject` and consumed with
`pop()`. Pop at the start of each tick so the run always sees the latest
signal before making decisions.

## Checkpoint Cadence

Set `interval_ticks` to a multiple of expected wakeup intervals to ensure
at least one checkpoint between wakeups. This bounds replay cost on restart.
