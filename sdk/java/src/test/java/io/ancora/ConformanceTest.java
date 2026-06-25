package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Conformance scenarios for the Java SDK.
 * These mirror the same scenarios verified by the Rust core and other SDK
 * conformance suites. Integration tests require the native library and are
 * skipped gracefully when it is absent.
 */
class ConformanceTest {

    // --- fixture: event parsing (no native lib required) ---

    @Test
    void fixture_started_event_parses_correctly() throws Exception {
        String json = "{\"kind\":\"started\",\"run_id\":\"fixture-run-1\","
            + "\"spec\":\"{\\\"model\\\":\\\"claude-3-5-sonnet\\\"}\"}";
        RunEvent ev = Wire.parseEvent(json);
        RunEvent.Started started = assertInstanceOf(RunEvent.Started.class, ev);
        assertEquals("fixture-run-1", started.runId());
        assertTrue(started.spec().contains("claude-3-5-sonnet"));
    }

    @Test
    void fixture_token_event_parses_correctly() throws Exception {
        String json = "{\"kind\":\"token\",\"run_id\":\"fixture-run-1\",\"text\":\"Hello\"}";
        RunEvent ev = Wire.parseEvent(json);
        RunEvent.Token token = assertInstanceOf(RunEvent.Token.class, ev);
        assertEquals("Hello", token.text());
    }

    @Test
    void fixture_completed_event_parses_correctly() throws Exception {
        String json = "{\"kind\":\"completed\",\"run_id\":\"fixture-run-1\"}";
        RunEvent ev = Wire.parseEvent(json);
        RunEvent.Completed completed = assertInstanceOf(RunEvent.Completed.class, ev);
        assertEquals("fixture-run-1", completed.runId());
    }

    @Test
    void fixture_tool_call_event_parses_correctly() throws Exception {
        String json = "{\"kind\":\"tool_call\",\"run_id\":\"fixture-run-2\","
            + "\"name\":\"get_weather\",\"input\":\"{\\\"city\\\":\\\"Paris\\\"}\"}";
        RunEvent ev = Wire.parseEvent(json);
        RunEvent.ToolCall tc = assertInstanceOf(RunEvent.ToolCall.class, ev);
        assertEquals("get_weather", tc.name());
        assertTrue(tc.input().contains("Paris"));
    }

    @Test
    void fixture_resumed_event_parses_correctly() throws Exception {
        String json = "{\"kind\":\"resumed\",\"run_id\":\"fixture-run-2\","
            + "\"decision\":\"{\\\"temperature\\\":\\\"22C\\\"}\"}";
        RunEvent ev = Wire.parseEvent(json);
        RunEvent.Resumed resumed = assertInstanceOf(RunEvent.Resumed.class, ev);
        assertTrue(resumed.decision().contains("22C"));
    }

    @Test
    void fixture_agent_spec_serializes_to_matching_fields() throws Exception {
        AgentSpec spec = new AgentSpec("claude-3-5-sonnet", "You are helpful.", null, 1024, 0.7);
        String json = new String(Wire.encodeAgentSpec(spec));
        assertTrue(json.contains("\"model\""));
        assertTrue(json.contains("claude-3-5-sonnet"));
        assertTrue(json.contains("1024"));
        assertTrue(json.contains("0.7"));
    }

    // --- single agent (native lib required) ---

    @Test
    void single_agent_started_event_arrives_first() throws Throwable {
        skipIfNativeLibraryAbsent();
        try (Agent agent = new Agent()) {
            AgentSpec spec = new AgentSpec("conformance-model", "Conform to the spec",
                null, null, null);
            List<RunEvent> events = agent.run(spec).collectAll();
            assertFalse(events.isEmpty());
            assertInstanceOf(RunEvent.Started.class, events.get(0));
        } catch (UnsatisfiedLinkError e) {
            skipIfNativeLibraryAbsent();
        } catch (Throwable t) {
            fail("Unexpected: " + t);
        }
    }

    @Test
    void single_agent_completed_event_arrives_last() throws Throwable {
        skipIfNativeLibraryAbsent();
        try (Agent agent = new Agent()) {
            AgentSpec spec = new AgentSpec("conformance-model", "Complete after one step",
                null, null, null);
            List<RunEvent> events = agent.run(spec).collectAll();
            assertFalse(events.isEmpty());
            assertInstanceOf(RunEvent.Completed.class, events.get(events.size() - 1));
        } catch (UnsatisfiedLinkError e) {
            skipIfNativeLibraryAbsent();
        } catch (Throwable t) {
            fail("Unexpected: " + t);
        }
    }

    @Test
    void single_agent_run_id_is_nonempty() throws Throwable {
        skipIfNativeLibraryAbsent();
        try (Agent agent = new Agent()) {
            AgentSpec spec = new AgentSpec("conformance-model", "Non-empty run id",
                null, null, null);
            RunHandle handle = agent.run(spec);
            assertFalse(handle.runId().isBlank());
            handle.collectAll();
        } catch (UnsatisfiedLinkError e) {
            skipIfNativeLibraryAbsent();
        } catch (Throwable t) {
            fail("Unexpected: " + t);
        }
    }

