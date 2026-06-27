package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class Phase149SecMcpAuthTest {

    static final class SecureMcpTools149 {
        static final String VALID_TOKEN     = "valid-sec-token-java-149";
        static final String ERR_UNAUTHORIZED = "unauthorized";
        static final String ERR_FORBIDDEN    = "forbidden";

        private final String token;

        SecureMcpTools149(String token) { this.token = token; }

        @Tool(description = "Secure read operation", name = "secure_read149")
        public String secureRead(@ToolInput(name = "resource", description = "Resource ID") String resource) {
            if (token == null || token.isBlank()) return "{\"error\":\"" + ERR_UNAUTHORIZED + "\"}";
            if (!VALID_TOKEN.equals(token))        return "{\"error\":\"" + ERR_FORBIDDEN + "\"}";
            return "{\"data\":\"resource-" + resource + "\"}";
        }
    }

    @Test
    void validToken_returnsData() {
        SecureMcpTools149 tools = new SecureMcpTools149(SecureMcpTools149.VALID_TOKEN);
        String result = tools.secureRead("doc-1");
        assertTrue(result.contains("resource-doc-1"));
    }

    @Test
    void nullToken_returnsUnauthorized() {
        SecureMcpTools149 tools = new SecureMcpTools149(null);
        String result = tools.secureRead("doc-1");
        assertTrue(result.contains(SecureMcpTools149.ERR_UNAUTHORIZED));
    }

    @Test
    void emptyToken_returnsUnauthorized() {
        SecureMcpTools149 tools = new SecureMcpTools149("");
        String result = tools.secureRead("doc-1");
        assertTrue(result.contains(SecureMcpTools149.ERR_UNAUTHORIZED));
    }

    @Test
    void wrongToken_returnsForbidden() {
        SecureMcpTools149 tools = new SecureMcpTools149("wrong-token");
        String result = tools.secureRead("doc-1");
        assertTrue(result.contains(SecureMcpTools149.ERR_FORBIDDEN));
    }

    @Test
    void blankToken_returnsUnauthorized() {
        SecureMcpTools149 tools = new SecureMcpTools149("   ");
        String result = tools.secureRead("doc-1");
        assertTrue(result.contains(SecureMcpTools149.ERR_UNAUTHORIZED));
    }

    @Test
    void validToken_constant_nonEmpty() {
        assertFalse(SecureMcpTools149.VALID_TOKEN.isEmpty());
    }

    @Test
    void tool_annotation_present() throws Exception {
        var method = SecureMcpTools149.class.getDeclaredMethod("secureRead", String.class);
        Tool t = method.getAnnotation(Tool.class);
        assertNotNull(t);
        assertEquals("secure_read149", t.name());
    }

    @Test
    void registerAll_findsSecureReadTool() {
        skipIfAbsent();
        try (Runtime rt = new Runtime()) {
            SecureMcpTools149 tools = new SecureMcpTools149(SecureMcpTools149.VALID_TOKEN);
            List<ToolRegistration> regs = ToolRegistry.registerAll(rt, tools);
            assertEquals(1, regs.size());
            assertEquals("secure_read149", regs.get(0).spec().name());
            for (ToolRegistration reg : regs) reg.disposable().close();
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void validToken_differentResources_allSucceed() {
        SecureMcpTools149 tools = new SecureMcpTools149(SecureMcpTools149.VALID_TOKEN);
        for (String res : List.of("a", "b", "c")) {
            assertTrue(tools.secureRead(res).contains("resource-" + res));
        }
    }

    @Test
    void auth_errors_doNotLeakToken() {
        SecureMcpTools149 tools = new SecureMcpTools149("super-secret-key");
        String result = tools.secureRead("doc");
        assertFalse(result.contains("super-secret-key"));
    }

    private static void skipIfAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "ancora_ffi not present");
    }
}
