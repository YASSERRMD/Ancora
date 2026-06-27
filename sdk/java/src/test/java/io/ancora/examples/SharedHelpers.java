package io.ancora.examples;

import java.time.Instant;
import java.util.*;

/** Shared helpers used across the Java example tests. */
public final class SharedHelpers {

    private SharedHelpers() {}

    // ------------------------------------------------------------------
    // RunJournal
    // ------------------------------------------------------------------

    /** In-memory event store that mimics a durable restart journal. */
    public static final class RunJournal {
        private final Map<String, List<String>> store = new LinkedHashMap<>();

        /** Register a run ID. Idempotent: calling twice keeps count at 1. */
        public void recordRun(String runId) {
            store.putIfAbsent(runId, new ArrayList<>());
        }

        public void appendEvent(String runId, String eventJson) {
            store.computeIfAbsent(runId, k -> new ArrayList<>()).add(eventJson);
        }

        public List<String> eventsForRun(String runId) {
            return Collections.unmodifiableList(store.getOrDefault(runId, List.of()));
        }

        public int runCount() {
            return store.size();
        }
    }

    // ------------------------------------------------------------------
    // Span
    // ------------------------------------------------------------------

    /** Lightweight in-process span mirroring what an OTEL exporter would consume. */
    public static final class Span {
        private final String name;
        private final Instant start = Instant.now();
        private final Map<String, Object> attributes = new LinkedHashMap<>();
        private long durationMs = -1;

        public Span(String name) {
            this.name = name;
        }

        public String name() {
            return name;
        }

        public void setAttribute(String key, Object value) {
            attributes.put(key, value);
        }

        public Object getAttribute(String key) {
            return attributes.get(key);
        }

        public Map<String, Object> attributes() {
            return Collections.unmodifiableMap(attributes);
        }

        /** Marks the span as ended and returns elapsed milliseconds. */
        public long endMs() {
            durationMs = Instant.now().toEpochMilli() - start.toEpochMilli();
            return durationMs;
        }

        public long durationMs() {
            return durationMs;
        }
    }

    // ------------------------------------------------------------------
    // TokenEstimator
    // ------------------------------------------------------------------

    /** Rough 4-chars-per-token estimator for offline cost tracking. */
    public static final class TokenEstimator {
        private TokenEstimator() {}

        public static int estimateTokens(String text) {
            if (text == null || text.isEmpty()) return 1;
            return (int) Math.max(1, Math.ceil(text.length() / 4.0));
        }
    }
}
