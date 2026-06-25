# Ancora Quickstarts

Pick the binding for your language. Each quickstart walks through installing
the package, writing a minimal single-agent graph, running it against a local
inference endpoint, and verifying the journal output.

## Quickstart index

| Language | Package | Quickstart |
|----------|---------|------------|
| Rust | `ancora-core` (native) | [Rust quickstart](#rust) |
| Go | `sdk/go` | [Go quickstart](#go) |
| Python | `sdk/python` | [Python quickstart](#python) |
| TypeScript | `sdk/ts` | [TypeScript quickstart](#typescript) |
| .NET | `sdk/dotnet` | [.NET quickstart](#net) |
| Java | `sdk/java` | [Java quickstart](#java) |

All examples assume a local [Ollama](https://ollama.com) instance serving a
compatible model at `http://127.0.0.1:11434`. Substitute any OpenAI-compatible
endpoint via the `ANCORA_MODEL_URL` environment variable.

---

## Rust

```toml
# Cargo.toml
[dependencies]
ancora-core = { path = "../../crates/ancora-core" }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

```rust
use ancora_core::{
    graph::{Edge, Graph, Node, NodeKind, NodeSpec},
    runner::run_graph,
};

#[tokio::main]
async fn main() {
    let graph = Graph {
        id: "hello".into(),
        entry_node: "greeter".into(),
        nodes: vec![Node {
            id: "greeter".into(),
            kind: NodeKind::Agent,
            model_id: None,
            spec: NodeSpec::Agent(Default::default()),
        }],
        edges: vec![],
    };
    let journal = run_graph(&graph, "Say hello").await.unwrap();
    println!("{} events", journal.len());
}
```

Run:
```bash
cargo run --example hello
```

---

## Go

```bash
go get github.com/YASSERRMD/Ancora/sdk/go
```

```go
package main

import (
    "fmt"
    ancora "github.com/YASSERRMD/Ancora/sdk/go"
)

func main() {
    rt, err := ancora.NewRuntime()
    if err != nil {
        panic(err)
    }
    defer rt.Close()

    result, err := rt.Run("say hello")
    if err != nil {
        panic(err)
    }
    fmt.Println(result)
}
```

Run:
```bash
go run ./examples/hello
```

---

## Python

```bash
pip install ancora-sdk
```

```python
from ancora import Runtime

rt = Runtime()
result = rt.run("say hello")
print(result)
```

Run:
```bash
python examples/hello.py
```

---

## TypeScript

```bash
npm install @ancora/sdk
```

```typescript
import { Runtime } from "@ancora/sdk";

const rt = new Runtime();
const result = await rt.run("say hello");
console.log(result);
```

Run:
```bash
npx ts-node examples/hello.ts
```

---

## .NET

```bash
dotnet add package Ancora.Sdk
```

```csharp
using Ancora;

var rt = new Runtime();
var result = await rt.RunAsync("say hello");
Console.WriteLine(result);
```

Run:
```bash
dotnet run --project examples/Hello
```

---

## Java

```xml
<!-- pom.xml -->
<dependency>
    <groupId>com.ancora</groupId>
    <artifactId>ancora-sdk</artifactId>
    <version>0.1.0</version>
</dependency>
```

```java
import com.ancora.Runtime;

public class Hello {
    public static void main(String[] args) throws Exception {
        try (var rt = new Runtime()) {
            System.out.println(rt.run("say hello"));
        }
    }
}
```

Run:
```bash
mvn exec:java -Dexec.mainClass="Hello"
```

---

## Next steps

- [Architecture overview](../spec/architecture.md) -- how the engine and
  journal work
- [Orchestration guide](../guides/orchestration.md) -- multi-node graphs,
  branching, and parallel execution
- [Durability guide](../guides/durability.md) -- crash recovery and
  replay guarantees
- [Observability guide](../guides/observability.md) -- OpenTelemetry spans
  and cost tracking
- [Governance guide](../guides/governance.md) -- policy, air-gapped mode,
  PII redaction
