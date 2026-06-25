package io.ancora.examples;

import io.ancora.*;
import java.util.List;

/**
 * Demonstrates registering annotation-based tool callbacks and running an agent
 * that can invoke them. The same pattern works with MCP-backed tools.
 */
public class McpToolUse {
    public static void main(String[] args) throws Throwable {
        try (Runtime runtime = new Runtime()) {
            List<ToolRegistration> tools = ToolRegistry.registerAll(runtime, new WeatherTools());
            try {
                List<ToolSpec> toolSpecs = tools.stream().map(ToolRegistration::spec).toList();

                AgentSpec spec = new AgentSpec(
                    "claude-3-5-haiku-20241022",
                    "Use the get_weather tool to answer questions about weather.",
                    toolSpecs, null, null);

                RunHandle handle = new Agent(runtime).run(spec);
                System.out.println("Run ID: " + handle.runId());

                for (RunEvent ev : handle.events()) {
                    switch (ev) {
                        case RunEvent.Started s ->
                            System.out.println("[started]");
                        case RunEvent.Token t ->
                            System.out.print(t.text());
                        case RunEvent.ToolCall tc ->
                            System.out.println("\n[tool_call] " + tc.name() + " <- " + tc.input());
                        case RunEvent.Completed c ->
                            System.out.println("\n[completed]");
                        default -> {}
                    }
                }

                System.out.println("\nCost: " + handle.getCost());
            } finally {
                for (ToolRegistration r : tools) r.close();
            }
        }
    }

    static class WeatherTools {
        @Tool(description = "Get current weather for a city")
        public String getWeather(@ToolInput(description = "city name") String city) {
            return "{\"city\":\"" + city + "\",\"weather\":\"sunny\",\"temp_c\":22}";
        }

        @Tool(description = "Convert Celsius to Fahrenheit")
        public String celsiusToFahrenheit(
            @ToolInput(description = "temperature in Celsius") double celsius) {
            double fahrenheit = celsius * 9.0 / 5.0 + 32;
            return "{\"celsius\":" + celsius + ",\"fahrenheit\":" + fahrenheit + "}";
        }
    }
}
