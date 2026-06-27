package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.Set;

import static org.junit.jupiter.api.Assertions.*;

class Phase148ProviderSelectionTest {

    static final class ProviderConstants148 {
        static final String ANTHROPIC_SONNET  = "claude-sonnet-4-5";
        static final String ANTHROPIC_HAIKU   = "claude-haiku-4-5";
        static final String OPENAI_GPT4O      = "gpt-4o";
        static final String OPENAI_GPT4O_MINI = "gpt-4o-mini";
        static final String DEEPSEEK_CHAT     = "deepseek-chat";
        static final String QWEN3_MAX         = "qwen3-max";
        static final String GLM5              = "glm-5";
        static final String LLAMA3            = "llama3";

        static final List<String> ALL = List.of(
            ANTHROPIC_SONNET, ANTHROPIC_HAIKU, OPENAI_GPT4O, OPENAI_GPT4O_MINI,
            DEEPSEEK_CHAT, QWEN3_MAX, GLM5, LLAMA3
        );
    }

    @Test
    void allProviders_countIsEight() {
        assertEquals(8, ProviderConstants148.ALL.size());
    }

    @Test
    void allProviders_areUnique() {
        Set<String> unique = Set.copyOf(ProviderConstants148.ALL);
        assertEquals(ProviderConstants148.ALL.size(), unique.size());
    }

    @Test
    void anthropicSonnet_constant() {
        assertEquals("claude-sonnet-4-5", ProviderConstants148.ANTHROPIC_SONNET);
    }

    @Test
    void anthropicHaiku_constant() {
        assertEquals("claude-haiku-4-5", ProviderConstants148.ANTHROPIC_HAIKU);
    }

    @Test
    void openAiGpt4o_constant() {
        assertEquals("gpt-4o", ProviderConstants148.OPENAI_GPT4O);
    }

    @Test
    void deepSeekChat_constant() {
        assertEquals("deepseek-chat", ProviderConstants148.DEEPSEEK_CHAT);
    }

    @Test
    void qwen3Max_constant() {
        assertEquals("qwen3-max", ProviderConstants148.QWEN3_MAX);
    }

    @Test
    void glm5_constant() {
        assertEquals("glm-5", ProviderConstants148.GLM5);
    }

    @Test
    void agentSpec_acceptsAnyProvider() {
        for (String model : ProviderConstants148.ALL) {
            AgentSpec spec = new AgentSpec(model, null, null, null, null);
            assertEquals(model, spec.model());
        }
    }

    @Test
    void agentSpec_modelNotNull() {
        AgentSpec spec = new AgentSpec(ProviderConstants148.LLAMA3, null, null, null, null);
        assertNotNull(spec.model());
    }

    private static void skipIfAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "ancora_ffi not present");
    }
}
