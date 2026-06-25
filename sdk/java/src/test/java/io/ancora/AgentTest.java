package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.nio.charset.StandardCharsets;
import java.util.List;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.*;

class AgentTest {

    // --- serialization tests (no native lib required) ---

    @Test
    void agentSpec_minimal_omits_null_fields() throws Exception {
        AgentSpec spec = new AgentSpec("claude-3-5-haiku-20241022", "You are helpful.", null, null, null);
        String json = new String(Wire.encodeAgentSpec(spec), StandardCharsets.UTF_8);
        assertTrue(json.contains("\"model\""));
        assertTrue(json.contains("claude-3-5-haiku-20241022"));
        assertFalse(json.contains("tools"), "null tools should be omitted");
        assertFalse(json.contains("max_tokens"), "null max_tokens should be omitted");
        assertFalse(json.contains("temperature"), "null temperature should be omitted");
    }

    @Test
    void agentSpec_with_tools_includes_input_schema() throws Exception {
        ToolInputProperty prop = new ToolInputProperty("string", "city name");
        ToolInputSchema schema = new ToolInputSchema("object", Map.of("city", prop), List.of("city"));
        ToolSpec tool = new ToolSpec("get_weather", "Get weather", schema);
        AgentSpec spec = new AgentSpec("claude-3-5-haiku-20241022", "", List.of(tool), null, null);
        String json = new String(Wire.encodeAgentSpec(spec), StandardCharsets.UTF_8);
        assertTrue(json.contains("\"get_weather\""));
        assertTrue(json.contains("\"input_schema\""));
        assertTrue(json.contains("\"city\""));
    }

    @Test
    void agentSpec_max_tokens_uses_snake_case() throws Exception {
        AgentSpec spec = new AgentSpec("claude-3-5-haiku-20241022", "", null, 1024, null);
        String json = new String(Wire.encodeAgentSpec(spec), StandardCharsets.UTF_8);
        assertTrue(json.contains("\"max_tokens\""), "maxTokens should serialize as max_tokens");
        assertTrue(json.contains("1024"));
    }

    @Test
    void toolInputSchema_defaults_type_to_object() {
        ToolInputSchema schema = new ToolInputSchema(null, null, null);
        assertEquals("object", schema.type());
    }

    @Test
    void toolInputSchema_blank_type_defaults_to_object() {
        ToolInputSchema schema = new ToolInputSchema("  ", null, null);
        assertEquals("object", schema.type());
    }

    @Test
    void runEvent_started_deserializes() throws Exception {
        String json = "{\"kind\":\"started\",\"run_id\":\"abc\",\"spec\":\"{}\"}";
        RunEvent event = Wire.parseEvent(json);
        assertInstanceOf(RunEvent.Started.class, event);
        RunEvent.Started started = (RunEvent.Started) event;
        assertEquals("abc", started.runId());
        assertEquals("{}", started.spec());
    }

    @Test
    void runEvent_completed_deserializes() throws Exception {
        String json = "{\"kind\":\"completed\",\"run_id\":\"abc\"}";
        RunEvent event = Wire.parseEvent(json);
        assertInstanceOf(RunEvent.Completed.class, event);
        assertEquals("abc", ((RunEvent.Completed) event).runId());
    }

    @Test
    void runEvent_token_deserializes() throws Exception {
        String json = "{\"kind\":\"token\",\"run_id\":\"r1\",\"token\":\"hello\",\"model\":\"claude-3\"}";
        RunEvent event = Wire.parseEvent(json);
        assertInstanceOf(RunEvent.Token.class, event);
        RunEvent.Token tok = (RunEvent.Token) event;
        assertEquals("r1", tok.runId());
        assertEquals("hello", tok.token());
        assertEquals("claude-3", tok.model());
    }

    @Test
    void runEvent_tool_call_deserializes() throws Exception {
        String json = "{\"kind\":\"tool_call\",\"run_id\":\"r2\",\"name\":\"search\",\"input\":\"{}\"}";
        RunEvent event = Wire.parseEvent(json);
        assertInstanceOf(RunEvent.ToolCall.class, event);
        RunEvent.ToolCall tc = (RunEvent.ToolCall) event;
        assertEquals("search", tc.name());
        assertEquals("{}", tc.input());
    }

