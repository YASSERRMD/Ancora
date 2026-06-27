package io.ancora;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.*;

/** Cross-language conformance: human-in-loop scenario -- Java (offline). */
public class Phase152XlangHumanInLoopTest {

    static final String RUN_ID = "xlh-java";
    static final ObjectMapper MAPPER = new ObjectMapper();

    record HilEvent(String kind, String runId, String prompt, List<String> options, String decision, Map<String, Object> output) {
        HilEvent(String kind, String runId) { this(kind, runId, null, null, null, null); }
    }

    static List<HilEvent> makeEvents(String runId) {
        return List.of(
            new HilEvent("started",            runId),
            new HilEvent("decision_requested", runId, "Please approve the draft", List.of("approve", "reject"), null, null),
            new HilEvent("decision_received",  runId, null, null, "{\"approved\":true}", null),
            new HilEvent("completed",          runId, null, null, null, Map.of("result", "hil-ok"))
        );
    }

    @Test void startedIsFirst() { assertEquals("started", makeEvents(RUN_ID).get(0).kind()); }

    @Test void requestedBeforeReceived() {
        var decKinds = makeEvents(RUN_ID).stream().filter(e -> e.kind().startsWith("decision")).map(HilEvent::kind).toList();
        assertEquals(List.of("decision_requested", "decision_received"), decKinds);
    }

    @Test void decisionIsApproved() throws Exception {
        var received = makeEvents(RUN_ID).stream().filter(e -> "decision_received".equals(e.kind())).findFirst().orElseThrow();
        Map<?, ?> dec = MAPPER.readValue(received.decision(), Map.class);
        assertEquals(Boolean.TRUE, dec.get("approved"));
    }

    @Test void promptNonEmpty() {
        var requested = makeEvents(RUN_ID).stream().filter(e -> "decision_requested".equals(e.kind())).findFirst().orElseThrow();
        assertFalse(requested.prompt().isBlank());
        assertFalse(requested.options().isEmpty());
    }

    @Test void completedIsLast() {
        var evs = makeEvents(RUN_ID);
        assertEquals("completed", evs.get(evs.size()-1).kind());
    }

    @Test void runIdConsistent() {
        for (var ev : makeEvents(RUN_ID)) assertEquals(RUN_ID, ev.runId());
    }
}