    @Test
    void single_agent_run_id_consistent_across_events() throws Throwable {
        skipIfNativeLibraryAbsent();
        try (Agent agent = new Agent()) {
            AgentSpec spec = new AgentSpec("conformance-model", "Consistent run id",
                null, null, null);
            RunHandle handle = agent.run(spec);
            String runId = handle.runId();
            for (RunEvent ev : handle.events()) {
                switch (ev) {
                    case RunEvent.Started s -> assertEquals(runId, s.runId());
                    case RunEvent.Completed c -> assertEquals(runId, c.runId());
                    case RunEvent.Token t -> assertEquals(runId, t.runId());
                    case RunEvent.ToolCall tc -> assertEquals(runId, tc.runId());
                    case RunEvent.Resumed r -> assertEquals(runId, r.runId());
                }
            }
        } catch (UnsatisfiedLinkError e) {
            skipIfNativeLibraryAbsent();
        } catch (Throwable t) {
            fail("Unexpected: " + t);
        }
    }

    // --- multi-run (native lib required) ---

    @Test
    void multi_run_each_gets_distinct_run_id() throws Throwable {
        skipIfNativeLibraryAbsent();
        try (Agent agent = new Agent()) {
            AgentSpec spec = new AgentSpec("conformance-model", "Multi-run", null, null, null);
            RunHandle h1 = agent.run(spec);
            RunHandle h2 = agent.run(spec);
            assertNotEquals(h1.runId(), h2.runId(), "each run must have a unique run id");
            h1.collectAll();
            h2.collectAll();
        } catch (UnsatisfiedLinkError e) {
            skipIfNativeLibraryAbsent();
        } catch (Throwable t) {
            fail("Unexpected: " + t);
        }
    }

    // --- human-in-loop (native lib required) ---

    @Test
    void resume_produces_resumed_event() throws Throwable {
        skipIfNativeLibraryAbsent();
        try (Agent agent = new Agent()) {
            AgentSpec spec = new AgentSpec("conformance-model", "Await decision",
                null, null, null);
            RunHandle handle = agent.run(spec);
            handle.collectAll();
            List<RunEvent> afterResume = handle.resumeAndCollectAll("approved".getBytes());
            assertFalse(afterResume.isEmpty());
            assertInstanceOf(RunEvent.Resumed.class, afterResume.get(0));
        } catch (UnsatisfiedLinkError e) {
            skipIfNativeLibraryAbsent();
        } catch (Throwable t) {
            fail("Unexpected: " + t);
        }
    }

    @Test
    void resume_completed_event_is_last_after_decision() throws Throwable {
        skipIfNativeLibraryAbsent();
        try (Agent agent = new Agent()) {
            AgentSpec spec = new AgentSpec("conformance-model", "Decision flow",
                null, null, null);
            RunHandle handle = agent.run(spec);
            handle.collectAll();
            List<RunEvent> afterResume = handle.resumeAndCollectAll("approved".getBytes());
            assertTrue(afterResume.size() >= 2);
            assertInstanceOf(RunEvent.Completed.class, afterResume.get(afterResume.size() - 1));
        } catch (UnsatisfiedLinkError e) {
            skipIfNativeLibraryAbsent();
        } catch (Throwable t) {
            fail("Unexpected: " + t);
        }
    }

    // --- crash recovery (native lib required) ---

    @Test
    void dispose_and_recreate_agent_does_not_corrupt_state() throws Throwable {
        skipIfNativeLibraryAbsent();
        try {
            AgentSpec spec = new AgentSpec("conformance-model", "Crash scenario",
                null, null, null);
            String firstRunId;
            Agent agent1 = new Agent();
            RunHandle h1 = agent1.run(spec);
            firstRunId = h1.runId();
            agent1.close();

            try (Agent agent2 = new Agent()) {
                RunHandle h2 = agent2.run(spec);
                assertNotEquals(firstRunId, h2.runId());
                List<RunEvent> events = h2.collectAll();
                assertFalse(events.isEmpty());
            }
        } catch (UnsatisfiedLinkError e) {
            skipIfNativeLibraryAbsent();
        } catch (Throwable t) {
            fail("Unexpected: " + t);
        }
    }

    // --- cost (native lib required) ---

    @Test
    void cost_json_contains_run_id() throws Throwable {
        skipIfNativeLibraryAbsent();
        try (Agent agent = new Agent()) {
            AgentSpec spec = new AgentSpec("conformance-model", "Cost tracking",
                null, null, null);
            RunHandle handle = agent.run(spec);
            handle.collectAll();
            String cost = handle.getCost();
            assertTrue(cost.contains(handle.runId()),
                "cost JSON should contain the run id");
        } catch (UnsatisfiedLinkError e) {
            skipIfNativeLibraryAbsent();
        } catch (Throwable t) {
            fail("Unexpected: " + t);
        }
    }

    @Test
    void cost_json_contains_total_usd_field() throws Throwable {
        skipIfNativeLibraryAbsent();
        try (Agent agent = new Agent()) {
            AgentSpec spec = new AgentSpec("conformance-model", "Cost USD field",
                null, null, null);
            RunHandle handle = agent.run(spec);
            handle.collectAll();
            String cost = handle.getCost();
            assertTrue(cost.contains("total_usd"),
                "cost JSON should contain total_usd field");
        } catch (UnsatisfiedLinkError e) {
            skipIfNativeLibraryAbsent();
        } catch (Throwable t) {
            fail("Unexpected: " + t);
        }
    }

    private static void skipIfNativeLibraryAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE,
            "ancora_ffi native library not present; CI provides it.");
    }
}
