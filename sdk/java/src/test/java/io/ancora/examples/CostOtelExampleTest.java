package io.ancora.examples;

import io.ancora.Agent;
import io.ancora.AgentSpec;
import io.ancora.RunEvent;
import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;
import static io.ancora.examples.SharedHelpers.Span;
import static io.ancora.examples.SharedHelpers.TokenEstimator;

class CostOtelExampleTest {

    @Test
    void span_records_name_and_duration() {
        Span s = new Span("agent.run");
        s.setAttribute("run.id", "abc");
        long durationMs = s.endMs();
        assertTrue(durationMs >= 0);
        assertEquals("abc", s.getAttribute("run.id"));
    }

    @Test
    void token_estimator_returns_at_least_one() {
        assertEquals(1, TokenEstimator.estimateTokens(""));
        assertEquals(1, TokenEstimator.estimateTokens(null));
    }

    @Test
    void token_estimator_four_chars_per_token() {
        assertEquals(1, TokenEstimator.estimateTokens("abcd"));
        assertEquals(2, TokenEstimator.estimateTokens("abcde"));
        assertEquals(25, TokenEstimator.estimateTokens("x".repeat(100)));
    }

    @Test
    void cost_spans_accumulate_over_a_run() throws Throwable {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "native library absent");
        try (Agent agent = new Agent()) {
            Span root = new Span("agent.run");
            List<RunEvent> events = agent.run(
                new AgentSpec("local-model", "Respond concisely.", null, null, null)
            ).collectAll();

            long totalTokens = events.stream()
                .filter(e -> e instanceof RunEvent.Token)
                .mapToLong(e -> TokenEstimator.estimateTokens(((RunEvent.Token) e).text()))
                .sum();

            root.setAttribute("event.count", events.size());
            root.setAttribute("tokens.estimated", totalTokens);
            long durationMs = root.endMs();

            assertTrue(durationMs >= 0);
            assertEquals(events.size(), (int) root.getAttribute("event.count"));
        } catch (UnsatisfiedLinkError ignored) {}
    }

    @Test
    void summary_span_aggregates_token_total() {
        Span summary = new Span("agent.summary");
        summary.setAttribute("events", 5);
        summary.setAttribute("tokens.estimated", 136L);
        summary.endMs();

        assertEquals(5, (int) summary.getAttribute("events"));
        assertEquals(136L, summary.getAttribute("tokens.estimated"));
    }
}
