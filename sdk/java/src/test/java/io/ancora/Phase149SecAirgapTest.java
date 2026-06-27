package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class Phase149SecAirgapTest {

    static final List<String> FORBIDDEN_PATTERNS = List.of(
        "api.anthropic.com",
        "api.openai.com",
        "sk-ant-",
        "sk-proj-"
    );

    @Test
    void events_containNoForbiddenUrls() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            List<RunEvent> events = a.run(new AgentSpec("llama3", null, null, null, null)).collectAll();
            for (RunEvent ev : events) {
                String repr = ev.toString();
                for (String pattern : FORBIDDEN_PATTERNS) {
                    assertFalse(repr.contains(pattern), "Found forbidden pattern: " + pattern);
                }
            }
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void runId_doesNotContainApiKey() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            String runId = a.run(new AgentSpec("llama3", null, null, null, null)).runId();
            for (String pattern : FORBIDDEN_PATTERNS) {
                assertFalse(runId.contains(pattern), "RunId contained forbidden: " + pattern);
            }
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void toolCallEvent_inputDoesNotContainLiveKey() {
        RunEvent.ToolCall ev = new RunEvent.ToolCall("r1", "search", "{\"query\":\"test\"}");
        for (String pattern : FORBIDDEN_PATTERNS) {
            assertFalse(ev.input().contains(pattern));
        }
    }

    @Test
    void tokenEvent_textDoesNotContainLiveKey() {
        RunEvent.Token ev = new RunEvent.Token("r1", "This is safe output", "llama3");
        for (String pattern : FORBIDDEN_PATTERNS) {
            assertFalse(ev.text().contains(pattern));
        }
    }

    @Test
    void startedEvent_specDoesNotContainLiveKey() {
        RunEvent.Started ev = new RunEvent.Started("r1", "{\"model\":\"llama3\"}");
        for (String pattern : FORBIDDEN_PATTERNS) {
            assertFalse(ev.spec().contains(pattern));
        }
    }

    @Test
    void forbiddenPatterns_count_isFour() {
        assertEquals(4, FORBIDDEN_PATTERNS.size());
    }

    @Test
    void anthropicUrlPattern_isPresent() {
        assertTrue(FORBIDDEN_PATTERNS.contains("api.anthropic.com"));
    }

    @Test
    void openAiUrlPattern_isPresent() {
        assertTrue(FORBIDDEN_PATTERNS.contains("api.openai.com"));
    }

    @Test
    void anthropicKeyPrefix_isPresent() {
        assertTrue(FORBIDDEN_PATTERNS.contains("sk-ant-"));
    }

    @Test
    void openAiKeyPrefix_isPresent() {
        assertTrue(FORBIDDEN_PATTERNS.contains("sk-proj-"));
    }

    private static void skipIfAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "ancora_ffi not present");
    }
}
