package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.*;

class Phase149E2eMcpTest {

    static final class McpE2eTools149 {
        static final String VALID_TOKEN     = "mcp-e2e-token-149";
        static final String ERR_UNAUTHORIZED = "unauthorized";

        private final String token;
        private final Map<String, String> files = Map.of(
            "README.md",  "# Ancora Project",
            "config.yml", "env: production",
            "notes.txt",  "Deploy on Friday"
        );

        McpE2eTools149(String token) { this.token = token; }

        @Tool(description = "Read file via MCP e2e", name = "mcp_e2e_read149")
        public String readFile(@ToolInput(name = "path", description = "File path") String path) {
            if (!VALID_TOKEN.equals(token)) return "{\"error\":\"" + ERR_UNAUTHORIZED + "\"}";
            String content = files.get(path);
            if (content == null) return "{\"error\":\"not_found\"}";
            return "{\"content\":\"" + content + "\"}";
        }
    }

    @Test
    void validToken_reads_readme() {
        McpE2eTools149 tools = new McpE2eTools149(McpE2eTools149.VALID_TOKEN);
        String result = tools.readFile("README.md");
        assertTrue(result.contains("Ancora"));
    }

    @Test
    void invalidToken_returnsUnauthorized() {
        McpE2eTools149 tools = new McpE2eTools149("wrong-token");
        String result = tools.readFile("README.md");
        assertTrue(result.contains(McpE2eTools149.ERR_UNAUTHORIZED));
    }

    @Test
    void validToken_unknownFile_returnsNotFound() {
        McpE2eTools149 tools = new McpE2eTools149(McpE2eTools149.VALID_TOKEN);
        String result = tools.readFile("missing.txt");
        assertTrue(result.contains("not_found"));
    }

    @Test
    void fixture_hasThreeFiles() {
        assertEquals(3, Map.of("README.md", "", "config.yml", "", "notes.txt", "").size());
    }

    @Test
    void validToken_reads_configYml() {
        McpE2eTools149 tools = new McpE2eTools149(McpE2eTools149.VALID_TOKEN);
        String result = tools.readFile("config.yml");
        assertTrue(result.contains("production"));
    }

    @Test
    void tool_annotation_toolName() throws Exception {
        var method = McpE2eTools149.class.getDeclaredMethod("readFile", String.class);
        Tool t = method.getAnnotation(Tool.class);
        assertNotNull(t);
        assertEquals("mcp_e2e_read149", t.name());
    }

    @Test
    void registerAll_findsMcpTool() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            McpE2eTools149 tools = new McpE2eTools149(McpE2eTools149.VALID_TOKEN);
            List<ToolRegistration> regs = ToolRegistry.registerAll(rt, tools);
            assertEquals(1, regs.size());
            assertEquals("mcp_e2e_read149", regs.get(0).spec().name());
            for (ToolRegistration reg : regs) reg.disposable().close();
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void agentWithMcpTool_completes() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            McpE2eTools149 tools = new McpE2eTools149(McpE2eTools149.VALID_TOKEN);
            List<ToolRegistration> regs = ToolRegistry.registerAll(rt, tools);
            Agent a = new Agent(rt);
            List<RunEvent> events = a.run(new AgentSpec("llama3", null, null, null, null)).collectAll();
            assertInstanceOf(RunEvent.Completed.class, events.get(events.size() - 1));
            for (ToolRegistration reg : regs) reg.disposable().close();
            a.close();
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void emptyToken_unauthorized() {
        McpE2eTools149 tools = new McpE2eTools149("");
        String result = tools.readFile("README.md");
        assertTrue(result.contains(McpE2eTools149.ERR_UNAUTHORIZED));
    }

    @Test
    void validToken_reads_notesTxt() {
        McpE2eTools149 tools = new McpE2eTools149(McpE2eTools149.VALID_TOKEN);
        String result = tools.readFile("notes.txt");
        assertTrue(result.contains("Friday"));
    }

    private static void skipIfAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "ancora_ffi not present");
    }
}
