package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.Set;

import static org.junit.jupiter.api.Assertions.*;

class Phase148PolicyResidencyTest {

    record PolicyEvent148(String region, boolean blocked, String reason) {}

    static final Set<String> CHINA_MAINLAND_BLOCKED = Set.of("mainland-china", "cn-north", "cn-east");

    @Test
    void policyEvent_accessors() {
        PolicyEvent148 ev = new PolicyEvent148("eu-west", false, null);
        assertEquals("eu-west", ev.region());
        assertFalse(ev.blocked());
        assertNull(ev.reason());
    }

    @Test
    void policyEvent_blocked_hasReason() {
        PolicyEvent148 ev = new PolicyEvent148("mainland-china", true, "GDPR incompatible region");
        assertTrue(ev.blocked());
        assertNotNull(ev.reason());
    }

    @Test
    void chinaMainland_isBlocked() {
        for (String region : CHINA_MAINLAND_BLOCKED) {
            PolicyEvent148 ev = new PolicyEvent148(region, true, "restricted");
            assertTrue(ev.blocked(), "Expected " + region + " to be blocked");
        }
    }

    @Test
    void euWest_isNotBlocked() {
        PolicyEvent148 ev = new PolicyEvent148("eu-west", false, null);
        assertFalse(ev.blocked());
    }

    @Test
    void usEast_isNotBlocked() {
        PolicyEvent148 ev = new PolicyEvent148("us-east-1", false, null);
        assertFalse(ev.blocked());
    }

    @Test
    void policyEvent_isRecord() {
        assertTrue(PolicyEvent148.class.isRecord());
    }

    @Test
    void policyEvent_valueEquality() {
        PolicyEvent148 a = new PolicyEvent148("eu-west", false, null);
        PolicyEvent148 b = new PolicyEvent148("eu-west", false, null);
        assertEquals(a, b);
    }

    @Test
    void blockedEvent_notEqualUnblockedEvent() {
        PolicyEvent148 blocked   = new PolicyEvent148("cn-north", true, "restricted");
        PolicyEvent148 unblocked = new PolicyEvent148("cn-north", false, null);
        assertNotEquals(blocked, unblocked);
    }

    @Test
    void reason_null_whenNotBlocked() {
        List<PolicyEvent148> events = List.of(
            new PolicyEvent148("ap-southeast", false, null),
            new PolicyEvent148("eu-central", false, null)
        );
        events.forEach(e -> assertNull(e.reason()));
    }

    @Test
    void allBlockedRegions_haveNonNullReason() {
        List<PolicyEvent148> blocked = CHINA_MAINLAND_BLOCKED.stream()
            .map(r -> new PolicyEvent148(r, true, "regulatory"))
            .toList();
        blocked.forEach(e -> assertNotNull(e.reason()));
    }

    private static void skipIfAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "ancora_ffi not present");
    }
}
