package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.Comparator;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class Phase149E2eRagMilvusTest {

    record Chunk149(int id, String content, double score) {}

    static final List<Chunk149> FIXTURE_CHUNKS = List.of(
        new Chunk149(1, "The Eiffel Tower is in Paris.", 0.97),
        new Chunk149(2, "Mount Fuji is in Japan.",       0.91),
        new Chunk149(3, "Big Ben is in London.",         0.88),
        new Chunk149(4, "The Colosseum is in Rome.",     0.83)
    );

    static class MilvusE2eTools149 {
        static final String TOOL_NAME = "milvus_e2e149";

        @Tool(description = "Retrieve from Milvus e2e fixture", name = "milvus_e2e149")
        public String retrieve(@ToolInput(name = "query", description = "Query") String query) {
            return FIXTURE_CHUNKS.stream()
                .filter(c -> c.score() > 0.85)
                .sorted(Comparator.comparingDouble(Chunk149::score).reversed())
                .map(Chunk149::content)
                .reduce("", (a, b) -> a + "\n" + b).strip();
        }
    }

    @Test
    void toolName_isCorrect() {
        assertEquals("milvus_e2e149", MilvusE2eTools149.TOOL_NAME);
    }

    @Test
    void retrieve_returnsHighScoreChunks() {
        MilvusE2eTools149 tools = new MilvusE2eTools149();
        String result = tools.retrieve("landmarks");
        assertTrue(result.contains("Eiffel"));
        assertTrue(result.contains("Fuji"));
        assertTrue(result.contains("Big Ben"));
    }

    @Test
    void retrieve_excludesLowScore() {
        MilvusE2eTools149 tools = new MilvusE2eTools149();
        String result = tools.retrieve("landmarks");
        assertFalse(result.contains("Colosseum"));
    }

    @Test
    void retrieve_orderByScoreDescending() {
        MilvusE2eTools149 tools = new MilvusE2eTools149();
        String result = tools.retrieve("landmarks");
        assertTrue(result.indexOf("Eiffel") < result.indexOf("Fuji"));
    }

    @Test
    void chunk_record_isRecord() {
        assertTrue(Chunk149.class.isRecord());
    }

    @Test
    void chunk_record_valueEquality() {
        Chunk149 a = new Chunk149(1, "content", 0.9);
        Chunk149 b = new Chunk149(1, "content", 0.9);
        assertEquals(a, b);
    }

    @Test
    void fixtureHasFourChunks() {
        assertEquals(4, FIXTURE_CHUNKS.size());
    }

    @Test
    void registerAll_discoversMilvusTool() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            MilvusE2eTools149 tools = new MilvusE2eTools149();
            List<ToolRegistration> regs = ToolRegistry.registerAll(rt, tools);
            assertEquals(1, regs.size());
            assertEquals("milvus_e2e149", regs.get(0).spec().name());
            for (ToolRegistration reg : regs) reg.disposable().close();
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void agentWithMilvusTool_completes() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            MilvusE2eTools149 tools = new MilvusE2eTools149();
            List<ToolRegistration> regs = ToolRegistry.registerAll(rt, tools);
            Agent a = new Agent(rt);
            List<RunEvent> events = a.run(new AgentSpec("llama3", "Use the milvus tool", null, null, null)).collectAll();
            assertInstanceOf(RunEvent.Completed.class, events.get(events.size() - 1));
            for (ToolRegistration reg : regs) reg.disposable().close();
            a.close();
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void retrieve_emptyQuery_returnsNonEmpty() {
        MilvusE2eTools149 tools = new MilvusE2eTools149();
        String result = tools.retrieve("");
        assertFalse(result.isEmpty());
    }

    private static void skipIfAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "ancora_ffi not present");
    }
}
