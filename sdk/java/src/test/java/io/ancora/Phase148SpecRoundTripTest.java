package io.ancora;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import org.junit.jupiter.api.Test;

import java.nio.charset.StandardCharsets;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class Phase148SpecRoundTripTest {

    private static final ObjectMapper MAPPER = Wire.MAPPER;

    @Test
    void agentSpec_minimal_encodesToJson() throws Exception {
        AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
        byte[] bytes = Wire.encodeAgentSpec(spec);
        String json = new String(bytes, StandardCharsets.UTF_8);
        assertTrue(json.contains("llama3"));
    }

    @Test
    void agentSpec_model_preserved() throws Exception {
        AgentSpec spec = new AgentSpec("gpt-4o", null, null, null, null);
        byte[] bytes = Wire.encodeAgentSpec(spec);
        String json = new String(bytes, StandardCharsets.UTF_8);
        assertTrue(json.contains("gpt-4o"));
    }

    @Test
    void agentSpec_instructions_preserved() throws Exception {
        AgentSpec spec = new AgentSpec("llama3", "Be concise.", null, null, null);
        byte[] bytes = Wire.encodeAgentSpec(spec);
        String json = new String(bytes, StandardCharsets.UTF_8);
        assertTrue(json.contains("Be concise."));
    }

    @Test
    void agentSpec_toolSpec_name_preserved() throws Exception {
        ToolInputSchema schema = new ToolInputSchema("object", null, null);
        ToolSpec tool = new ToolSpec("my_tool", "Does things", schema);
        AgentSpec spec = new AgentSpec("llama3", null, List.of(tool), null, null);
        byte[] bytes = Wire.encodeAgentSpec(spec);
        String json = new String(bytes, StandardCharsets.UTF_8);
        assertTrue(json.contains("my_tool"));
    }

    @Test
    void agentSpec_maxTokens_encoded() throws Exception {
        AgentSpec spec = new AgentSpec("llama3", null, null, 512, null);
        byte[] bytes = Wire.encodeAgentSpec(spec);
        String json = new String(bytes, StandardCharsets.UTF_8);
        assertTrue(json.contains("512"));
    }

    @Test
    void agentSpec_temperature_encoded() throws Exception {
        AgentSpec spec = new AgentSpec("llama3", null, null, null, 0.7);
        byte[] bytes = Wire.encodeAgentSpec(spec);
        String json = new String(bytes, StandardCharsets.UTF_8);
        assertTrue(json.contains("0.7"));
    }

    @Test
    void agentSpec_nullTools_omitted() throws Exception {
        AgentSpec spec = new AgentSpec("llama3", null, null, null, null);
        byte[] bytes = Wire.encodeAgentSpec(spec);
        String json = new String(bytes, StandardCharsets.UTF_8);
        assertFalse(json.contains("\"tools\""));
    }

    @Test
    void agentSpec_usesSnakeCase() throws Exception {
        AgentSpec spec = new AgentSpec("llama3", null, null, 100, null);
        byte[] bytes = Wire.encodeAgentSpec(spec);
        String json = new String(bytes, StandardCharsets.UTF_8);
        assertTrue(json.contains("max_tokens"));
    }

    @Test
    void agentSpec_record_equality() {
        AgentSpec a = new AgentSpec("llama3", "sys", null, null, null);
        AgentSpec b = new AgentSpec("llama3", "sys", null, null, null);
        assertEquals(a, b);
    }

    @Test
    void agentSpec_record_isRecord() {
        assertTrue(AgentSpec.class.isRecord());
    }
}
