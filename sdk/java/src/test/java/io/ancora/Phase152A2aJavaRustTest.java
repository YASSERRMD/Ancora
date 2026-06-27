package io.ancora;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.junit.jupiter.api.Test;

import java.util.Map;

import static org.junit.jupiter.api.Assertions.*;

/** Cross-language A2A: Java hands off to Rust over A2A (offline fixture). */
public class Phase152A2aJavaRustTest {

    static final String HANDOFF_RUN_ID = "a2a-java-rust-001";
    static final ObjectMapper MAPPER = new ObjectMapper();

    static final Map<String, Object> ENVELOPE = Map.of(
        "protocol", "a2a/1.0",
        "sender", Map.of("lang", "java", "sdk_version", "0.3.0"),
        "recipient", Map.of("lang", "rust", "sdk_version", "0.3.0"),
        "run_id", HANDOFF_RUN_ID,
        "payload", Map.of(
            "task", "execute",
            "input", "Java produced this input",
            "handoff_reason", "executor runs in Rust"
        )
    );

    static final Map<String, Object> RESPONSE = Map.of(
        "protocol", "a2a/1.0",
        "sender", Map.of("lang", "rust"),
        "recipient", Map.of("lang", "java"),
        "run_id", HANDOFF_RUN_ID,
        "payload", Map.of("status", "executed", "result", "rust-ok")
    );

    @Test void envelopeProtocolIsA2a() { assertEquals("a2a/1.0", ENVELOPE.get("protocol")); }

    @Test void senderIsJava() {
        var sender = (Map<?, ?>) ENVELOPE.get("sender");
        assertEquals("java", sender.get("lang"));
    }

    @Test void recipientIsRust() {
        var recipient = (Map<?, ?>) ENVELOPE.get("recipient");
        assertEquals("rust", recipient.get("lang"));
    }

    @Test void responseRunIdMatchesEnvelope() { assertEquals(ENVELOPE.get("run_id"), RESPONSE.get("run_id")); }

    @Test void responseSenderIsRust() {
        var sender = (Map<?, ?>) RESPONSE.get("sender");
        assertEquals("rust", sender.get("lang"));
    }

    @Test void responsePayloadHasStatus() {
        var payload = (Map<?, ?>) RESPONSE.get("payload");
        assertEquals("executed", payload.get("status"));
    }

    @Test void envelopeSerialisesToJson() throws Exception {
        String json = MAPPER.writeValueAsString(ENVELOPE);
        Map<?, ?> decoded = MAPPER.readValue(json, Map.class);
        assertEquals("a2a/1.0", decoded.get("protocol"));
    }

    @Test void payloadHasHandoffReason() {
        var payload = (Map<?, ?>) ENVELOPE.get("payload");
        assertNotNull(payload.get("handoff_reason"));
    }
}
