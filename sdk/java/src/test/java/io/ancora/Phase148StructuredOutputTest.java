package io.ancora;

import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class Phase148StructuredOutputTest {

    record AnswerRecord148(String answer, double confidence) {}

    @Test
    void agentSpec_isRecord() {
        assertTrue(AgentSpec.class.isRecord());
    }

    @Test
    void agentSpec_valueEquality() {
        AgentSpec a = new AgentSpec("llama3", "sys", null, null, null);
        AgentSpec b = new AgentSpec("llama3", "sys", null, null, null);
        assertEquals(a, b);
    }

    @Test
    void agentSpec_model_accessor() {
        AgentSpec spec = new AgentSpec("gpt-4o", null, null, null, null);
        assertEquals("gpt-4o", spec.model());
    }

    @Test
    void toolSpec_isRecord() {
        assertTrue(ToolSpec.class.isRecord());
    }

    @Test
    void toolSpec_valueEquality() {
        ToolSpec a = new ToolSpec("tool_a", "Does A", null);
        ToolSpec b = new ToolSpec("tool_a", "Does A", null);
        assertEquals(a, b);
    }

    @Test
    void answerRecord_storesFields() {
        AnswerRecord148 ans = new AnswerRecord148("Paris", 0.99);
        assertEquals("Paris", ans.answer());
        assertEquals(0.99, ans.confidence());
    }

    @Test
    void answerRecord_valueEquality() {
        AnswerRecord148 a = new AnswerRecord148("Paris", 0.99);
        AnswerRecord148 b = new AnswerRecord148("Paris", 0.99);
        assertEquals(a, b);
    }

    @Test
    void startedEvent_isRecord() {
        assertTrue(RunEvent.Started.class.isRecord());
    }

    @Test
    void tokenEvent_storesText() {
        RunEvent.Token ev = new RunEvent.Token("r1", "Hello", "llama3");
        assertEquals("Hello", ev.text());
        assertEquals("llama3", ev.model());
    }

    @Test
    void toolCallEvent_storesNameAndInput() {
        RunEvent.ToolCall ev = new RunEvent.ToolCall("r1", "search", "{\"q\":\"cats\"}");
        assertEquals("search", ev.name());
        assertEquals("{\"q\":\"cats\"}", ev.input());
    }
}
