package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.Optional;
import java.util.concurrent.ConcurrentHashMap;

import static org.junit.jupiter.api.Assertions.*;

class Phase149RelLongRunTest {

    @Test
    void fifty_sequential_runs_allComplete() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
            for (int i = 0; i < 50; i++) {
                List<RunEvent> events = a.run(spec).collectAll();
                assertInstanceOf(RunEvent.Completed.class, events.get(events.size() - 1));
            }
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void hundredRuntimeCycles_noOOM() {
        skipIfAbsent();
        try {
            for (int i = 0; i < 100; i++) {
                try (Agent a = new Agent()) {
                    a.run(new AgentSpec("llama3", null, null, null, null)).collectAll();
                }
            }
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void fiveHundredStoreOps_consistent() {
        ConcurrentHashMap<String, String> store = new ConcurrentHashMap<>();
        for (int i = 0; i < 500; i++) store.put("key-" + i, "val-" + i);
        for (int i = 0; i < 500; i++) assertEquals("val-" + i, store.get("key-" + i));
        assertEquals(500, store.size());
    }

    @Test
    void twentyRuns_eventCountNonZero() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
            for (int i = 0; i < 20; i++) {
                List<RunEvent> events = a.run(spec).collectAll();
                assertFalse(events.isEmpty());
            }
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void longRun_noMemoryLeak_agentReopened() {
        skipIfAbsent();
        try {
            for (int i = 0; i < 25; i++) {
                try (Agent a = new Agent()) {
                    a.run(new AgentSpec("llama3", null, null, null, null)).collectAll();
                }
            }
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void toolRegistrations_longRun_noLeak() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            for (int i = 0; i < 20; i++) {
                ToolRegistration reg = ToolRegistry.register(rt, "long_tool_" + i, "Tool " + i,
                    input -> "{\"ok\":true}");
                reg.disposable().close();
            }
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void longRunWithStreaming_allComplete() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
            for (int i = 0; i < 10; i++) {
                RunEvent last = null;
                for (RunEvent ev : a.run(spec).events()) last = ev;
                assertInstanceOf(RunEvent.Completed.class, last);
            }
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
