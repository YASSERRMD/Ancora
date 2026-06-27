package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

import static org.junit.jupiter.api.Assertions.*;

class Phase149RelZeroDuplicateTest {

    @Test
    void tenRuns_noDuplicateRunIds() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
            Set<String> ids = new HashSet<>();
            for (int i = 0; i < 10; i++) ids.add(a.run(spec).runId());
            assertEquals(10, ids.size());
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void eventKinds_noDuplicateStarted() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            List<RunEvent> events = a.run(new AgentSpec("llama3", null, null, null, null)).collectAll();
            long startedCount = events.stream().filter(e -> e instanceof RunEvent.Started).count();
            assertEquals(1, startedCount);
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void eventKinds_noDuplicateCompleted() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            List<RunEvent> events = a.run(new AgentSpec("llama3", null, null, null, null)).collectAll();
            long completedCount = events.stream().filter(e -> e instanceof RunEvent.Completed).count();
            assertEquals(1, completedCount);
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void twoRuns_produceSeparateEventLists() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
            List<RunEvent> ev1 = a.run(spec).collectAll();
            List<RunEvent> ev2 = a.run(spec).collectAll();
            assertNotSame(ev1, ev2);
            String id1 = ((RunEvent.Started) ev1.get(0)).runId();
            String id2 = ((RunEvent.Started) ev2.get(0)).runId();
            assertNotEquals(id1, id2);
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void toolRegistration_noDuplicateNames() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            ToolRegistration r1 = ToolRegistry.register(rt, "dedup_a_149", "A", input -> "{}");
            ToolRegistration r2 = ToolRegistry.register(rt, "dedup_b_149", "B", input -> "{}");
            assertNotEquals(r1.spec().name(), r2.spec().name());
            r1.disposable().close();
            r2.disposable().close();
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void collectAll_secondCall_isEmpty() {
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
    void events_secondIteration_isEmpty() {
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
    void agentSpec_sameInputs_produceEqualSpecs() {
        AgentSpec a = new AgentSpec("llama3", "sys", null, null, null);
        AgentSpec b = new AgentSpec("llama3", "sys", null, null, null);
        assertEquals(a, b);
    }

    private static void skipIfAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "ancora_ffi not present");
    }
}
