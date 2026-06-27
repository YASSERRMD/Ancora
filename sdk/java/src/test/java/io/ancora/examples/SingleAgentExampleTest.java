package io.ancora.examples;

import io.ancora.Agent;
import io.ancora.AgentSpec;
import io.ancora.RunEvent;
import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class SingleAgentExampleTest {

    @Test
    void run_returns_non_empty_events() throws Throwable {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "native library absent");
        try (Agent agent = new Agent()) {
            AgentSpec spec = new AgentSpec("local-model", "Respond with a greeting.", null, null, null);
            List<RunEvent> events = agent.run(spec).collectAll();
            assertFalse(events.isEmpty());
        } catch (UnsatisfiedLinkError ignored) {}
    }

    @Test
    void run_first_event_is_started() throws Throwable {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "native library absent");
        try (Agent agent = new Agent()) {
            AgentSpec spec = new AgentSpec("local-model", "Respond.", null, null, null);
            List<RunEvent> events = agent.run(spec).collectAll();
            assertFalse(events.isEmpty());
            assertInstanceOf(RunEvent.Started.class, events.get(0));
        } catch (UnsatisfiedLinkError ignored) {}
    }

    @Test
    void run_last_event_is_completed() throws Throwable {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "native library absent");
        try (Agent agent = new Agent()) {
            AgentSpec spec = new AgentSpec("local-model", "Respond.", null, null, null);
            List<RunEvent> events = agent.run(spec).collectAll();
            assertFalse(events.isEmpty());
            assertInstanceOf(RunEvent.Completed.class, events.get(events.size() - 1));
        } catch (UnsatisfiedLinkError ignored) {}
    }

    @Test
    void runId_is_non_blank() throws Throwable {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "native library absent");
        try (Agent agent = new Agent()) {
            AgentSpec spec = new AgentSpec("local-model", "Respond.", null, null, null);
            var handle = agent.run(spec);
            assertFalse(handle.runId().isBlank());
        } catch (UnsatisfiedLinkError ignored) {}
    }
}
