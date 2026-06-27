package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.Comparator;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class Phase148RagRetrievalMilvusTest {

    record MilvusChunk148(int id, String content, double score) {}

    static class MilvusTools148 {
        static final String TOOL_NAME = "milvus_retrieve148";

        private final List<MilvusChunk148> chunks = List.of(
            new MilvusChunk148(1, "The capital of France is Paris.", 0.98),
            new MilvusChunk148(2, "Berlin is the capital of Germany.", 0.91),
            new MilvusChunk148(3, "Tokyo is the capital of Japan.", 0.87),
            new MilvusChunk148(4, "Ottawa is the capital of Canada.", 0.82)
        );

        @Tool(description = "Retrieve chunks from Milvus vector store", name = "milvus_retrieve148")
        public String retrieve(@ToolInput(name = "query", description = "Search query") String query) {
            return chunks.stream()
                .filter(c -> c.score() > 0.85)
                .sorted(Comparator.comparingDouble(MilvusChunk148::score).reversed())
                .map(c -> c.content())
                .reduce("", (a, b) -> a + "\n" + b).strip();
        }
    }

    @Test
    void toolName_isMilvusRetrieve() {
        assertEquals("milvus_retrieve148", MilvusTools148.TOOL_NAME);
    }

    @Test
    void retrieve_returnsTopChunks() {
        MilvusTools148 tools = new MilvusTools148();
        String result = tools.retrieve("capital");
        assertTrue(result.contains("Paris"));
        assertTrue(result.contains("Berlin"));
        assertTrue(result.contains("Tokyo"));
    }

    @Test
    void retrieve_excludesLowScore() {
        MilvusTools148 tools = new MilvusTools148();
        String result = tools.retrieve("anything");
        assertFalse(result.contains("Ottawa"));
    }

    @Test
    void chunk_record_accessors() {
        MilvusChunk148 chunk = new MilvusChunk148(1, "content", 0.95);
        assertEquals(1, chunk.id());
        assertEquals("content", chunk.content());
        assertEquals(0.95, chunk.score());
    }

    @Test
    void chunk_valueEquality() {
        MilvusChunk148 a = new MilvusChunk148(1, "text", 0.9);
        MilvusChunk148 b = new MilvusChunk148(1, "text", 0.9);
        assertEquals(a, b);
    }

    @Test
    void tool_annotation_present() throws Exception {
        var method = MilvusTools148.class.getDeclaredMethod("retrieve", String.class);
        Tool t = method.getAnnotation(Tool.class);
        assertNotNull(t);
        assertEquals("milvus_retrieve148", t.name());
    }

    @Test
    void toolInput_annotation_present() throws Exception {
        var method = MilvusTools148.class.getDeclaredMethod("retrieve", String.class);
        ToolInput ti = method.getParameters()[0].getAnnotation(ToolInput.class);
        assertNotNull(ti);
        assertEquals("query", ti.name());
    }

    @Test
    void registerAll_findsRetrieve() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            MilvusTools148 tools = new MilvusTools148();
            List<ToolRegistration> regs = ToolRegistry.registerAll(rt, tools);
            assertEquals(1, regs.size());
            assertEquals("milvus_retrieve148", regs.get(0).spec().name());
            for (ToolRegistration reg : regs) reg.disposable().close();
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void result_isOrderedByScore() {
        MilvusTools148 tools = new MilvusTools148();
        String result = tools.retrieve("capital");
        int parisPc = result.indexOf("Paris");
        int berlinPc = result.indexOf("Berlin");
        assertTrue(parisPc < berlinPc);
    }

    @Test
    void result_notEmpty_forGenericQuery() {
        MilvusTools148 tools = new MilvusTools148();
        String result = tools.retrieve("world capitals");
        assertFalse(result.isEmpty());
    }

    private static void skipIfAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "ancora_ffi not present");
    }
}
