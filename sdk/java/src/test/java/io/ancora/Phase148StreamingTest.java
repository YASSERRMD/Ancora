package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class Phase148StreamingTest {

    @Test
    void events_returnsIterable() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            Iterable<RunEvent> it = h.events();
            assertNotNull(it);
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void forEachLoop_drains_events() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            List<RunEvent> collected = new ArrayList<>();
            for (RunEvent ev : h.events()) {
                collected.add(ev);
            }
            assertFalse(collected.isEmpty());
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void forEachLoop_firstEvent_isStarted() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            RunEvent first = null;
            for (RunEvent ev : h.events()) {
                if (first == null) first = ev;
            }
            assertInstanceOf(RunEvent.Started.class, first);
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void forEachLoop_lastEvent_isCompleted() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            RunEvent last = null;
            for (RunEvent ev : h.events()) {
                last = ev;
            }
            assertInstanceOf(RunEvent.Completed.class, last);
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void events_and_collectAll_sameCount() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
            RunHandle h1 = a.run(spec);
            List<RunEvent> via_collect = h1.collectAll();

            RunHandle h2 = a.run(spec);
            List<RunEvent> via_iter = new ArrayList<>();
            for (RunEvent ev : h2.events()) via_iter.add(ev);

            assertEquals(via_collect.size(), via_iter.size());
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void runHandle_events_method_exists() throws Exception {
        assertNotNull(RunHandle.class.getDeclaredMethod("events"));
    }

    @Test
    void tokenEvent_hasText() {
        RunEvent.Token ev = new RunEvent.Token("r1", "chunk", "llama3");
        assertNotNull(ev.text());
        assertFalse(ev.text().isEmpty());
    }

    @Test
    void tokenEvent_hasModel() {
        RunEvent.Token ev = new RunEvent.Token("r1", "chunk", "llama3");
        assertEquals("llama3", ev.model());
    }

    @Test
    void iteratorExhausted_secondIteration_isEmpty() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            for (RunEvent ev : h.events()) { /* drain */ }
            int count = 0;
            for (RunEvent ev : h.events()) count++;
            assertEquals(0, count);
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void streamedRunIds_allMatch() {
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
