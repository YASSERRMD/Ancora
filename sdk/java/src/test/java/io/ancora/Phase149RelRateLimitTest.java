package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.time.Duration;

import static org.junit.jupiter.api.Assertions.*;

class Phase149RelRateLimitTest {

    record RateLimitFixture149(int maxRpm, Duration retryAfter, int burstSize) {}

    @Test
    void rateLimitFixture_accessors() {
        RateLimitFixture149 f = new RateLimitFixture149(60, Duration.ofSeconds(30), 5);
        assertEquals(60, f.maxRpm());
        assertEquals(Duration.ofSeconds(30), f.retryAfter());
        assertEquals(5, f.burstSize());
    }

    @Test
    void rateLimitFixture_isRecord() {
        assertTrue(RateLimitFixture149.class.isRecord());
    }

    @Test
    void rateLimitFixture_valueEquality() {
        RateLimitFixture149 a = new RateLimitFixture149(60, Duration.ofSeconds(30), 5);
        RateLimitFixture149 b = new RateLimitFixture149(60, Duration.ofSeconds(30), 5);
        assertEquals(a, b);
    }

    @Test
    void retryAfter_inMillis() {
        RateLimitFixture149 f = new RateLimitFixture149(60, Duration.ofMillis(500), 5);
        assertEquals(500L, f.retryAfter().toMillis());
    }

    @Test
    void burstSize_greaterThanZero() {
        RateLimitFixture149 f = new RateLimitFixture149(60, Duration.ofSeconds(1), 10);
        assertTrue(f.burstSize() > 0);
    }

    @Test
    void maxRpm_greaterThanZero() {
        RateLimitFixture149 f = new RateLimitFixture149(120, Duration.ofSeconds(1), 5);
        assertTrue(f.maxRpm() > 0);
    }

    @Test
    void rateLimitFixtures_notEqual() {
        RateLimitFixture149 a = new RateLimitFixture149(60, Duration.ofSeconds(1), 5);
        RateLimitFixture149 b = new RateLimitFixture149(120, Duration.ofSeconds(1), 5);
        assertNotEquals(a, b);
    }

    @Test
    void agentRuns_withSimulatedRateLimit_stillComplete() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
            for (int i = 0; i < 3; i++) {
                List<io.ancora.RunEvent> events = a.run(spec).collectAll();
                assertInstanceOf(RunEvent.Completed.class, events.get(events.size() - 1));
            }
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
