# Cross-Language App Guide

## Overview

ancora-appspar ships one sample agent app per language. All apps follow the
same interaction model: one user turn, one assistant reply, and a trace ID.

## Common Interaction Model

1. Construct the app with a name and any language-specific constructor args.
2. Call `run(user_input)` - returns a trace containing messages.
3. The trace always has exactly two messages: user, then assistant.
4. The trace ID encodes the language prefix for easy filtering.

## A2A Composition

The `polyglot` module provides a `PolyglotRouter` that dispatches messages
between language agents. In production, each endpoint is a remote service.
In tests and offline mode, dispatch is in-process and makes no network calls.

## Guardrails

Every app enforces:
- Non-empty input content
- Valid constructor parameters (model name, SDK version, framework, Java/Rust edition)
- Known message roles

Guardrail violations return `Err(...)` rather than panicking.

## Parity Verification

Run `cargo test -p ancora-appspar` to verify:
- All language apps compile and run offline
- All apps implement the required feature set
- All apps emit traces in the canonical two-message shape
- All apps enforce guardrails correctly
- Polyglot A2A routing works for all language pairs
