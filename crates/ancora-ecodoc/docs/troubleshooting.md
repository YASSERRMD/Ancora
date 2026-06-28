# Troubleshooting

## trbl-001 - Plugin fails to load with `symbol not found`

**Severity:** Error

**Cause:** ABI mismatch between the plugin and the host runtime.

**Resolution:** Recompile the plugin against the same Ancora version as the host.

---

## trbl-002 - Capability request silently denied

**Severity:** Warning

**Cause:** Plugin trust level does not permit the requested capability.

**Resolution:** Raise the plugin trust level or remove the capability request.

---

## trbl-003 - Graph build panics with `cycle detected`

**Severity:** Error

**Cause:** The task graph contains a cycle.

**Resolution:** Use `TaskGraph::has_cycle()` to detect cycles before running.

---

## trbl-004 - CLI command not visible in `ancora help`

**Severity:** Info

**Cause:** The plugin was registered after the CLI registry was frozen.

**Resolution:** Register CLI commands in the plugin `Init` event handler.
