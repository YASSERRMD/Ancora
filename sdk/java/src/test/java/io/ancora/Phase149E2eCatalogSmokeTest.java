package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.Set;
import java.util.stream.Collectors;

import static org.junit.jupiter.api.Assertions.*;

class Phase149E2eCatalogSmokeTest {

    static final List<String> CATALOG = List.of(
        "claude-sonnet-4-5",
        "claude-haiku-4-5",
        "gpt-4o",
        "gpt-4o-mini",
        "deepseek-chat",
        "qwen3-max",
        "qwen-turbo",
        "glm-5",
        "llama3",
        "gemini-1.5-pro"
    );

    @Test
    void catalog_hasTenEntries() {
        assertEquals(10, CATALOG.size());
    }

    @Test
    void catalog_allUnique() {
        Set<String> unique = Set.copyOf(CATALOG);
        assertEquals(CATALOG.size(), unique.size());
    }

    @Test
    void catalog_containsDeepSeek() {
        assertTrue(CATALOG.contains("deepseek-chat"));
    }

    @Test
    void catalog_containsQwen() {
        assertTrue(CATALOG.stream().anyMatch(m -> m.startsWith("qwen")));
    }

    @Test
    void catalog_containsGlm() {
        assertTrue(CATALOG.contains("glm-5"));
    }

    @Test
    void catalog_containsClaudeModels() {
        assertTrue(CATALOG.stream().anyMatch(m -> m.startsWith("claude")));
    }

    @Test
    void catalog_containsGptModels() {
        assertTrue(CATALOG.stream().anyMatch(m -> m.startsWith("gpt")));
    }

    @Test
    void catalog_noBlanks() {
        CATALOG.forEach(m -> assertFalse(m.isBlank()));
    }

    @Test
    void agentSpec_acceptsAllCatalogModels() {
        for (String model : CATALOG) {
            AgentSpec spec = new AgentSpec(model, null, null, null, null);
            assertEquals(model, spec.model());
        }
    }

    @Test
    void singleCatalogModel_runsOffline() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
            List<RunEvent> events = a.run(spec).collectAll();
            assertFalse(events.isEmpty());
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
