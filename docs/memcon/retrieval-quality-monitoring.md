# Retrieval Quality Monitoring

After consolidation, run `RetrievalChecker::check` with the keywords that
must survive to verify that the key facts are still accessible.

```rust
let ok = RetrievalChecker::check(&retained_contents, &["rust", "offline"]);
if !ok {
    let missing = RetrievalChecker::missing_keywords(&retained_contents, &["rust", "offline"]);
    // alert on missing keywords
}
```

## What to Monitor

- Required domain terms that agents rely on for routing decisions.
- User preferences that were stated early in the conversation.
- Any fact referenced more than once across recent turns.

## Integration

Run retrieval quality check as part of `ConsolidationJob::run` and journal
failures alongside the Forgot events so they are replayable.
