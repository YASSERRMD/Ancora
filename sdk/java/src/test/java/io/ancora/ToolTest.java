package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.reflect.Method;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class ToolTest {

    // --- annotation retention tests (no native lib) ---

    @Test
    void tool_annotation_has_runtime_retention() throws Exception {
        Retention r = Tool.class.getAnnotation(Retention.class);
        assertNotNull(r);
        assertEquals(RetentionPolicy.RUNTIME, r.value());
    }

    @Test
    void toolInput_annotation_has_runtime_retention() throws Exception {
        Retention r = ToolInput.class.getAnnotation(Retention.class);
        assertNotNull(r);
        assertEquals(RetentionPolicy.RUNTIME, r.value());
    }

    @Test
    void toolHandler_is_functional_interface() {
        assertTrue(ToolHandler.class.isAnnotationPresent(FunctionalInterface.class));
    }

    @Test
    void toolRegistration_implements_auto_closeable() {
        assertTrue(AutoCloseable.class.isAssignableFrom(ToolRegistration.class));
    }

    // --- ToolRegistry unit tests (no native lib) ---

    @Test
    void toolRegistry_to_snake_case_converts_camel() {
        assertEquals("get_weather", ToolRegistry.toSnakeCase("getWeather"));
        assertEquals("my_long_method", ToolRegistry.toSnakeCase("myLongMethod"));
        assertEquals("version", ToolRegistry.toSnakeCase("version"));
    }

    @Test
    void toolRegistry_build_schema_reflects_annotated_params() throws Exception {
        class Holder {
            @Tool(description = "a tool")
            public String greet(@ToolInput(description = "the name") String name,
                                @ToolInput(required = false) int count) {
                return name.repeat(count);
            }
        }
        Method m = Holder.class.getMethod("greet", String.class, int.class);
        ToolInputSchema schema = ToolRegistry.buildSchema(m);
        assertNotNull(schema);
        assertEquals("object", schema.type());
        assertTrue(schema.properties().containsKey("name"), "should have 'name' property");
        assertTrue(schema.properties().containsKey("count"), "should have 'count' property");
        assertEquals("string", schema.properties().get("name").type());
        assertEquals("number", schema.properties().get("count").type());
        assertTrue(schema.required().contains("name"), "name should be required");
        assertFalse(schema.required().contains("count"), "count should not be required");
    }

    @Test
    void toolRegistry_build_schema_with_custom_name() throws Exception {
        class Holder {
            @Tool(description = "lookup")
            public String lookup(@ToolInput(name = "city", description = "city name") String c) {
                return c;
            }
        }
        Method m = Holder.class.getMethod("lookup", String.class);
        ToolInputSchema schema = ToolRegistry.buildSchema(m);
        assertNotNull(schema);
        assertTrue(schema.properties().containsKey("city"),
            "should use @ToolInput name='city' not parameter name");
    }

    @Test
    void toolRegistry_build_schema_no_annotations_returns_null() throws Exception {
        class Holder {
            @Tool(description = "no params")
            public String noParams() { return "ok"; }
        }
        Method m = Holder.class.getMethod("noParams");
        ToolInputSchema schema = ToolRegistry.buildSchema(m);
        assertNull(schema, "method with no @ToolInput params should yield null schema");
    }

    @Test
    void toolRegistration_spec_accessible() throws Throwable {
        skipIfNativeLibraryAbsent();
        try (Runtime rt = new Runtime()) {
            ToolRegistration reg = ToolRegistry.register(rt, "echo", "returns input",
                input -> input != null ? input.toString() : "");
            try {
                assertNotNull(reg.spec());
                assertEquals("echo", reg.spec().name());
            } finally {
                reg.close();
            }
        } catch (UnsatisfiedLinkError e) {
            skipIfNativeLibraryAbsent();
        } catch (Throwable t) {
            fail("Unexpected exception: " + t);
        }
    }

    // --- integration tests (require native library) ---

    @Test
    void toolRegistry_register_increments_tool_count() throws Throwable {
        skipIfNativeLibraryAbsent();
        try (Runtime rt = new Runtime()) {
            assertEquals(0L, rt.toolCount());
            ToolRegistration reg = ToolRegistry.register(rt, "adder", "adds numbers",
                input -> String.valueOf(
                    input.path("a").asInt(0) + input.path("b").asInt(0)));
            try {
                assertEquals(1L, rt.toolCount());
            } finally {
                reg.close();
            }
            assertEquals(0L, rt.toolCount(), "should decrement after unregister");
        } catch (UnsatisfiedLinkError e) {
            skipIfNativeLibraryAbsent();
        } catch (Throwable t) {
            fail("Unexpected exception: " + t);
        }
    }

    @Test
    void toolRegistry_tool_exists_after_register() throws Throwable {
        skipIfNativeLibraryAbsent();
        try (Runtime rt = new Runtime()) {
            assertFalse(rt.toolExists("greeter"));
            ToolRegistration reg = ToolRegistry.register(rt, "greeter", "greets",
                input -> "Hello " + input.path("name").asText("World"));
            try {
                assertTrue(rt.toolExists("greeter"));
            } finally {
                reg.close();
            }
            assertFalse(rt.toolExists("greeter"), "should be gone after close");
        } catch (UnsatisfiedLinkError e) {
            skipIfNativeLibraryAbsent();
        } catch (Throwable t) {
            fail("Unexpected exception: " + t);
        }
    }

    @Test
    void toolRegistry_register_all_from_object() throws Throwable {
        skipIfNativeLibraryAbsent();
        class MyTools {
            @Tool(description = "gets the weather")
            public String getWeather(@ToolInput(description = "city") String city) {
                return "Sunny in " + city;
            }
        }
        try (Runtime rt = new Runtime()) {
            List<ToolRegistration> regs = ToolRegistry.registerAll(rt, new MyTools());
            try {
                assertEquals(1, regs.size());
                assertEquals("get_weather", regs.get(0).spec().name());
                assertTrue(rt.toolExists("get_weather"));
            } finally {
                for (ToolRegistration r : regs) r.close();
            }
        } catch (UnsatisfiedLinkError e) {
            skipIfNativeLibraryAbsent();
        } catch (Throwable t) {
            fail("Unexpected exception: " + t);
        }
    }

    @Test
    void tool_runs_in_a_run() throws Throwable {
        skipIfNativeLibraryAbsent();
        class WeatherTools {
            @Tool(description = "get weather for a city")
            public String getWeather(@ToolInput(description = "city name") String city) {
                return "{\"weather\": \"sunny\", \"city\": \"" + city + "\"}";
            }
        }
        try (Runtime rt = new Runtime()) {
            List<ToolRegistration> regs = ToolRegistry.registerAll(rt, new WeatherTools());
            try {
                List<ToolSpec> toolSpecs = regs.stream().map(ToolRegistration::spec).toList();
                AgentSpec spec = new AgentSpec(
                    "claude-3-5-haiku-20241022",
                    "Use the get_weather tool.",
                    toolSpecs, null, null);
                List<RunEvent> events = new Agent(rt).run(spec).collectAll();
                assertFalse(events.isEmpty());
                assertInstanceOf(RunEvent.Started.class, events.get(0));
                assertInstanceOf(RunEvent.Completed.class, events.get(events.size() - 1));
            } finally {
                for (ToolRegistration r : regs) r.close();
            }
        } catch (UnsatisfiedLinkError e) {
            skipIfNativeLibraryAbsent();
        } catch (Throwable t) {
            fail("Unexpected exception: " + t);
        }
    }

    private static void skipIfNativeLibraryAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE,
            "ancora_ffi native library not present; CI provides it.");
    }
}
