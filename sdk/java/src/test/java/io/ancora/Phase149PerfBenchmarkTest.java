package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class Phase149PerfBenchmarkTest {

    @Test
    void singleRun_wallTime_under30s() {
        skipIfAbsent();
        long start = System.nanoTime();
        try (Agent a = new Agent()) {
            a.run(new AgentSpec("llama3", null, null, null, null)).collectAll();
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
            return;
        } catch (Throwable t) {
            fail(t.toString());
            return;
        }
        long elapsed = System.nanoTime() - start;
        assertTrue(elapsed < 30_000_000_000L, "Single run exceeded 30s");
    }

    @Test
    void agentCreation_wallTime_under100ms() {
        skipIfAbsent();
        long start = System.nanoTime();
        try {
            Agent a = new Agent();
            a.close();
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
            return;
        } catch (Throwable t) {
            fail(t.toString());
            return;
        }
        long elapsed = System.nanoTime() - start;
        assertTrue(elapsed < 100_000_000L, "Agent creation exceeded 100ms");
    }

    @Test
    void runtimeCreation_wallTime_under200ms() {
        skipIfAbsent();
        long start = System.nanoTime();
        try {
            Runtime rt = new Runtime();
            rt.close();
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
            return;
        } catch (Throwable t) {
            fail(t.toString());
            return;
        }
        long elapsed = System.nanoTime() - start;
        assertTrue(elapsed < 200_000_000L, "Runtime creation exceeded 200ms");
    }

    @Test
    void tenRuns_totalTime_under5min() {
        skipIfAbsent();
        long start = System.nanoTime();
        try (Agent a = new Agent()) {
            AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
            for (int i = 0; i < 10; i++) a.run(spec).collectAll();
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
            return;
        } catch (Throwable t) {
            fail(t.toString());
            return;
        }
        long elapsed = System.nanoTime() - start;
        assertTrue(elapsed < 300_000_000_000L, "10 runs exceeded 5 minutes");
    }

    @Test
    void collectAll_vs_iterable_speedParity() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
            long t1 = System.nanoTime();
            a.run(spec).collectAll();
            long collectTime = System.nanoTime() - t1;

            long t2 = System.nanoTime();
            for (RunEvent ev : a.run(spec).events()) { /* drain */ }
            long iterTime = System.nanoTime() - t2;

            assertTrue(Math.abs(collectTime - iterTime) < 5_000_000_000L,
                "collectAll vs iterable time difference > 5s");
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void toolRegistration_wallTime_under50ms() {
        skipIfAbsent();
        long start = System.nanoTime();
        try (Runtime rt = new Runtime()) {
            ToolRegistration reg = ToolRegistry.register(rt, "perf_tool149", "Perf", input -> "{}");
            reg.disposable().close();
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
            return;
        } catch (Throwable t) {
            fail(t.toString());
            return;
        }
        long elapsed = System.nanoTime() - start;
        assertTrue(elapsed < 50_000_000L, "Tool registration exceeded 50ms");
    }

    @Test
    void fifty_runs_averageTime_reasonable() {
        skipIfAbsent();
        long start = System.nanoTime();
        try (Agent a = new Agent()) {
            AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
            for (int i = 0; i < 50; i++) a.run(spec).collectAll();
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
            return;
        } catch (Throwable t) {
            fail(t.toString());
            return;
        }
        long totalMs = (System.nanoTime() - start) / 1_000_000;
        assertTrue(totalMs < 600_000L, "50 runs exceeded 10 minutes total");
    }

    private static void skipIfAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "ancora_ffi not present");
    }
}
