package io.ancora;

import io.ancora.ffi.AncoraNative;

import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.lang.reflect.Method;
import java.lang.reflect.Parameter;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;

public final class ToolRegistry {
    private ToolRegistry() {}

    public static ToolRegistration register(
        Runtime runtime, String name, String description, ToolHandler handler) throws Throwable {
        ToolBridge bridge = ToolBridge.create(handler);
        try (Arena scratch = Arena.ofConfined()) {
            MemorySegment nameSeg = scratch.allocateFrom(name);
            int rc = (int) AncoraNative.TOOL_REGISTER.invokeExact(
                runtime.rawPtr(), nameSeg, bridge.stub());
            if (rc != 0) {
                bridge.close();
                throw new AncorException(rc, "ancora_tool_register failed for: " + name);
            }
        }
        ToolSpec spec = new ToolSpec(name, description, null);
        ToolDisposable disposable = new ToolDisposable(runtime, name, bridge);
        return new ToolRegistration(spec, disposable);
    }

    public static List<ToolRegistration> registerAll(Runtime runtime, Object target) throws Throwable {
        List<ToolRegistration> results = new ArrayList<>();
        for (Method method : target.getClass().getDeclaredMethods()) {
            io.ancora.Tool annotation = method.getAnnotation(io.ancora.Tool.class);
            if (annotation == null) continue;
            method.setAccessible(true);
            String toolName = annotation.name().isEmpty()
                ? toSnakeCase(method.getName())
                : annotation.name();
            ToolInputSchema schema = buildSchema(method);
            ToolHandler handler = input -> {
                Object[] args = extractArgs(method, input);
                Object r = method.invoke(target, args);
                return r != null ? r.toString() : "";
            };
            ToolBridge bridge = ToolBridge.create(handler);
            try (Arena scratch = Arena.ofConfined()) {
                MemorySegment nameSeg = scratch.allocateFrom(toolName);
                int rc = (int) AncoraNative.TOOL_REGISTER.invokeExact(
                    runtime.rawPtr(), nameSeg, bridge.stub());
                if (rc != 0) {
                    bridge.close();
                    throw new AncorException(rc, "ancora_tool_register failed for: " + toolName);
                }
            }
            ToolSpec spec = new ToolSpec(toolName, annotation.description(), schema);
            ToolDisposable disposable = new ToolDisposable(runtime, toolName, bridge);
            results.add(new ToolRegistration(spec, disposable));
        }
        return results;
    }

    static ToolInputSchema buildSchema(Method method) {
        Map<String, ToolInputProperty> props = new LinkedHashMap<>();
        List<String> required = new ArrayList<>();
        for (Parameter param : method.getParameters()) {
            ToolInput ti = param.getAnnotation(ToolInput.class);
            if (ti == null) continue;
            String paramName = ti.name().isEmpty() ? param.getName() : ti.name();
            String desc = ti.description().isEmpty() ? null : ti.description();
            props.put(paramName, new ToolInputProperty(javaTypeToJsonType(param.getType()), desc));
            if (ti.required()) required.add(paramName);
        }
        if (props.isEmpty()) return null;
        return new ToolInputSchema("object",
            props.isEmpty() ? null : props,
            required.isEmpty() ? null : required);
    }

    private static Object[] extractArgs(Method method, com.fasterxml.jackson.databind.JsonNode input)
        throws Exception {
        Parameter[] params = method.getParameters();
        Object[] args = new Object[params.length];
        for (int i = 0; i < params.length; i++) {
            ToolInput ti = params[i].getAnnotation(ToolInput.class);
            String paramName = (ti != null && !ti.name().isEmpty()) ? ti.name() : params[i].getName();
            if (input != null && input.has(paramName)) {
                args[i] = Wire.MAPPER.treeToValue(input.get(paramName), params[i].getType());
            }
        }
        return args;
    }

    private static String javaTypeToJsonType(Class<?> type) {
        if (type == String.class) return "string";
        if (type == int.class || type == Integer.class
            || type == long.class || type == Long.class
            || type == double.class || type == Double.class
            || type == float.class || type == Float.class) return "number";
        if (type == boolean.class || type == Boolean.class) return "boolean";
        return "object";
    }

    static String toSnakeCase(String name) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < name.length(); i++) {
            char c = name.charAt(i);
            if (Character.isUpperCase(c) && i > 0) sb.append('_');
            sb.append(Character.toLowerCase(c));
        }
        return sb.toString();
    }
}
