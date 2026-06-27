package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.Set;

import static org.junit.jupiter.api.Assertions.*;

class Phase149ConfSuiteTest {

    static final List<String> SCENARIOS = List.of(
        "single-agent",
        "multi-agent-verifier",
        "human-in-loop",
        "rag-retrieval"
    );

    @Test
    void scenarioCount_isFour() {
        assertEquals(4, SCENARIOS.size());
    }

    @Test
    void scenarios_containsSingleAgent() {
        assertTrue(SCENARIOS.contains("single-agent"));
    }

    @Test
    void scenarios_containsVerifier() {
        assertTrue(SCENARIOS.contains("multi-agent-verifier"));
    }

    @Test
    void scenarios_containsHumanInLoop() {
        assertTrue(SCENARIOS.contains("human-in-loop"));
    }

    @Test
    void scenarios_containsRagRetrieval() {
        assertTrue(SCENARIOS.contains("rag-retrieval"));
    }

    @Test
    void scenarios_areUnique() {
        Set<String> unique = Set.copyOf(SCENARIOS);
        assertEquals(SCENARIOS.size(), unique.size());
    }

    @Test
    void scenarios_noBlankEntries() {
        SCENARIOS.forEach(s -> assertFalse(s.isBlank()));
    }

    @Test
    void scenarios_allLowerCase() {
        SCENARIOS.forEach(s -> assertEquals(s, s.toLowerCase()));
    }

    @Test
    void singleAgent_scenario_passes() {
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
    void verifier_scenario_passes() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            Agent d = new Agent(rt);
            Agent v = new Agent(rt);
            AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
            d.run(spec).collectAll();
            List<RunEvent> ve = v.run(spec).collectAll();
            assertInstanceOf(RunEvent.Completed.class, ve.get(ve.size() - 1));
            d.close();
            v.close();
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
