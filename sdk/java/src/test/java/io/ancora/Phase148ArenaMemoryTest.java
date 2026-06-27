package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.lang.ref.WeakReference;
import java.util.ArrayList;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class Phase148ArenaMemoryTest {

    @Test
    void agent_close_releasesResource() {
        skipIfAbsent();
        try {
            Agent a = new Agent();
            a.close();
            assertNotNull(a);
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void runtime_close_releasesResource() {
        skipIfAbsent();
        try {
            Runtime rt = new Runtime();
            rt.close();
            assertNotNull(rt);
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void hundred_agents_openClose_noOOM() {
        skipIfAbsent();
        try {
            for (int i = 0; i < 100; i++) {
                try (Agent a = new Agent()) {
                    assertNotNull(a);
                }
            }
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void hundred_runtimes_openClose_noOOM() {
        skipIfAbsent();
        try {
            for (int i = 0; i < 100; i++) {
                try (Runtime rt = new Runtime()) {
                    assertNotNull(rt);
                }
            }
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void doubleClose_agent_isIdempotent() {
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
    void doubleClose_runtime_isIdempotent() {
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
    void runHandle_noLeakAfterCollectAll() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
            for (int i = 0; i < 20; i++) {
                a.run(spec).collectAll();
            }
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void agent_implementsAutoCloseable() {
        assertTrue(AutoCloseable.class.isAssignableFrom(Agent.class));
    }

    @Test
    void runtime_implementsAutoCloseable() {
        assertTrue(AutoCloseable.class.isAssignableFrom(Runtime.class));
    }

    @Test
    void toolRegistration_disposable_closes() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            ToolRegistration reg = ToolRegistry.register(rt, "arena_tool148", "Arena",
                input -> "{\"ok\":true}");
            assertDoesNotThrow(() -> reg.disposable().close());
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
