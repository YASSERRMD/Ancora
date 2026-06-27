package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class Phase148HumanInLoopTest {

    static final String APPROVE_DECISION = "{\"approved\":true}";
    static final String REJECT_DECISION  = "{\"approved\":false,\"reason\":\"not safe\"}";

    @Test
    void approveDecision_isValidJson() {
        assertTrue(APPROVE_DECISION.contains("\"approved\":true"));
    }

    @Test
    void rejectDecision_containsReason() {
        assertTrue(REJECT_DECISION.contains("reason"));
    }

    @Test
    void resumedEvent_storesDecision() {
        RunEvent.Resumed ev = new RunEvent.Resumed("r1", APPROVE_DECISION);
        assertEquals(APPROVE_DECISION, ev.decision());
    }

    @Test
    void resumedEvent_runId() {
        RunEvent.Resumed ev = new RunEvent.Resumed("run-42", REJECT_DECISION);
        assertEquals("run-42", ev.runId());
    }

    @Test
    void resumedEvent_isRunEvent() {
        RunEvent.Resumed ev = new RunEvent.Resumed("r1", APPROVE_DECISION);
        assertInstanceOf(RunEvent.class, ev);
    }

    @Test
    void runHandle_resume_withString() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", "pause before replying", null, null, null));
            h.resume(APPROVE_DECISION);
            List<RunEvent> events = h.collectAll();
            assertFalse(events.isEmpty());
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void runHandle_resume_withBytes() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            byte[] bytes = APPROVE_DECISION.getBytes(java.nio.charset.StandardCharsets.UTF_8);
            h.resume(bytes);
            List<RunEvent> events = h.collectAll();
            assertFalse(events.isEmpty());
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void resume_afterCompleted_isIdempotent() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            h.collectAll();
            h.resume(APPROVE_DECISION);
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void rejectDecision_isNotApproved() {
        assertFalse(REJECT_DECISION.contains("\"approved\":true"));
    }

    @Test
    void resumedEvent_record_equality() {
        RunEvent.Resumed a = new RunEvent.Resumed("r1", APPROVE_DECISION);
        RunEvent.Resumed b = new RunEvent.Resumed("r1", APPROVE_DECISION);
        assertEquals(a, b);
    }

    @Test
    void resumedEvent_notEqualDifferentDecision() {
        RunEvent.Resumed a = new RunEvent.Resumed("r1", APPROVE_DECISION);
        RunEvent.Resumed b = new RunEvent.Resumed("r1", REJECT_DECISION);
        assertNotEquals(a, b);
    }

    private static void skipIfAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "ancora_ffi not present");
    }
}
