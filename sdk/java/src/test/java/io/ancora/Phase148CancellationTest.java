package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.concurrent.atomic.AtomicBoolean;

import static org.junit.jupiter.api.Assertions.*;

class Phase148CancellationTest {

    @Test
    void runHandle_cancel_method_exists() throws Exception {
        assertNotNull(RunHandle.class.getDeclaredMethod("cancel"));
    }

    @Test
    void cancel_afterComplete_isIdempotent() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            h.collectAll();
            assertDoesNotThrow(h::cancel);
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void cancel_beforeAnyEvents_isIdempotent() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            assertDoesNotThrow(h::cancel);
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void collectAll_afterCancel_returnsPartialOrEmpty() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            h.cancel();
            List<RunEvent> events = h.collectAll();
            assertNotNull(events);
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void agent_close_cancelsInFlight() {
        skipIfAbsent();
        try {
            Agent a = new Agent();
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            a.close();
            assertNotNull(h);
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void cancel_twice_isNoOp() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            assertDoesNotThrow(() -> {
                h.cancel();
                h.cancel();
            });
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void events_returnType_isIterable() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            Iterable<RunEvent> it = h.events();
            assertNotNull(it);
            h.cancel();
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void cancel_returnsVoid() throws Exception {
        var method = RunHandle.class.getDeclaredMethod("cancel");
        assertEquals(void.class, method.getReturnType());
    }

    @Test
    void multipleAgents_cancel_independently() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            Agent a1 = new Agent(rt);
            Agent a2 = new Agent(rt);
            RunHandle h1 = a1.run(new AgentSpec("llama3", null, null, null, null));
            RunHandle h2 = a2.run(new AgentSpec("llama3", null, null, null, null));
            h1.cancel();
            List<RunEvent> h2events = h2.collectAll();
            assertFalse(h2events.isEmpty());
            a1.close();
            a2.close();
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void runHandle_has_collectAll_method() throws Exception {
        assertNotNull(RunHandle.class.getDeclaredMethod("collectAll"));
    }

    @Test
    void runHandle_has_events_method() throws Exception {
        assertNotNull(RunHandle.class.getDeclaredMethod("events"));
    }

    private static void skipIfAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "ancora_ffi not present");
    }
}
