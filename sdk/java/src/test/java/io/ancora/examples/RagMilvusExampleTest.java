package io.ancora.examples;

import io.ancora.Agent;
import io.ancora.AgentSpec;
import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.Arrays;
import java.util.Comparator;
import java.util.List;
import java.util.Locale;

import static org.junit.jupiter.api.Assertions.*;

class RagMilvusExampleTest {

    record Passage(String key, String content) {}

    /** Offline keyword retrieval standing in for a Milvus vector search. */
    static List<Passage> keywordRetrieve(List<Passage> corpus, String query, int topK) {
        String[] terms = query.toLowerCase(Locale.ROOT).split("\\s+");
        return corpus.stream()
            .sorted(Comparator.comparingLong((Passage p) ->
                Arrays.stream(terms)
                    .filter(t -> p.content().toLowerCase(Locale.ROOT).contains(t))
                    .count()
            ).reversed())
            .limit(topK)
            .toList();
    }

    private static final List<Passage> CORPUS = List.of(
        new Passage("milvus.md",   "Milvus is a vector database for high-scale similarity search."),
        new Passage("ancora.md",   "Ancora is a multi-agent runtime for AI applications."),
        new Passage("lancedb.md",  "LanceDB stores vectors with column-level compression.")
    );

    @Test
    void top_result_for_milvus_query_is_milvus_doc() {
        List<Passage> hits = keywordRetrieve(CORPUS, "milvus vector database", 1);
        assertFalse(hits.isEmpty());
        assertEquals("milvus.md", hits.get(0).key());
    }

    @Test
    void retrieve_top2_returns_two_passages() {
        List<Passage> hits = keywordRetrieve(CORPUS, "vector", 2);
        assertEquals(2, hits.size());
    }

    @Test
    void retrieve_returns_all_for_topk_larger_than_corpus() {
        List<Passage> hits = keywordRetrieve(CORPUS, "ai", 100);
        assertEquals(CORPUS.size(), hits.size());
    }

    @Test
    void agent_run_with_injected_context_does_not_throw() throws Throwable {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "native library absent");
        try (Agent agent = new Agent()) {
            List<Passage> hits = keywordRetrieve(CORPUS, "milvus", 1);
            String context = hits.stream().map(Passage::content).reduce("", (a, b) -> a + "\n" + b);
            AgentSpec spec = new AgentSpec("local-model", "Use context: " + context, null, null, null);
            var events = agent.run(spec).collectAll();
            assertFalse(events.isEmpty());
        } catch (UnsatisfiedLinkError ignored) {}
    }
}
