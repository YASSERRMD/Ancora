use crate::playbook::Playbook;

pub fn high_error_rate() -> Playbook {
    Playbook::new("high-error-rate", "error_rate > 5% for 5 minutes")
        .add_step(
            "Check Prometheus for the top error by label",
            "Identify which tenant or model is producing errors",
            "Check ancora-alerting logs for dedup/silence issues",
        )
        .add_step(
            "Check provider circuit breaker state",
            "If open, wait for half-open probe or manually trigger",
            "If all providers failed, enable emergency mode",
        )
        .add_step(
            "Check worker logs for panics or OOM",
            "Clean worker logs at expected throughput",
            "Drain and restart the affected worker pod",
        )
}

pub fn worker_down() -> Playbook {
    Playbook::new("worker-down", "missed heartbeats > 3")
        .add_step(
            "Confirm worker pod is unreachable (kubectl get pod)",
            "Pod shows CrashLoopBackOff or Terminating",
            "Check control plane for false positive",
        )
        .add_step(
            "Drain worker via WorkerRegistry::drain(worker_id)",
            "in-flight runs are cancelled and requeued",
            "Manually cancel stuck runs with RunStore::cancel",
        )
        .add_step(
            "Restart pod (kubectl rollout restart)",
            "Pod comes up healthy, readiness probe passes",
            "Scale up replacement pod and investigate root cause",
        )
}

pub fn queue_backlog() -> Playbook {
    Playbook::new("queue-backlog", "queue_depth > 1000 for 2 minutes")
        .add_step(
            "Check worker count and utilization",
            "Workers at < 80% utilization",
            "Scale workers horizontally",
        )
        .add_step(
            "Check for stuck runs blocking workers",
            "No runs older than timeout_secs",
            "Cancel stuck runs, requeue via AutoRequeue",
        )
}

pub fn all_playbooks() -> Vec<Playbook> {
    vec![high_error_rate(), worker_down(), queue_backlog()]
}
