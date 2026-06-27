package io.ancora;

import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.*;

/** Cross-language conformance: verifier scenario -- Java (offline). */
public class Phase152XlangVerifierTest {

    static final String RUN_ID = "xlv-java";

    record VerifierEvent(String kind, String runId, String activityKey, Map<String, Object> output) {
        VerifierEvent(String kind, String runId) { this(kind, runId, null, null); }
        VerifierEvent(String kind, String runId, String ak) { this(kind, runId, ak, null); }
    }

    static List<VerifierEvent> makeEvents(String runId) {
        return List.of(
            new VerifierEvent("started",   runId),
            new VerifierEvent("activity",  runId, "drafter"),
            new VerifierEvent("activity",  runId, "verifier"),
            new VerifierEvent("completed", runId, null, Map.of("verdict", "approved"))
        );
    }

    @Test void startedIsFirst() { assertEquals("started", makeEvents(RUN_ID).get(0).kind()); }
    @Test void completedIsLast() {
        var evs = makeEvents(RUN_ID);
        assertEquals("completed", evs.get(evs.size()-1).kind());
    }
    @Test void drafterBeforeVerifier() {
        var keys = makeEvents(RUN_ID).stream().filter(e -> "activity".equals(e.kind())).map(VerifierEvent::activityKey).toList();
        assertEquals(List.of("drafter", "verifier"), keys);
    }
    @Test void outputVerdictApproved() {
        var last = makeEvents(RUN_ID).get(makeEvents(RUN_ID).size()-1);
        assertEquals("approved", last.output().get("verdict"));
    }
    @Test void runIdConsistent() {
        for (var ev : makeEvents(RUN_ID)) assertEquals(RUN_ID, ev.runId());
    }
}
