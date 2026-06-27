package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.HashSet;
import java.util.List;
import java.util.Set;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;
import java.util.concurrent.TimeUnit;

import static org.junit.jupiter.api.Assertions.*;

class Phase149E2eVerifierTest {

    @Test
    void drafter_completesFirst_thenVerifier() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            Agent drafter  = new Agent(rt);
            Agent verifier = new Agent(rt);
            List<RunEvent> de = drafter.run(new AgentSpec("llama3", "You are a drafter.", null, null, null)).collectAll();
            List<RunEvent> ve = verifier.run(new AgentSpec("llama3", "You are a verifier.", null, null, null)).collectAll();
            assertInstanceOf(RunEvent.Completed.class, de.get(de.size() - 1));
            assertInstanceOf(RunEvent.Completed.class, ve.get(ve.size() - 1));
            drafter.close();
            verifier.close();
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void drafter_and_verifier_haveDistinctRunIds() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            Agent d = new Agent(rt);
            Agent v = new Agent(rt);
            AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
            String dId = d.run(spec).runId();
            String vId = v.run(spec).runId();
            assertNotEquals(dId, vId);
            d.close();
            v.close();
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void parallel_drafter_and_verifier_bothComplete() throws Exception {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            ExecutorService pool = Executors.newFixedThreadPool(2);
            Future<List<RunEvent>> dFuture = pool.submit(() -> {
                Agent d = new Agent(rt);
                List<RunEvent> events = d.run(new AgentSpec("llama3", "draft", null, null, null)).collectAll();
                d.close();
                return events;
            });
            Future<List<RunEvent>> vFuture = pool.submit(() -> {
                Agent v = new Agent(rt);
                List<RunEvent> events = v.run(new AgentSpec("llama3", "verify", null, null, null)).collectAll();
                v.close();
                return events;
            });
            pool.shutdown();
            pool.awaitTermination(30, TimeUnit.SECONDS);
            List<RunEvent> de = dFuture.get();
            List<RunEvent> ve = vFuture.get();
            assertInstanceOf(RunEvent.Completed.class, de.get(de.size() - 1));
            assertInstanceOf(RunEvent.Completed.class, ve.get(ve.size() - 1));
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void threePasses_verifierAccumulatesIds() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
            Set<String> ids = new HashSet<>();
            for (int i = 0; i < 3; i++) ids.add(a.run(spec).runId());
            assertEquals(3, ids.size());
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void verifierEvents_haveMatchingId() {
        skipIfAbsent();
        try (Agent v = new Agent()) {
            RunHandle h = v.run(new AgentSpec("llama3", "verify quality", null, null, null));
            String id = h.runId();
            for (RunEvent ev : h.events()) {
                assertEquals(id, getRunId(ev));
            }
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void agent_afterVerifierDone_isReusable() {
        skipIfAbsent();
        try (Agent v = new Agent()) {
            AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
            v.run(spec).collectAll();
            List<RunEvent> second = v.run(spec).collectAll();
            assertFalse(second.isEmpty());
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void verifier_completedEvent_runIdMatchesStart() {
        skipIfAbsent();
        try (Agent v = new Agent()) {
            RunHandle h = v.run(new AgentSpec("llama3", null, null, null, null));
            List<RunEvent> events = h.collectAll();
            String startId = ((RunEvent.Started) events.get(0)).runId();
            String endId = ((RunEvent.Completed) events.get(events.size() - 1)).runId();
            assertEquals(startId, endId);
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void fiveVerifierPipelines_allComplete() {
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

    @Test
    void runEvent_types_areNotNull() {
        RunEvent.Started  s  = new RunEvent.Started("r1", "{}");
        RunEvent.Completed c = new RunEvent.Completed("r1");
        assertNotNull(s.runId());
        assertNotNull(c.runId());
    }

    @Test
    void draftAndVerify_separateRuntimes_noInterference() {
        skipIfAbsent();
        try {
            List<RunEvent> de, ve;
            try (Agent d = new Agent()) {
                de = d.run(new AgentSpec("llama3", "draft", null, null, null)).collectAll();
            }
            try (Agent v = new Agent()) {
                ve = v.run(new AgentSpec("llama3", "verify", null, null, null)).collectAll();
            }
            assertInstanceOf(RunEvent.Completed.class, de.get(de.size() - 1));
            assertInstanceOf(RunEvent.Completed.class, ve.get(ve.size() - 1));
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    private static String getRunId(RunEvent e) {
        return switch (e) {
            case RunEvent.Started s -> s.runId();
            case RunEvent.Token t -> t.runId();
            case RunEvent.Completed c -> c.runId();
            case RunEvent.Resumed r -> r.runId();
            case RunEvent.ToolCall tc -> tc.runId();
        };
    }

    private static void skipIfAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "ancora_ffi not present");
    }
}
