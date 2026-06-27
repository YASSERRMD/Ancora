package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;
import java.util.concurrent.TimeUnit;

import static org.junit.jupiter.api.Assertions.*;

class Phase148ConcurrentRunsTest {

    @Test
    void fifty_sequential_runs_haveUniqueIds() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
            Set<String> ids = new HashSet<>();
            for (int i = 0; i < 50; i++) ids.add(a.run(spec).runId());
            assertEquals(50, ids.size());
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void one_hundred_rt_cycles_noCrash() {
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
    void runHandle_isNotAutoCloseable() throws Exception {
        var method = RunHandle.class.getDeclaredMethods();
        boolean hasClose = false;
        for (var m : method) {
            if (m.getName().equals("close") && m.getParameterCount() == 0) {
                hasClose = true;
                break;
            }
        }
        assertFalse(hasClose, "RunHandle should NOT implement AutoCloseable");
    }

    @Test
    void agent_implementsAutoCloseable() {
        assertTrue(AutoCloseable.class.isAssignableFrom(Agent.class));
    }

    @Test
    void twoAgents_sameRuntime_doNotInterfere() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            Agent a1 = new Agent(rt);
            Agent a2 = new Agent(rt);
            AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
            RunHandle h1 = a1.run(spec);
            RunHandle h2 = a2.run(spec);
            assertNotEquals(h1.runId(), h2.runId());
            h1.collectAll();
            h2.collectAll();
            a1.close();
            a2.close();
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void ten_parallel_runs_allComplete() throws InterruptedException {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            ExecutorService pool = Executors.newFixedThreadPool(4);
            List<Future<Boolean>> futures = new ArrayList<>();
            for (int i = 0; i < 10; i++) {
                futures.add(pool.submit(() -> {
                    Agent a = new Agent(rt);
                    List<RunEvent> events = a.run(new AgentSpec("llama3", null, null, null, null)).collectAll();
                    a.close();
                    return events.get(events.size() - 1) instanceof RunEvent.Completed;
                }));
            }
            pool.shutdown();
            assertTrue(pool.awaitTermination(30, TimeUnit.SECONDS));
            for (var f : futures) {
                try { assertTrue(f.get()); } catch (Exception e) { fail(e.toString()); }
            }
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void runIds_acrossParallelRuns_areUnique() throws InterruptedException {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            Set<String> ids = java.util.Collections.synchronizedSet(new HashSet<>());
            ExecutorService pool = Executors.newFixedThreadPool(4);
            for (int i = 0; i < 10; i++) {
                pool.submit(() -> {
                    Agent a = new Agent(rt);
                    ids.add(a.run(new AgentSpec("llama3", null, null, null, null)).runId());
                    a.close();
                });
            }
            pool.shutdown();
            pool.awaitTermination(30, TimeUnit.SECONDS);
            assertEquals(10, ids.size());
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void singleAgent_multiRun_collectAll_allNonEmpty() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
            for (int i = 0; i < 5; i++) {
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
    void agentClosed_thenReopened_works() {
        skipIfAbsent();
        try {
            for (int i = 0; i < 3; i++) {
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
    void runtime_isAutoCloseable() {
        assertTrue(AutoCloseable.class.isAssignableFrom(Runtime.class));
    }

    @Test
    void fiveAgents_sequential_sameRuntime() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
            for (int i = 0; i < 5; i++) {
                Agent a = new Agent(rt);
                List<RunEvent> events = a.run(spec).collectAll();
                assertInstanceOf(RunEvent.Completed.class, events.get(events.size() - 1));
                a.close();
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
