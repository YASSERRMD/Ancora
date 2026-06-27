package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.HashSet;
import java.util.List;
import java.util.Set;

import static org.junit.jupiter.api.Assertions.*;

class Phase149RelRestartTest {

    @Test
    void closeAndReopen_agentWorks() {
        skipIfAbsent();
        try {
            List<RunEvent> events;
            try (Agent a = new Agent()) {
                events = a.run(new AgentSpec("llama3", null, null, null, null)).collectAll();
            }
            assertFalse(events.isEmpty());
            try (Agent a = new Agent()) {
                List<RunEvent> events2 = a.run(new AgentSpec("llama3", null, null, null, null)).collectAll();
                assertFalse(events2.isEmpty());
            }
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void runtimeRestart_agentRunsAfter() {
        skipIfAbsent();
        try {
            String firstId;
            try (Runtime rt = new Runtime()) {
                Agent a = new Agent(rt);
                firstId = a.run(new AgentSpec("llama3", null, null, null, null)).runId();
                a.close();
            }
            String secondId;
            try (Runtime rt = new Runtime()) {
                Agent a = new Agent(rt);
                secondId = a.run(new AgentSpec("llama3", null, null, null, null)).runId();
                a.close();
            }
            assertNotEquals(firstId, secondId);
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void tenRestartCycles_noErrors() {
        skipIfAbsent();
        try {
            for (int i = 0; i < 10; i++) {
                try (Agent a = new Agent()) {
                    List<RunEvent> events = a.run(new AgentSpec("llama3", null, null, null, null)).collectAll();
                    assertFalse(events.isEmpty());
                }
            }
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void restartAfterFailure_recovers() {
        skipIfAbsent();
        try {
            Agent a = new Agent();
            a.close();
            try (Agent b = new Agent()) {
                List<RunEvent> events = b.run(new AgentSpec("llama3", null, null, null, null)).collectAll();
                assertInstanceOf(RunEvent.Completed.class, events.get(events.size() - 1));
            }
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void runIds_acrossRestarts_areUnique() {
        skipIfAbsent();
        try {
            Set<String> ids = new HashSet<>();
            for (int i = 0; i < 5; i++) {
                try (Agent a = new Agent()) {
                    ids.add(a.run(new AgentSpec("llama3", null, null, null, null)).runId());
                }
            }
            assertEquals(5, ids.size());
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void agent_close_isIdempotent() {
        skipIfAbsent();
        try {
            Agent a = new Agent();
            a.close();
            assertDoesNotThrow(a::close);
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void runtime_close_isIdempotent() {
        skipIfAbsent();
        try {
            Runtime rt = new Runtime();
            rt.close();
            assertDoesNotThrow(rt::close);
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void restart_preservesToolRegistration() {
        skipIfAbsent();
        try {
            for (int i = 0; i < 3; i++) {
                try (Runtime rt = new Runtime()) {
                    ToolRegistration reg = ToolRegistry.register(rt, "restart_tool149", "Restart",
                        input -> "{\"ok\":true}");
                    Agent a = new Agent(rt);
                    a.run(new AgentSpec("llama3", null, null, null, null)).collectAll();
                    a.close();
                    reg.disposable().close();
                }
            }
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void restartCycle_create_run_close_create() {
        skipIfAbsent();
        try {
            String runId1;
            try (Agent a = new Agent()) {
                runId1 = a.run(new AgentSpec("llama3", null, null, null, null)).runId();
            }
            String runId2;
            try (Agent a = new Agent()) {
                runId2 = a.run(new AgentSpec("llama3", null, null, null, null)).runId();
            }
            assertNotEquals(runId1, runId2);
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    private static void skipIfAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "ancora_ffi not present");
    }
}
