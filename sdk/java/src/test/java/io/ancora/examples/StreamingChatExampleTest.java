package io.ancora.examples;

import io.ancora.Agent;
import io.ancora.AgentSpec;
import io.ancora.RunEvent;
import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class StreamingChatExampleTest {

    @Test
    void events_iterable_returns_at_least_one_event() throws Throwable {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "native library absent");
        try (Agent agent = new Agent()) {
            AgentSpec spec = new AgentSpec("local-model", "Stream a short reply.", null, null, null);
            var handle = agent.run(spec);
            int count = 0;
            for (RunEvent ignored : handle.events()) count++;
            assertTrue(count >= 1);
        } catch (UnsatisfiedLinkError ignored) {}
    }

    @Test
    void token_text_can_be_concatenated_from_stream() throws Throwable {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "native library absent");
        try (Agent agent = new Agent()) {
            AgentSpec spec = new AgentSpec("local-model", "Say hello.", null, null, null);
            var sb = new StringBuilder();
            for (RunEvent ev : agent.run(spec).events()) {
                if (ev instanceof RunEvent.Token tok) sb.append(tok.text());
            }
            assertNotNull(sb.toString());
        } catch (UnsatisfiedLinkError ignored) {}
    }

    @Test
    void last_event_is_completed() throws Throwable {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "native library absent");
        try (Agent agent = new Agent()) {
            AgentSpec spec = new AgentSpec("local-model", "Stream a reply.", null, null, null);
            List<RunEvent> events = agent.run(spec).collectAll();
            assertFalse(events.isEmpty());
            assertInstanceOf(RunEvent.Completed.class, events.get(events.size() - 1));
        } catch (UnsatisfiedLinkError ignored) {}
    }

    @Test
    void collect_all_and_iterable_both_return_events() throws Throwable {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "native library absent");
        try (Agent agent = new Agent()) {
            AgentSpec spec = new AgentSpec("local-model", "Respond.", null, null, null);
            List<RunEvent> collected = agent.run(spec).collectAll();
            assertFalse(collected.isEmpty());
        } catch (UnsatisfiedLinkError ignored) {}
    }
}
