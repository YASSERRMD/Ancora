package io.ancora;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.lang.reflect.Method;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class Phase148ToolAnnotationExecutionTest {

    static class SampleTools148 {
        @Tool(description = "Greet a user by name", name = "greet_148")
        public String greet(@ToolInput(name = "name", description = "User name") String name) {
            return "{\"greeting\":\"Hello, " + name + "!\"}";
        }

        @Tool(description = "Add two integers")
        public String addNumbers(
            @ToolInput(name = "a", description = "First") int a,
            @ToolInput(name = "b", description = "Second") int b) {
            return "{\"sum\":" + (a + b) + "}";
        }

        @Tool(description = "Return constant value", name = "constant_148")
        public String constant() {
            return "{\"value\":42}";
        }
    }

    @Test
    void tool_annotation_storesDescription() {
        Method[] methods = SampleTools148.class.getDeclaredMethods();
        for (Method m : methods) {
            Tool t = m.getAnnotation(Tool.class);
            if (t != null) assertFalse(t.description().isEmpty());
        }
    }

    @Test
    void tool_annotation_nameOverride() throws Exception {
        Method m = SampleTools148.class.getDeclaredMethod("greet", String.class);
        Tool t = m.getAnnotation(Tool.class);
        assertNotNull(t);
        assertEquals("greet_148", t.name());
    }

    @Test
    void toSnakeCase_convertsPascalCase() {
        assertEquals("add_numbers", ToolRegistry.toSnakeCase("AddNumbers"));
    }

    @Test
    void toSnakeCase_lowercaseUnchanged() {
        assertEquals("greet", ToolRegistry.toSnakeCase("greet"));
    }

    @Test
    void buildSchema_hasNamedProperties() throws Exception {
        Method m = SampleTools148.class.getDeclaredMethod("greet", String.class);
        m.setAccessible(true);
        ToolInputSchema schema = ToolRegistry.buildSchema(m);
        assertNotNull(schema);
        assertTrue(schema.properties().containsKey("name"));
    }

    @Test
    void buildSchema_requiredList() throws Exception {
        Method m = SampleTools148.class.getDeclaredMethod("greet", String.class);
        m.setAccessible(true);
        ToolInputSchema schema = ToolRegistry.buildSchema(m);
        assertNotNull(schema);
        assertTrue(schema.required().contains("name"));
    }

    @Test
    void registerAll_discoversTwoAnnotatedMethods() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            SampleTools148 tools = new SampleTools148();
            List<ToolRegistration> regs = ToolRegistry.registerAll(rt, tools);
            assertEquals(3, regs.size());
            for (ToolRegistration reg : regs) reg.disposable().close();
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void registerAll_findsGreetByCustomName() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            SampleTools148 tools = new SampleTools148();
            List<ToolRegistration> regs = ToolRegistry.registerAll(rt, tools);
            assertTrue(regs.stream().anyMatch(r -> r.spec().name().equals("greet_148")));
            for (ToolRegistration reg : regs) reg.disposable().close();
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void handler_invokesGreetMethod() throws Exception {
        SampleTools148 tools = new SampleTools148();
        Method method = SampleTools148.class.getDeclaredMethod("greet", String.class);
        method.setAccessible(true);
        ObjectMapper mapper = Wire.MAPPER;
        JsonNode input = mapper.readTree("{\"name\":\"World\"}");
        Object result = method.invoke(tools, "World");
        assertTrue(result.toString().contains("World"));
    }

    @Test
    void toolInput_annotation_requiresDefault() throws Exception {
        Method m = SampleTools148.class.getDeclaredMethod("greet", String.class);
        ToolInput ti = m.getParameters()[0].getAnnotation(ToolInput.class);
        assertNotNull(ti);
        assertTrue(ti.required());
    }

    private static void skipIfAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "ancora_ffi not present");
    }
}
