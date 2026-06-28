# Per-Language Quickstarts: Observability and Eval

## Rust Quickstart: Observability Setup

### Install
Add the crate to your `Cargo.toml`.
```
cargo add ancora-observability
```

### Initialize
Call `init()` at the start of your application.
```rust
use ancora_observability::ObsConfig;
ObsConfig::default().init();
```

### Export traces
```rust
use ancora_trace::span;
let _span = span!("my-operation");
```

---

## Python Quickstart: Observability Setup

### Install
```
pip install ancora-obs
```

### Initialize
```python
from ancora_obs import ObsConfig
ObsConfig().init()
```

### Export metrics
```python
from ancora_obs import Counter
c = Counter("requests_total")
c.inc()
```

---

## Go Quickstart: Observability Setup

### Install
```
go get github.com/YASSERRMD/ancora/sdk/go/ancora-obs
```

### Initialize
```go
import "github.com/YASSERRMD/ancora/sdk/go/ancora-obs"
obs.Init(obs.DefaultConfig())
```

---

## TypeScript Quickstart: Observability Setup

### Install
```
npm install @ancora/obs
```

### Initialize
```typescript
import { ObsConfig } from "@ancora/obs";
new ObsConfig().init();
```

### Record an eval score
```typescript
import { recordEval } from "@ancora/obs";
recordEval({ name: "factual-accuracy", score: 0.92 });
```

---

## Eval Library Link

The eval computation library is provided by `ancora-evallib`.
See the [eval library reference](../../../crates/ancora-evallib/README.md) for
the full API surface.
