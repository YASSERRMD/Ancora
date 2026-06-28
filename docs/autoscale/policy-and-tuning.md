# Autoscaling Policy and Tuning Guide

The `ancora-autoscale` crate provides a pure-Rust, network-free autoscaling
policy engine. It emits `ScaleDecision` values; the caller applies them to the
actual worker pool (or forwards to a Kubernetes HPA via the scale-signal format).

## How it works

1. Collect an `AutoscaleMetrics` snapshot (queue depth, worker count, active runs, utilization).
2. Call `ScalePolicy::evaluate(&metrics)` to get a `ScaleDecision`.
3. Apply the decision: add or remove workers, or do nothing.
4. Emit a `ScaleSignal` to observability.

## Scale-up conditions

A scale-up fires when either:
- `queue_depth >= scale_up_queue_depth` (default: 5), OR
- `utilization >= scale_up_utilization` (default: 0.80)

AND the worker count is below `max_workers`, AND the scale-up cooldown has elapsed.

## Scale-down conditions

A scale-down fires when:
- `utilization <= scale_down_utilization` (default: 0.20)
- `queue_depth == 0`
- Worker count is above `min_workers`
- Scale-down cooldown has elapsed

## Cooldown windows

Cooldowns prevent flapping. Defaults:
- Scale-up cooldown: 60s
- Scale-down cooldown: 120s

Set both to `0` in tests to exercise all policies without waiting.

## Bounds

`ScaleBounds { min_workers, max_workers }` hard-clamp the desired count.
The policy never emits a decision that would violate the bounds.

## Per-tenant caps

Use `TenantCap { tenant_id, max_workers }` to limit scaling for a specific tenant.
Apply the cap after evaluating the global policy.

## Simulator

`Simulator::new(policy, initial_workers)` feeds metric ticks and returns
decisions without affecting any real infrastructure. Use it to validate a policy
against a recorded load profile before deploying.

## Scale signal for Kubernetes HPA

`ScaleSignal` is serializable. Forward it to an external metrics adapter that
exposes `desired_workers` as a custom metric for the HPA to consume.
