# Boot Sequence and Supervision

## Boot Phases

The headless agent boot sequence runs through six ordered phases:

| Phase | Description |
|-------|-------------|
| `init` | Process starts, signal handlers registered, PID file written |
| `config-load` | `/etc/ancora/headless.json` parsed and validated |
| `cgroup-setup` | CPU and memory limits written to cgroup v2 hierarchy |
| `model-preload` | All configured model files mapped into memory |
| `socket-bind` | Unix domain socket created and listening |
| `ready` | `READY=1` sent to systemd via `sd_notify` |

If any phase fails, the service exits non-zero and the supervisor schedules a restart.

## Service Unit

The generated unit file (`ServiceUnit::render()`) sets `Type=notify` so systemd
waits for the `READY=1` signal before declaring the service active. This ensures
dependent units do not start before the agent is truly ready.

## Supervision

The `Supervisor` struct implements exponential back-off restart:

- Initial delay: 500 ms
- Maximum delay: 30 s
- Maximum restart attempts: 10 (configurable)
- Stability window: 60 s (resets counter if process was stable for this long)

A clean exit (status 0) is not restarted. OOM kills and signal deaths are restarted.
After `max_restarts` attempts, the supervisor stops and logs a fatal event.

## PID File

A PID file is written to `pid_file_path` (default `/run/ancora/agent.pid`) before
`READY=1` is sent. The init system uses this to track the process and send
signals cleanly.
