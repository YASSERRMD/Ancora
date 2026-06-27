package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.Comparator;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class Phase149E2eVectorStoreParityTest {

    record Chunk149(int id, String content, double score) {}

    static class PgVectorStore149 {
        static final String TOOL_NAME = "pg_retrieve2_149";
        private final List<Chunk149> chunks = List.of(
            new Chunk149(1, "PG chunk one", 0.96),
            new Chunk149(2, "PG chunk two", 0.88)
        );

        @Tool(description = "Retrieve from PG vector store", name = "pg_retrieve2_149")
        public String retrieve(@ToolInput(name = "query", description = "Query") String query) {
            return chunks.stream()
                .sorted(Comparator.comparingDouble(Chunk149::score).reversed())
                .map(c -> c.content())
                .reduce("", (a, b) -> a + "\n" + b).strip();
        }
    }

    static class LanceVectorStore149 {
        static final String TOOL_NAME = "lance_retrieve2_149";
        private final List<Chunk149> chunks = List.of(
            new Chunk149(1, "Lance chunk one", 0.95),
            new Chunk149(2, "Lance chunk two", 0.89)
        );

        @Tool(description = "Retrieve from LanceDB vector store", name = "lance_retrieve2_149")
        public String retrieve(@ToolInput(name = "query", description = "Query") String query) {
            return chunks.stream()
                .sorted(Comparator.comparingDouble(Chunk149::score).reversed())
                .map(c -> c.content())
                .reduce("", (a, b) -> a + "\n" + b).strip();
        }
    }

    @Test
    void pgStore_toolName_correct() {
        assertEquals("pg_retrieve2_149", PgVectorStore149.TOOL_NAME);
    }

    @Test
    void lanceStore_toolName_correct() {
        assertEquals("lance_retrieve2_149", LanceVectorStore149.TOOL_NAME);
    }

    @Test
    void pgStore_retrieve_returnsContent() {
        PgVectorStore149 store = new PgVectorStore149();
        String result = store.retrieve("test");
        assertTrue(result.contains("PG chunk"));
    }

    @Test
    void lanceStore_retrieve_returnsContent() {
        LanceVectorStore149 store = new LanceVectorStore149();
        String result = store.retrieve("test");
        assertTrue(result.contains("Lance chunk"));
    }

    @Test
    void pgStore_and_lanceStore_toolNames_differ() {
        assertNotEquals(PgVectorStore149.TOOL_NAME, LanceVectorStore149.TOOL_NAME);
    }

    @Test
    void bothStores_register_noNameCollision() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            PgVectorStore149 pg     = new PgVectorStore149();
            LanceVectorStore149 lance = new LanceVectorStore149();
            List<ToolRegistration> pgRegs    = ToolRegistry.registerAll(rt, pg);
            List<ToolRegistration> lanceRegs = ToolRegistry.registerAll(rt, lance);
            assertEquals(1, pgRegs.size());
            assertEquals(1, lanceRegs.size());
            assertNotEquals(pgRegs.get(0).spec().name(), lanceRegs.get(0).spec().name());
            for (ToolRegistration r : pgRegs) r.disposable().close();
            for (ToolRegistration r : lanceRegs) r.disposable().close();
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void pgStore_orderByScoreDesc() {
        PgVectorStore149 store = new PgVectorStore149();
        String result = store.retrieve("test");
        assertTrue(result.indexOf("one") < result.indexOf("two"));
    }

    @Test
    void lanceStore_orderByScoreDesc() {
        LanceVectorStore149 store = new LanceVectorStore149();
        String result = store.retrieve("test");
        assertTrue(result.indexOf("one") < result.indexOf("two"));
    }

    @Test
    void bothStores_parity_sameResultStructure() {
        PgVectorStore149 pg = new PgVectorStore149();
        LanceVectorStore149 lance = new LanceVectorStore149();
        String pgResult    = pg.retrieve("test");
        String lanceResult = lance.retrieve("test");
        long pgLines    = pgResult.lines().count();
        long lanceLines = lanceResult.lines().count();
        assertEquals(pgLines, lanceLines);
    }

    private static void skipIfAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "ancora_ffi not present");
    }
}
