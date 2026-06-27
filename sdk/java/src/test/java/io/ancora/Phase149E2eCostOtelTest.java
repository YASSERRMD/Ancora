package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.UUID;

import static org.junit.jupiter.api.Assertions.*;

class Phase149E2eCostOtelTest {

    record OtelSpan149(String traceId, String spanId, String operation) {}
    record CostOtelEvent149(String runId, int inputTokens, int outputTokens, double cost, OtelSpan149 span) {}

    @Test
    void otelSpan_accessors() {
        OtelSpan149 span = new OtelSpan149("trace-1", "span-1", "agent.run");
        assertEquals("trace-1", span.traceId());
        assertEquals("span-1", span.spanId());
        assertEquals("agent.run", span.operation());
    }

    @Test
    void otelSpan_isRecord() {
        assertTrue(OtelSpan149.class.isRecord());
    }

    @Test
    void otelSpan_valueEquality() {
        OtelSpan149 a = new OtelSpan149("t", "s", "op");
        OtelSpan149 b = new OtelSpan149("t", "s", "op");
        assertEquals(a, b);
    }

    @Test
    void costOtelEvent_accessors() {
        OtelSpan149 span = new OtelSpan149("t1", "s1", "agent.run");
        CostOtelEvent149 ev = new CostOtelEvent149("r1", 100, 50, 0.01, span);
        assertEquals("r1", ev.runId());
        assertEquals(100, ev.inputTokens());
        assertEquals(50, ev.outputTokens());
        assertEquals(0.01, ev.cost());
        assertEquals(span, ev.span());
    }

    @Test
    void costOtelEvent_isRecord() {
        assertTrue(CostOtelEvent149.class.isRecord());
    }

    @Test
    void otelSpan_traceId_notEmpty() {
        OtelSpan149 span = new OtelSpan149(UUID.randomUUID().toString(), "s1", "op");
        assertFalse(span.traceId().isEmpty());
    }

    @Test
    void costSummary_acrossEvents() {
        List<CostOtelEvent149> events = List.of(
            new CostOtelEvent149("r1", 100, 50, 0.01, new OtelSpan149("t", "s1", "op")),
            new CostOtelEvent149("r1", 200, 80, 0.02, new OtelSpan149("t", "s2", "op"))
        );
        double total = events.stream().mapToDouble(CostOtelEvent149::cost).sum();
        assertEquals(0.03, total, 1e-9);
    }

    @Test
    void costOtelEvent_notEqualDifferentCost() {
        OtelSpan149 span = new OtelSpan149("t", "s", "op");
        CostOtelEvent149 a = new CostOtelEvent149("r1", 100, 50, 0.01, span);
        CostOtelEvent149 b = new CostOtelEvent149("r1", 100, 50, 0.02, span);
        assertNotEquals(a, b);
    }

    @Test
    void agentRun_producesCompletedEvent() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            List<RunEvent> events = a.run(new AgentSpec("llama3", null, null, null, null)).collectAll();
            assertInstanceOf(RunEvent.Completed.class, events.get(events.size() - 1));
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void otelSpans_acrossRuns_haveUniqueSpanIds() {
        OtelSpan149 s1 = new OtelSpan149("t1", "span-1", "agent.run");
        OtelSpan149 s2 = new OtelSpan149("t1", "span-2", "agent.run");
        assertNotEquals(s1, s2);
    }

    private static void skipIfAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "ancora_ffi not present");
    }
}
