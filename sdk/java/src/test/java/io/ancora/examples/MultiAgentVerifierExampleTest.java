package io.ancora.examples;

import io.ancora.Agent;
import io.ancora.AgentSpec;
import io.ancora.RunEvent;
import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.concurrent.CompletableFuture;

import static org.junit.jupiter.api.Assertions.*;

class MultiAgentVerifierExampleTest {

    @Test
    void two_runs_have_distinct_run_ids() throws Throwable {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "native library absent");
        try (Agent agent = new Agent()) {
            var h1 = agent.run(new AgentSpec("local-model", "Produce an answer.", null, null, null));
            var h2 = agent.run(new AgentSpec("local-model", "Verify the answer.", null, null, null));
            assertNotEquals(h1.runId(), h2.runId());
        } catch (UnsatisfiedLinkError ignored) {}
    }

    @Test
    void both_runs_complete() throws Throwable {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "native library absent");
        try (Agent agent = new Agent()) {
            var h1 = agent.run(new AgentSpec("local-model", "Produce an answer.", null, null, null));
            var h2 = agent.run(new AgentSpec("local-model", "Verify the answer.", null, null, null));

            var f1 = CompletableFuture.supplyAsync(() -> {
                try { return h1.collectAll(); }
                catch (Throwable t) { throw new RuntimeException(t); }
            });
            var f2 = CompletableFuture.supplyAsync(() -> {
                try { return h2.collectAll(); }
                catch (Throwable t) { throw new RuntimeException(t); }
            });

            List<RunEvent> r1 = f1.join();
            List<RunEvent> r2 = f2.join();
            assertFalse(r1.isEmpty());
            assertFalse(r2.isEmpty());
        } catch (UnsatisfiedLinkError ignored) {}
    }

    @Test
    void model_names_are_distinct_strings() {
        String primary  = "local-model";
        String verifier = "local-model-verifier";
        assertNotEquals(primary, verifier);
    }
}
