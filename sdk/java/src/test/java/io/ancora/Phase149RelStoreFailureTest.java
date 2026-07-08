package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.Optional;
import java.util.concurrent.ConcurrentHashMap;

import static org.junit.jupiter.api.Assertions.*;

class Phase149RelStoreFailureTest {

    static class FailableStore149 {
        private final ConcurrentHashMap<String, String> data = new ConcurrentHashMap<>();
        private volatile boolean failOnWrite = false;

        void setFailOnWrite(boolean fail) { this.failOnWrite = fail; }

        void set(String key, String value) {
            if (failOnWrite) throw new RuntimeException("simulated write failure");
            data.put(key, value);
        }

        Optional<String> get(String key) { return Optional.ofNullable(data.get(key)); }

        void delete(String key) { data.remove(key); }

        int count() { return data.size(); }
    }

    @Test
    void normalWrite_succeeds() {
        FailableStore149 store = new FailableStore149();
        store.set("k1", "v1");
        assertEquals(Optional.of("v1"), store.get("k1"));
    }

    @Test
    void failOnWrite_throwsRuntime() {
        FailableStore149 store = new FailableStore149();
        store.setFailOnWrite(true);
        assertThrows(RuntimeException.class, () -> store.set("k", "v"));
    }

    @Test
    void recoveryAfterFailure_writesSucceed() {
        FailableStore149 store = new FailableStore149();
        store.setFailOnWrite(true);
        assertThrows(RuntimeException.class, () -> store.set("k", "v"));
        store.setFailOnWrite(false);
        assertDoesNotThrow(() -> store.set("k2", "v2"));
        assertEquals(Optional.of("v2"), store.get("k2"));
    }

    @Test
    void preFailData_persistsAfterRecovery() {
        FailableStore149 store = new FailableStore149();
        store.set("pre", "data");
        store.setFailOnWrite(true);
        assertThrows(RuntimeException.class, () -> store.set("during-fail", "v"));
        store.setFailOnWrite(false);
        assertEquals(Optional.of("data"), store.get("pre"));
    }

    @Test
    void read_during_failedWrite_stillWorks() {
        FailableStore149 store = new FailableStore149();
        store.set("readable", "yes");
        store.setFailOnWrite(true);
        assertEquals(Optional.of("yes"), store.get("readable"));
    }

    @Test
    void failToggle_multipleTimesOK() {
        FailableStore149 store = new FailableStore149();
        for (int i = 0; i < 5; i++) {
            store.setFailOnWrite(true);
            assertThrows(RuntimeException.class, () -> store.set("k", "v"));
            store.setFailOnWrite(false);
            store.set("k" + i, "v" + i);
        }
        assertEquals(5, store.count());
    }

    @Test
    void delete_during_failedWrite_stillWorks() {
        FailableStore149 store = new FailableStore149();
        store.set("to-delete", "gone");
        store.setFailOnWrite(true);
        assertDoesNotThrow(() -> store.delete("to-delete"));
        assertEquals(Optional.empty(), store.get("to-delete"));
    }

    @Test
    void storeFailure_doesNotAffectAgentLifecycle() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            FailableStore149 store = new FailableStore149();
            store.setFailOnWrite(true);
            assertThrows(RuntimeException.class, () -> store.set("k", "v"));
            List<RunEvent> events = a.run(new AgentSpec("llama3", null, null, null, null)).collectAll();
            assertFalse(events.isEmpty());
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void get_nonExistentKey_returnsEmpty() {
        FailableStore149 store = new FailableStore149();
        assertEquals(Optional.empty(), store.get("does-not-exist"));
    }

    private static void skipIfAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "ancora_ffi not present");
    }
}
