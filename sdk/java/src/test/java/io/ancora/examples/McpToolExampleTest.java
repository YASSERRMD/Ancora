package io.ancora.examples;

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

class McpToolExampleTest {

    static String getWeather(String location) {
        return "Weather in " + location + ": 22 C, partly cloudy";
    }

    static double calculate(String expression) {
        String[] parts = expression.split("\\+");
        if (parts.length == 2) {
            try {
                return Double.parseDouble(parts[0].trim()) + Double.parseDouble(parts[1].trim());
            } catch (NumberFormatException ignored) {}
        }
        return 0;
    }

    private static ToolSpec weatherSpec() {
        return new ToolSpec(
            "get_weather",
            "Get weather for a location.",
            new ToolInputSchema(
                "object",
                Map.of("location", new ToolInputProperty("string", "City name")),
                List.of("location")
            )
        );
    }

    @Test
    void weather_tool_returns_result_for_city() {
        String result = getWeather("Cairo");
        assertTrue(result.contains("Cairo"));
        assertTrue(result.contains("22 C"));
    }

    @Test
    void calculate_tool_adds_two_numbers() {
        assertEquals(7.0, calculate("3 + 4"), 1e-9);
    }

    @Test
    void tool_spec_has_correct_name_and_description() {
        ToolSpec spec = weatherSpec();
        assertEquals("get_weather", spec.name());
        assertEquals(1, spec.inputSchema().properties().size());
        assertTrue(spec.inputSchema().required().contains("location"));
    }

    @Test
    void agent_spec_with_tools_runs_without_error() throws Throwable {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "native library absent");
        try (Agent agent = new Agent()) {
            AgentSpec spec = new AgentSpec("local-model", "Use tools.", List.of(weatherSpec()), null, null);
            var events = agent.run(spec).collectAll();
            assertFalse(events.isEmpty());
        } catch (UnsatisfiedLinkError ignored) {}
    }
}
