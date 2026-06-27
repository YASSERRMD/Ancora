package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.HashSet;
import java.util.List;
import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

import static org.junit.jupiter.api.Assertions.*;

class Phase148MultiAgentVerifierTest {

    @Test
    void twoAgents_sharingRuntime_haveDistinctHandles() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            Agent d = new Agent(rt);
            Agent v = new Agent(rt);
            RunHandle dh = d.run(new AgentSpec("llama3", "draft", null, null, null));
            RunHandle vh = v.run(new AgentSpec("llama3", "verify", null, null, null));
            assertNotEquals(dh.runId(), vh.runId());
            d.close();
            v.close();
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void drafter_emitsCompleted() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            List<RunEvent> events = a.run(new AgentSpec("llama3", null, null, null, null)).collectAll();
            assertInstanceOf(RunEvent.Completed.class, events.get(events.size() - 1));
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void verifier_emitsCompleted() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            Agent d = new Agent(rt);
            Agent v = new Agent(rt);
            d.run(new AgentSpec("llama3", null, null, null, null)).collectAll();
            List<RunEvent> ve = v.run(new AgentSpec("llama3", null, null, null, null)).collectAll();
            assertInstanceOf(RunEvent.Completed.class, ve.get(ve.size() - 1));
            d.close();
            v.close();
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void fiveSequentialRuns_haveUniqueIds() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            Set<String> ids = new HashSet<>();
            AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
            for (int i = 0; i < 5; i++) ids.add(a.run(spec).runId());
            assertEquals(5, ids.size());
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void drafterEvents_haveMatchingRunId() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            String id = h.runId();
            List<RunEvent> events = h.collectAll();
            for (RunEvent ev : events) assertEquals(id, getRunId(ev));
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void agent_nullRuntime_throwsNPE() {
        assertThrows(NullPointerException.class, () -> new Agent(null));
    }

    @Test
    void threeStagePipeline_allComplete() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
            for (int i = 0; i < 3; i++) {
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
    void startedEvent_and_completedEvent_kinds_differ() {
        RunEvent.Started s = new RunEvent.Started("r1", "{}");
        RunEvent.Completed c = new RunEvent.Completed("r1");
        assertNotEquals(s.getClass(), c.getClass());
    }

    @Test
    void verifierAndDrafter_eventsAreIndependent() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            Agent d = new Agent(rt);
            Agent v = new Agent(rt);
            RunHandle dh = d.run(new AgentSpec("llama3", null, null, null, null));
            RunHandle vh = v.run(new AgentSpec("llama3", null, null, null, null));
            List<RunEvent> de = dh.collectAll();
            List<RunEvent> ve = vh.collectAll();
            de.forEach(e -> assertEquals(dh.runId(), getRunId(e)));
            ve.forEach(e -> assertEquals(vh.runId(), getRunId(e)));
            d.close();
            v.close();
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void repeatVerifierCycle_thricePasses() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
            for (int i = 0; i < 3; i++) {
                List<RunEvent> events = a.run(spec).collectAll();
                assertInstanceOf(RunEvent.Completed.class, events.get(events.size() - 1));
            }
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
