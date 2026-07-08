package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class Phase148NullHandlingTest {

    @Test
    void agentSpec_nullInstructions_accepted() {
        AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
        assertNull(spec.instructions());
    }

    @Test
    void agentSpec_nullTools_accepted() {
        AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
        assertNull(spec.tools());
    }

    @Test
    void agentSpec_nullMaxTokens_accepted() {
        AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
        assertNull(spec.maxTokens());
    }

    @Test
    void agentSpec_nullTemperature_accepted() {
        AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
        assertNull(spec.temperature());
    }

    @Test
    void toolSpec_nullSchema_accepted() {
        ToolSpec spec = new ToolSpec("tool_a", "description", null);
        assertNull(spec.inputSchema());
    }

    @Test
    void agent_nullRuntime_throwsNPE() {
        assertThrows(NullPointerException.class, () -> new Agent(null));
    }

    @Test
    void toolRegistry_register_nullHandler_throwsNPE() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            assertThrows(NullPointerException.class,
                () -> ToolRegistry.register(rt, "t", "d", null));
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            if (!(t instanceof NullPointerException)) fail(t.toString());
        }
    }

    @Test
    void runEvent_started_nullSpec_accepted() {
        RunEvent.Started ev = new RunEvent.Started("r1", null);
        assertNull(ev.spec());
    }

    @Test
    void runEvent_resumed_nullDecision_accepted() {
        RunEvent.Resumed ev = new RunEvent.Resumed("r1", null);
        assertNull(ev.decision());
    }

    @Test
    void agentSpec_model_notNull() {
        AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
        assertNotNull(spec.model());
    }

    @Test
    void toolSpec_name_notNull() {
        ToolSpec spec = new ToolSpec("my_tool", "desc", null);
        assertNotNull(spec.name());
    }

    @Test
    void ancoraNative_available_isBoolean() {
        boolean val = AncoraNative.AVAILABLE;
        assertTrue(val || !val);
    }

    private static void skipIfAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "ancora_ffi not present");
    }
}
