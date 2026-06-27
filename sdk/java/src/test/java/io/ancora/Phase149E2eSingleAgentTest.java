package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.HashSet;
import java.util.List;
import java.util.Set;

import static org.junit.jupiter.api.Assertions.*;

class Phase149E2eSingleAgentTest {

    @Test
    void singleAgent_emitsStartedAndCompleted() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            List<RunEvent> events = a.run(new AgentSpec("llama3", null, null, null, null)).collectAll();
            assertInstanceOf(RunEvent.Started.class, events.get(0));
            assertInstanceOf(RunEvent.Completed.class, events.get(events.size() - 1));
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void singleAgent_runId_nonEmpty() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            assertFalse(h.runId().isEmpty());
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void singleAgent_allEventsShareRunId() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
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
    void singleAgent_withInstructions_completes() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            AgentSpec spec = new AgentSpec("llama3", "You are a helpful assistant.", null, null, null);
            List<RunEvent> events = a.run(spec).collectAll();
            assertInstanceOf(RunEvent.Completed.class, events.get(events.size() - 1));
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void singleAgent_fiveRuns_allComplete() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
            for (int i = 0; i < 5; i++) {
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
    void singleAgent_fiveRuns_uniqueIds() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
            Set<String> ids = new HashSet<>();
            for (int i = 0; i < 5; i++) ids.add(a.run(spec).runId());
            assertEquals(5, ids.size());
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void singleAgent_streamViaIterable_matchesCollect() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
            RunHandle h1 = a.run(spec);
            List<RunEvent> collected = h1.collectAll();

            RunHandle h2 = a.run(spec);
            int streamCount = 0;
            for (RunEvent ev : h2.events()) streamCount++;

            assertEquals(collected.size(), streamCount);
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void runEvent_started_hasSpec() {
        RunEvent.Started ev = new RunEvent.Started("r1", "{\"model\":\"llama3\"}");
        assertEquals("{\"model\":\"llama3\"}", ev.spec());
    }

    @Test
    void singleAgent_withMaxTokens_completes() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            AgentSpec spec = new AgentSpec("llama3", null, null, 100, null);
            List<RunEvent> events = a.run(spec).collectAll();
            assertFalse(events.isEmpty());
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void singleAgent_withTemperature_completes() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            AgentSpec spec = new AgentSpec("llama3", null, null, null, 0.7);
            List<RunEvent> events = a.run(spec).collectAll();
            assertFalse(events.isEmpty());
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
