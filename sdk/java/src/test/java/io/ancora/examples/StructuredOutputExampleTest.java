package io.ancora.examples;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.ObjectMapper;
import io.ancora.Agent;
import io.ancora.AgentSpec;
import io.ancora.ToolInputProperty;
import io.ancora.ToolInputSchema;
import io.ancora.ToolSpec;
import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.*;

class StructuredOutputExampleTest {

    record AnalysisResult(
        @JsonProperty("summary")   String summary,
        @JsonProperty("sentiment") String sentiment,
        @JsonProperty("score")     double score
    ) {}

    private static ToolInputSchema buildAnalysisSchema() {
        return new ToolInputSchema(
            "object",
            Map.of(
                "summary",   new ToolInputProperty("string", "Brief summary"),
                "sentiment", new ToolInputProperty("string", "positive, neutral, or negative"),
                "score",     new ToolInputProperty("number", "Confidence 0-1")
            ),
            List.of("summary", "sentiment", "score")
        );
    }

    @Test
    void schema_has_expected_type() {
        ToolInputSchema schema = buildAnalysisSchema();
        assertEquals("object", schema.type());
    }

    @Test
    void schema_has_three_properties() {
        ToolInputSchema schema = buildAnalysisSchema();
        assertNotNull(schema.properties());
        assertEquals(3, schema.properties().size());
    }

    @Test
    void schema_required_fields_include_all_three() {
        ToolInputSchema schema = buildAnalysisSchema();
        assertNotNull(schema.required());
        assertTrue(schema.required().containsAll(List.of("summary", "sentiment", "score")));
    }

    @Test
    void json_round_trip_preserves_field_names() throws Exception {
        ObjectMapper mapper = new ObjectMapper();
        AnalysisResult result = new AnalysisResult("All good", "positive", 0.9);
        String json = mapper.writeValueAsString(result);
        assertTrue(json.contains("\"summary\""));
        assertTrue(json.contains("\"sentiment\""));
        assertTrue(json.contains("\"score\""));
    }

    @Test
    void agent_with_output_schema_runs_without_error() throws Throwable {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "native library absent");
        try (Agent agent = new Agent()) {
            ToolSpec outputSpec = new ToolSpec("output", "Structured output.", buildAnalysisSchema());
            AgentSpec spec = new AgentSpec("local-model", "Analyze text.", List.of(outputSpec), null, null);
            var events = agent.run(spec).collectAll();
            assertFalse(events.isEmpty());
        } catch (UnsatisfiedLinkError ignored) {}
    }
}
