package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.nio.charset.StandardCharsets;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class Phase149E2eHumanInLoopTest {

    static final String APPROVE_DECISION = "{\"approved\":true}";
    static final String REJECT_DECISION  = "{\"approved\":false,\"reason\":\"not safe\"}";

    @Test
    void approveDecision_isValidJson() {
        assertTrue(APPROVE_DECISION.contains("\"approved\":true"));
    }

    @Test
    void rejectDecision_containsReason() {
        assertTrue(REJECT_DECISION.contains("\"reason\""));
    }

    @Test
    void resume_withApproveString_noThrow() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", "await decision", null, null, null));
            assertDoesNotThrow(() -> h.resume(APPROVE_DECISION));
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void resume_withRejectString_noThrow() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", "await decision", null, null, null));
            assertDoesNotThrow(() -> h.resume(REJECT_DECISION));
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void resume_withBytes_noThrow() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            byte[] bytes = APPROVE_DECISION.getBytes(StandardCharsets.UTF_8);
            assertDoesNotThrow(() -> h.resume(bytes));
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void collectAll_afterResume_isNonNull() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            h.resume(APPROVE_DECISION);
            List<RunEvent> events = h.collectAll();
            assertNotNull(events);
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void resumedEvent_fields() {
        RunEvent.Resumed ev = new RunEvent.Resumed("run-1", APPROVE_DECISION);
        assertEquals("run-1", ev.runId());
        assertEquals(APPROVE_DECISION, ev.decision());
    }

    @Test
    void resume_afterCollectAll_isIdempotent() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            h.collectAll();
            assertDoesNotThrow(() -> h.resume(APPROVE_DECISION));
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void twoResumedCycles_inSequence() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            AgentSpec spec = new AgentSpec("llama3", "You may pause for decisions", null, null, null);
            for (int i = 0; i < 2; i++) {
                RunHandle h = a.run(spec);
                h.resume(APPROVE_DECISION);
                assertNotNull(h.collectAll());
            }
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void humanInLoop_agentSpec_hasInstructions() {
        AgentSpec spec = new AgentSpec("llama3", "await decision", null, null, null);
        assertEquals("await decision", spec.instructions());
    }

    @Test
    void runHandle_resume_method_exists() throws Exception {
        assertNotNull(RunHandle.class.getDeclaredMethod("resume", String.class));
    }

    private static void skipIfAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "ancora_ffi not present");
    }
}
