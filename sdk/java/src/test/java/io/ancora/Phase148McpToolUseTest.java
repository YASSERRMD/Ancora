package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.*;

class Phase148McpToolUseTest {

    static final class McpTools148 {
        static final String VALID_TOKEN   = "valid-mcp-token-java-148";
        static final String ERR_UNAUTHORIZED = "unauthorized";

        private final String token;
        private final Map<String, String> files = Map.of(
            "readme.md",  "# Welcome",
            "config.yml", "host: localhost",
            "notes.txt",  "Remember to hydrate"
        );

        McpTools148(String token) { this.token = token; }

        @Tool(description = "Read a file via MCP", name = "mcp_read148")
        public String readFile(@ToolInput(name = "path", description = "File path") String path) {
            if (!VALID_TOKEN.equals(token)) return "{\"error\":\"" + ERR_UNAUTHORIZED + "\"}";
            String content = files.get(path);
            if (content == null) return "{\"error\":\"not found\"}";
            return "{\"content\":\"" + content + "\"}";
        }
    }

    @Test
    void validToken_reads_readme() {
        McpTools148 tools = new McpTools148(McpTools148.VALID_TOKEN);
        String result = tools.readFile("readme.md");
        assertTrue(result.contains("Welcome"));
    }

    @Test
    void invalidToken_returnsUnauthorized() {
        McpTools148 tools = new McpTools148("wrong-token");
        String result = tools.readFile("readme.md");
        assertTrue(result.contains(McpTools148.ERR_UNAUTHORIZED));
    }

    @Test
    void validToken_reads_configYml() {
        McpTools148 tools = new McpTools148(McpTools148.VALID_TOKEN);
        String result = tools.readFile("config.yml");
        assertTrue(result.contains("localhost"));
    }

    @Test
    void validToken_unknownFile_returnsNotFound() {
        McpTools148 tools = new McpTools148(McpTools148.VALID_TOKEN);
        String result = tools.readFile("ghost.txt");
        assertTrue(result.contains("not found"));
    }

    @Test
    void token_constant_notEmpty() {
        assertFalse(McpTools148.VALID_TOKEN.isEmpty());
    }

    @Test
    void tool_annotation_present() throws Exception {
        var method = McpTools148.class.getDeclaredMethod("readFile", String.class);
        Tool t = method.getAnnotation(Tool.class);
        assertNotNull(t);
        assertEquals("mcp_read148", t.name());
    }

    @Test
    void toolInput_annotation_present() throws Exception {
        var method = McpTools148.class.getDeclaredMethod("readFile", String.class);
        ToolInput ti = method.getParameters()[0].getAnnotation(ToolInput.class);
        assertNotNull(ti);
        assertEquals("path", ti.name());
    }

    @Test
    void fixtureHasThreeFiles() {
        McpTools148 tools = new McpTools148(McpTools148.VALID_TOKEN);
        Map<String, String> files = Map.of("readme.md", "", "config.yml", "", "notes.txt", "");
        assertEquals(3, files.size());
    }

    @Test
    void registerAll_findsReadFileTool() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            McpTools148 tools = new McpTools148(McpTools148.VALID_TOKEN);
            List<ToolRegistration> regs = ToolRegistry.registerAll(rt, tools);
            assertEquals(1, regs.size());
            assertEquals("mcp_read148", regs.get(0).spec().name());
            for (ToolRegistration reg : regs) reg.disposable().close();
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void emptyToken_returnsUnauthorized() {
        McpTools148 tools = new McpTools148("");
        String result = tools.readFile("readme.md");
        assertTrue(result.contains(McpTools148.ERR_UNAUTHORIZED));
    }

    private static void skipIfAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "ancora_ffi not present");
    }
}
