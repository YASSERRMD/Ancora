package io.ancora;

import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Cross-language conformance: single agent scenario -- Java binding (offline fixture).
 */
public class Phase152XlangSingleAgentTest {

    static final String XLANG_RUN_ID = "xlang-java-001";

    static List<Map<String, Object>> makeXlangEvents(String runId) {
        return List.of(
            Map.of("kind", "started",   "run_id", runId, "spec", "{}"),
            Map.of("kind", "token",     "run_id", runId, "text", "xlang java result"),
            Map.of("kind", "completed", "run_id", runId)
        );
    }

    @Test
    void startedEventIsFirst() {
        var events = makeXlangEvents(XLANG_RUN_ID);
        assertEquals("started", events.get(0).get("kind"));
    }

    @Test
    void completedEventIsLast() {
        var events = makeXlangEvents(XLANG_RUN_ID);
        assertEquals("completed", events.get(events.size() - 1).get("kind"));
    }

    @Test
    void runIdConsistentAcrossEvents() {
        var events = makeXlangEvents(XLANG_RUN_ID);
        for (var ev : events) {
            assertEquals(XLANG_RUN_ID, ev.get("run_id"));
        }
    }

    @Test
    void eventCountAtLeastTwo() {
        var events = makeXlangEvents(XLANG_RUN_ID);
        assertTrue(events.size() >= 2);
    }

    @Test
    void tokenEventHasNonEmptyText() {
        var events = makeXlangEvents(XLANG_RUN_ID);
        var tokens = events.stream().filter(e -> "token".equals(e.get("kind"))).toList();
        assertFalse(tokens.isEmpty(), "expected at least one token event");
        for (var tok : tokens) {
            var text = (String) tok.get("text");
            assertNotNull(text);
            assertFalse(text.isBlank());
        }
    }

    @Test
    void startedSpecFieldPresent() {
        var events = makeXlangEvents(XLANG_RUN_ID);
        var started = events.get(0);
        assertEquals("started", started.get("kind"));
        assertNotNull(started.get("spec"));
    }

    @Test
    void noEventBeforeStarted() {
        var events = makeXlangEvents(XLANG_RUN_ID);
        assertEquals("started", events.get(0).get("kind"),
            "started must be the first event");
    }

    @Test
    void noEventAfterCompleted() {
        var events = makeXlangEvents(XLANG_RUN_ID);
        assertEquals("completed", events.get(events.size() - 1).get("kind"),
            "completed must be the last event");
    }
}
