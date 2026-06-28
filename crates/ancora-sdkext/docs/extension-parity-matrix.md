# Extension Parity Matrix

This document records which languages have full interop-kit parity for each
built-in Ancora extension.

## Parity status

| Extension | Rust | Go | Python | TypeScript | .NET | Java |
|-----------|:----:|:--:|:------:|:----------:|:----:|:----:|
| echo | PASS | PASS | PASS | PASS | PASS | PASS |
| add | PASS | - | - | - | - | - |
| kv_store | PASS | - | - | - | - | - |
| go_echo | - | PASS | - | - | - | - |
| go_word_count | - | PASS | - | - | - | - |
| py_echo | - | - | PASS | - | - | - |
| py_sentiment | - | - | PASS | - | - | - |

Legend: PASS = all interop-kit checks pass, `-` = language not applicable for this extension.

## Interop kit check definitions

| Check | Description |
|-------|-------------|
| `meta_non_empty` | `meta()` returns non-empty name, description, and version |
| `health_ok` | `health_check()` returns `Ok(())` |
| `execute_returns_value` | `execute({})` returns without panicking |
| `invalid_arg_no_panic` | `execute({"__invalid__": "garbage"})` does not panic |

## How to add a new language

1. Implement the adapter in a new module (e.g. `src/ruby_interfaces.rs`).
2. Add a registration helper in `registration.rs`.
3. Add `Language::Ruby` to the `Language` enum.
4. Write tests in `src/tests/test_ruby_tool_ext.rs`.
5. Run `InteropKit::run_all` in the test and assert all checks pass.
6. Update this matrix.
