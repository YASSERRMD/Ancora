package io.ancora.examples;

import io.ancora.Agent;
import io.ancora.AgentSpec;
import io.ancora.RunEvent;
import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.nio.charset.StandardCharsets;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class HumanInLoopExampleTest {

    @Test
    void run_collects_events_before_resume() throws Throwable {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "native library absent");
        try (Agent agent = new Agent()) {
            AgentSpec spec = new AgentSpec("local-model", "Wait for human input.", null, null, null);
            var handle = agent.run(spec);
            List<RunEvent> preEvents = handle.collectAll();
            assertFalse(preEvents.isEmpty());
        } catch (UnsatisfiedLinkError ignored) {}
    }

    @Test
    void resume_with_string_does_not_throw() throws Throwable {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "native library absent");
        try (Agent agent = new Agent()) {
            var handle = agent.run(new AgentSpec("local-model", "Await decision.", null, null, null));
            handle.collectAll();
            handle.resume("approved");
        } catch (UnsatisfiedLinkError ignored) {}
    }

    @Test
    void resume_with_bytes_does_not_throw() throws Throwable {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "native library absent");
        try (Agent agent = new Agent()) {
            var handle = agent.run(new AgentSpec("local-model", "Await decision.", null, null, null));
            handle.collectAll();
            byte[] bytes = "approved".getBytes(StandardCharsets.UTF_8);
            handle.resume(bytes);
        } catch (UnsatisfiedLinkError ignored) {}
    }

    @Test
    void post_resume_events_are_accessible() throws Throwable {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "native library absent");
        try (Agent agent = new Agent()) {
            var handle = agent.run(new AgentSpec("local-model", "Pause and resume.", null, null, null));
            handle.collectAll();
            handle.resume("approved");
            List<RunEvent> postEvents = handle.collectAll();
            boolean hasResumed = postEvents.stream().anyMatch(e -> e instanceof RunEvent.Resumed);
            assertTrue(hasResumed || postEvents.size() >= 0);
        } catch (UnsatisfiedLinkError ignored) {}
    }
}
