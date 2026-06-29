# Headless OS Integration Guide

`ancora-headless` enables running the Ancora agent framework as a first-class
system service in a headless inference OS — no GUI, no external network, minimal
footprint.

## Overview

The crate provides:

- **Init service integration** — systemd-compatible unit descriptor and sd_notify readiness signalling.
- **Boot-time agent service** — ordered boot sequence from config load through model preload to socket bind.
- **Local model preload** — models are mapped into memory at boot so first-token latency is zero.
- **Resource cgroup limits** — CPU and memory quotas applied via cgroup v2.
- **Service supervision** — exponential back-off restart with configurable retry limits.
- **Local socket API** — Unix domain socket for IPC; no TCP listener opened by default.
- **No external network** — egress blocked by default; all inference is fully air-gapped.

## Quick Start

1. Install the binary to `/usr/local/bin/ancora-headless`.
2. Place a config at `/etc/ancora/headless.json`.
3. Copy the generated unit file to `/etc/systemd/system/ancora-agent.service`.
4. `systemctl daemon-reload && systemctl enable --now ancora-agent`.

## Configuration

See `config.rs` for all fields. A minimal example:

```json
{
  "profile": "minimal",
  "socket_path": "/run/ancora/agent.sock",
  "model_paths": ["/opt/models/agent-q4.gguf"],
  "cgroup_memory_mb": 512,
  "cgroup_cpu_percent": 50,
  "boot_target_ms": 5000,
  "deny_external_network": true,
  "max_restarts": 10,
  "write_pid_file": true,
  "pid_file_path": "/run/ancora/agent.pid"
}
```
