# Consuming Native Artifacts

The CI workflow `native-artifacts.yml` produces two artifact categories for each supported platform:

- `ancora-ffi-<platform>` - the compiled cdylib (`.so`, `.dylib`, or `.dll`)
- `ancora-header-<platform>` - the generated `ancora.h` C header

## Supported targets

| Artifact suffix | Target triple |
|---|---|
| `linux-x86_64` | `x86_64-unknown-linux-gnu` |
| `linux-aarch64` | `aarch64-unknown-linux-gnu` |
| `macos-x86_64` | `x86_64-apple-darwin` |
| `macos-arm64` | `aarch64-apple-darwin` |
| `windows-x86_64` | `x86_64-pc-windows-msvc` |

## Linking in C

Download `ancora-ffi-<platform>` and `ancora-header-<platform>` from CI artifacts.

```c
#include "ancora.h"

int main(void) {
    AncorRuntime *rt = NULL;
    ancora_runtime_new(&rt);
    // ...
    ancora_free_runtime(rt);
    return 0;
}
```

Compile with:

```sh
cc myapp.c -L/path/to/artifact -lancora_ffi -o myapp
```

## Linking in Swift (on Apple platforms)

Place `libancora_ffi.dylib` and `ancora.h` in a directory, then create a bridging header:

```swift
import Foundation

// module.modulemap
module AncorFFI {
    header "ancora.h"
    link "ancora_ffi"
    export *
}
```

Pass `-I/path/to/header -L/path/to/lib` to the Swift compiler.

## Linking in Kotlin (via JNI/JNA)

Place `libancora_ffi.so` (Linux) or `ancora_ffi.dll` (Windows) on the Java library path:

```kotlin
System.loadLibrary("ancora_ffi")
```

Declare the FFI interface with JNA or a JNI wrapper generated from the C header.

## Artifact naming convention

Artifacts are named `ancora-ffi-<platform>-<version>` where `<version>` corresponds to the crate version from `Cargo.toml`. The header artifact is always named `ancora-header-<platform>-<version>`.

Platform values match the `artifact_suffix` column in the target matrix table above.
