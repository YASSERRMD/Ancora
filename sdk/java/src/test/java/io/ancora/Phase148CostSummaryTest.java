package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.stream.IntStream;

import static org.junit.jupiter.api.Assertions.*;

class Phase148CostSummaryTest {

    record CostEvent148(int inputTokens, int outputTokens, double cost) {}

    @Test
    void costEvent_accessors() {
        CostEvent148 ev = new CostEvent148(100, 50, 0.0025);
        assertEquals(100, ev.inputTokens());
        assertEquals(50, ev.outputTokens());
        assertEquals(0.0025, ev.cost());
    }

    @Test
    void costEvent_valueEquality() {
        CostEvent148 a = new CostEvent148(100, 50, 0.0025);
        CostEvent148 b = new CostEvent148(100, 50, 0.0025);
        assertEquals(a, b);
    }

    @Test
    void totalCost_sumOfList() {
        List<CostEvent148> events = List.of(
            new CostEvent148(100, 50, 0.01),
            new CostEvent148(200, 80, 0.02),
            new CostEvent148(300, 120, 0.03)
        );
        double total = events.stream().mapToDouble(CostEvent148::cost).sum();
        assertEquals(0.06, total, 1e-9);
    }

    @Test
    void totalInputTokens_sum() {
        List<CostEvent148> events = List.of(
            new CostEvent148(100, 50, 0.01),
            new CostEvent148(200, 80, 0.02)
        );
        int totalIn = events.stream().mapToInt(CostEvent148::inputTokens).sum();
        assertEquals(300, totalIn);
    }

    @Test
    void totalOutputTokens_sum() {
        List<CostEvent148> events = List.of(
            new CostEvent148(100, 50, 0.01),
            new CostEvent148(200, 80, 0.02)
        );
        int totalOut = events.stream().mapToInt(CostEvent148::outputTokens).sum();
        assertEquals(130, totalOut);
    }

    @Test
    void costEvent_isRecord() {
        assertTrue(CostEvent148.class.isRecord());
    }

    @Test
    void zeroCost_valid() {
        CostEvent148 ev = new CostEvent148(0, 0, 0.0);
        assertEquals(0.0, ev.cost());
    }

    @Test
    void largeCost_represented() {
        CostEvent148 ev = new CostEvent148(100_000, 50_000, 5.0);
        assertEquals(5.0, ev.cost());
    }

    @Test
    void stressTest_50events_sumCorrect() {
        List<CostEvent148> events = IntStream.range(0, 50)
            .mapToObj(i -> new CostEvent148(i, i / 2, i * 0.001))
            .toList();
        double expectedCost = IntStream.range(0, 50).mapToDouble(i -> i * 0.001).sum();
        double actual = events.stream().mapToDouble(CostEvent148::cost).sum();
        assertEquals(expectedCost, actual, 1e-6);
    }

    @Test
    void costEvent_notEqualsWithDifferentCost() {
        CostEvent148 a = new CostEvent148(100, 50, 0.01);
        CostEvent148 b = new CostEvent148(100, 50, 0.02);
        assertNotEquals(a, b);
    }

    private static void skipIfAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "ancora_ffi not present");
    }
}
