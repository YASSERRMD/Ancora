package io.ancora;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.databind.DeserializationFeature;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.PropertyNamingStrategies;

final class Wire {
    private Wire() {}

    static final ObjectMapper MAPPER = new ObjectMapper()
        .setPropertyNamingStrategy(PropertyNamingStrategies.SNAKE_CASE)
        .setSerializationInclusion(JsonInclude.Include.NON_NULL)
        .configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false);

    static byte[] encodeAgentSpec(AgentSpec spec) throws Exception {
        return MAPPER.writeValueAsBytes(spec);
    }

    static byte[] encodeGraphSpec(GraphSpec spec) throws Exception {
        return MAPPER.writeValueAsBytes(spec);
    }

    static RunEvent parseEvent(String json) throws Exception {
        return MAPPER.readValue(json, RunEvent.class);
    }
}