    @Test
    void runEvent_unknown_fields_are_ignored() throws Exception {
        String json = "{\"kind\":\"completed\",\"run_id\":\"r2\",\"extra\":\"ignored\"}";
        RunEvent event = Wire.parseEvent(json);
        assertInstanceOf(RunEvent.Completed.class, event);
    }

    @Test
    void nodeKind_serializes_as_lowercase() throws Exception {
        GraphNode node = new GraphNode("n1", NodeKind.AGENT, null);
        GraphSpec spec = new GraphSpec(List.of(node), List.of());
        String json = new String(Wire.encodeGraphSpec(spec), StandardCharsets.UTF_8);
        assertTrue(json.contains("\"agent\""), "NodeKind.AGENT should serialize as \"agent\"");
    }

    @Test
    void nodeKind_subgraph_serializes() throws Exception {
        GraphNode node = new GraphNode("n1", NodeKind.SUBGRAPH, null);
        GraphSpec spec = new GraphSpec(List.of(node), List.of());
        String json = new String(Wire.encodeGraphSpec(spec), StandardCharsets.UTF_8);
        assertTrue(json.contains("\"subgraph\""));
    }

    @Test
    void agent_implements_auto_closeable() {
        assertTrue(AutoCloseable.class.isAssignableFrom(Agent.class));
    }

    @Test
    void runHandle_is_package_constructible() {
        assertTrue(RunHandle.class.getDeclaredConstructors().length > 0);
    }

    // --- integration tests (require native library) ---

    @Test
    void runHandle_runId_is_nonempty() throws Throwable {
        skipIfNativeLibraryAbsent();
        try (Agent agent = new Agent()) {
            AgentSpec spec = new AgentSpec("claude-3-5-haiku-20241022", "You are helpful.", null, null, null);
            RunHandle handle = agent.run(spec);
            assertFalse(handle.runId().isBlank(), "run ID should be a non-empty string");
        } catch (UnsatisfiedLinkError e) {
            skipIfNativeLibraryAbsent();
        } catch (Throwable t) {
            fail("Unexpected exception: " + t);
        }
    }

    @Test
    void agent_run_first_event_is_started() throws Throwable {
        skipIfNativeLibraryAbsent();
        try (Agent agent = new Agent()) {
            AgentSpec spec = new AgentSpec("claude-3-5-haiku-20241022", "You are helpful.", null, null, null);
            RunHandle handle = agent.run(spec);
            RunEvent first = handle.events().iterator().next();
            assertInstanceOf(RunEvent.Started.class, first, "first event should be Started");
        } catch (UnsatisfiedLinkError e) {
            skipIfNativeLibraryAbsent();
        } catch (Throwable t) {
            fail("Unexpected exception: " + t);
        }
    }

    @Test
    void agent_run_completes() throws Throwable {
        skipIfNativeLibraryAbsent();
        try (Agent agent = new Agent()) {
            AgentSpec spec = new AgentSpec("claude-3-5-haiku-20241022", "You are helpful.", null, null, null);
            List<RunEvent> events = agent.run(spec).collectAll();
            assertFalse(events.isEmpty(), "should have at least one event");
            assertInstanceOf(RunEvent.Started.class, events.get(0), "first event should be Started");
            assertInstanceOf(RunEvent.Completed.class, events.get(events.size() - 1), "last event should be Completed");
        } catch (UnsatisfiedLinkError e) {
            skipIfNativeLibraryAbsent();
        } catch (Throwable t) {
            fail("Unexpected exception: " + t);
        }
    }

    @Test
    void agent_run_cost_returns_string() throws Throwable {
        skipIfNativeLibraryAbsent();
        try (Agent agent = new Agent()) {
            AgentSpec spec = new AgentSpec("claude-3-5-haiku-20241022", "You are helpful.", null, null, null);
            RunHandle handle = agent.run(spec);
            handle.collectAll();
            String cost = handle.getCost();
            assertNotNull(cost);
        } catch (UnsatisfiedLinkError e) {
            skipIfNativeLibraryAbsent();
        } catch (Throwable t) {
            fail("Unexpected exception: " + t);
        }
    }

    private static void skipIfNativeLibraryAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE,
            "ancora_ffi native library not present; CI provides it.");
    }
}
