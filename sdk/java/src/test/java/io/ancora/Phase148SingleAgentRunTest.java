package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class Phase148SingleAgentRunTest {

    @Test
    void runHandle_hasRunId() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            assertNotNull(h.runId());
            assertFalse(h.runId().isEmpty());
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void collectAll_returnsNonEmpty() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            List<RunEvent> events = h.collectAll();
            assertFalse(events.isEmpty());
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void firstEvent_isStarted() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            List<RunEvent> events = h.collectAll();
            assertInstanceOf(RunEvent.Started.class, events.get(0));
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void lastEvent_isCompleted() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            List<RunEvent> events = h.collectAll();
            assertInstanceOf(RunEvent.Completed.class, events.get(events.size() - 1));
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void allEvents_shareRunId() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            String id = h.runId();
            List<RunEvent> events = h.collectAll();
            assertTrue(events.stream().allMatch(e -> getRunId(e).equals(id)));
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void secondCollectAll_isEmpty() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            h.collectAll();
            List<RunEvent> second = h.collectAll();
            assertTrue(second.isEmpty());
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void twoRuns_haveDistinctIds() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
            String id1 = a.run(spec).runId();
            String id2 = a.run(spec).runId();
            assertNotEquals(id1, id2);
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void startedEvent_kind() {
        RunEvent.Started ev = new RunEvent.Started("r1", "{}");
        assertEquals("r1", ev.runId());
    }

    @Test
    void completedEvent_kind() {
        RunEvent.Completed ev = new RunEvent.Completed("r1");
        assertEquals("r1", ev.runId());
    }

    @Test
    void runHandle_notNull() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            assertNotNull(h);
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
