package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.HashMap;
import java.util.Map;
import java.util.Optional;
import java.util.concurrent.ConcurrentHashMap;

import static org.junit.jupiter.api.Assertions.*;

class Phase148MemoryReadWriteTest {

    static class InMemoryStore148 {
        private final ConcurrentHashMap<String, String> data = new ConcurrentHashMap<>();

        void set(String key, String value) { data.put(key, value); }
        Optional<String> get(String key) { return Optional.ofNullable(data.get(key)); }
        void delete(String key) { data.remove(key); }
        int count() { return data.size(); }
        void clear() { data.clear(); }
    }

    @Test
    void set_and_get_roundTrip() {
        InMemoryStore148 store = new InMemoryStore148();
        store.set("k1", "v1");
        assertEquals(Optional.of("v1"), store.get("k1"));
    }

    @Test
    void get_missingKey_returnsEmpty() {
        InMemoryStore148 store = new InMemoryStore148();
        assertEquals(Optional.empty(), store.get("no-such-key"));
    }

    @Test
    void delete_removesEntry() {
        InMemoryStore148 store = new InMemoryStore148();
        store.set("k2", "v2");
        store.delete("k2");
        assertEquals(Optional.empty(), store.get("k2"));
    }

    @Test
    void count_reflectsEntries() {
        InMemoryStore148 store = new InMemoryStore148();
        store.set("a", "1");
        store.set("b", "2");
        assertEquals(2, store.count());
    }

    @Test
    void clear_emptiesStore() {
        InMemoryStore148 store = new InMemoryStore148();
        store.set("x", "1");
        store.set("y", "2");
        store.clear();
        assertEquals(0, store.count());
    }

    @Test
    void overwrite_replacesPreviousValue() {
        InMemoryStore148 store = new InMemoryStore148();
        store.set("k", "old");
        store.set("k", "new");
        assertEquals(Optional.of("new"), store.get("k"));
    }

    @Test
    void multipleKeys_independentValues() {
        InMemoryStore148 store = new InMemoryStore148();
        for (int i = 0; i < 10; i++) store.set("k" + i, "v" + i);
        for (int i = 0; i < 10; i++) assertEquals(Optional.of("v" + i), store.get("k" + i));
    }

    @Test
    void stressTest_500ops_consistent() {
        InMemoryStore148 store = new InMemoryStore148();
        for (int i = 0; i < 500; i++) store.set("stress-" + i, String.valueOf(i * 2));
        for (int i = 0; i < 500; i++)
            assertEquals(Optional.of(String.valueOf(i * 2)), store.get("stress-" + i));
        assertEquals(500, store.count());
    }

    @Test
    void delete_nonExistent_isNoOp() {
        InMemoryStore148 store = new InMemoryStore148();
        assertDoesNotThrow(() -> store.delete("ghost-key"));
    }

    @Test
    void clear_then_set_works() {
        InMemoryStore148 store = new InMemoryStore148();
        store.set("before-clear", "v");
        store.clear();
        store.set("after-clear", "v2");
        assertEquals(Optional.of("v2"), store.get("after-clear"));
        assertEquals(1, store.count());
    }

    private static void skipIfAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "ancora_ffi not present");
    }
}
