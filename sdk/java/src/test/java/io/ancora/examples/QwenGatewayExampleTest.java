package io.ancora.examples;

import io.ancora.Agent;
import io.ancora.AgentSpec;
import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class QwenGatewayExampleTest {

    private static final List<String> QWEN_MODELS = List.of(
        "qwen-turbo", "qwen-plus", "qwen-max", "qwen-long"
    );

    @Test
    void model_list_has_four_entries() {
        assertEquals(4, QWEN_MODELS.size());
    }

    @Test
    void model_names_are_distinct() {
        assertEquals(QWEN_MODELS.size(), new HashSet<>(QWEN_MODELS).size());
    }

    @Test
    void all_model_names_start_with_qwen() {
        assertTrue(QWEN_MODELS.stream().allMatch(m -> m.startsWith("qwen-")));
    }

    @Test
    void each_model_produces_distinct_run_id() throws Throwable {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "native library absent");
        try (Agent agent = new Agent()) {
            var runIds = new ArrayList<String>();
            for (String model : QWEN_MODELS) {
                AgentSpec spec = new AgentSpec(model, "Respond briefly.", null, null, null);
                var handle = agent.run(spec);
                runIds.add(handle.runId());
                handle.collectAll();
            }
            assertEquals(runIds.size(), new HashSet<>(runIds).size());
        } catch (UnsatisfiedLinkError ignored) {}
    }
}
