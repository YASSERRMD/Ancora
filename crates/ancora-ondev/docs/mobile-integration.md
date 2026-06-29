# Mobile Integration

This guide explains how to embed `ancora-ondev` into Android and iOS applications.

## Android (JNI)

### Prerequisites

- Android NDK r25 or later
- `aarch64-linux-android` Rust target installed
- `cargo-ndk` (optional but recommended)

### Add the NDK target

```bash
rustup target add aarch64-linux-android
```

### Build the shared library

```bash
cargo ndk -t arm64-v8a build \
    --profile ondev \
    --no-default-features \
    --features android_jni
```

This produces `libancora_ondev.so` that you copy into
`app/src/main/jniLibs/arm64-v8a/`.

### JNI Entry Points

The crate uses the convention
`Java_<package>_<class>_<method>` for exported symbols.  With the default
`JniBridge` configuration the prefix is:

```
Java_com_ancora_agent_AgentRuntime_
```

Example Kotlin binding:

```kotlin
class AgentRuntime {
    external fun init(modelPath: String): Long
    external fun run(handle: Long, prompt: String): String
    external fun close(handle: Long)

    companion object {
        init { System.loadLibrary("ancora_ondev") }
    }
}
```

### ProGuard / R8

Add to your `proguard-rules.pro`:

```
-keep class com.ancora.agent.AgentRuntime { *; }
```

---

## iOS (C-ABI)

### Prerequisites

- Xcode 15 or later
- `aarch64-apple-ios` Rust target installed
- `cbindgen` for header generation

### Add the iOS target

```bash
rustup target add aarch64-apple-ios
```

### Build the static library

```bash
cargo build --target aarch64-apple-ios \
    --profile ondev \
    --no-default-features \
    --features ios_cabi
```

This produces `libancora_ondev.a`.

### Generate the C header

```bash
cbindgen --crate ancora-ondev \
         --output include/ancora_ondev.h \
         --lang c
```

### Xcode Integration

1. Drag `libancora_ondev.a` into your Xcode project target.
2. Add the `include/` folder to **Header Search Paths**.
3. Add a bridging header that imports `ancora_ondev.h`.
4. Ensure **Other Linker Flags** contains `-lancora_ondev`.

Example Swift usage:

```swift
import Foundation

func runAgent(prompt: String) -> String {
    // ancora_ondev_infer is the C-ABI exported function
    guard let cStr = ancora_ondev_infer(prompt) else { return "" }
    return String(cString: cStr)
}
```

---

## Shared Considerations

- **Model files** must be bundled with the app (APK assets or iOS bundle).
- **Journal path** should be set to the app's private data directory
  (`getFilesDir()` on Android, `FileManager.default.urls(.documentDirectory)`
  on iOS).
- **Memory store** is in-process by default; set an explicit path for durable
  persistence across process restarts.
