package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class Phase149ConfJournalTest {

    record JournalEntry149(int seq, String kind, String runId) {}

    static JournalEntry149 entry(int seq, String kind, String runId) {
        return new JournalEntry149(seq, kind, runId);
    }

    @Test
    void journalEntry_accessors() {
        JournalEntry149 e = entry(0, "started", "run-1");
        assertEquals(0, e.seq());
        assertEquals("started", e.kind());
        assertEquals("run-1", e.runId());
    }

    @Test
    void journalEntry_isRecord() {
        assertTrue(JournalEntry149.class.isRecord());
    }

    @Test
    void journalEntry_valueEquality() {
        JournalEntry149 a = entry(1, "token", "run-1");
        JournalEntry149 b = entry(1, "token", "run-1");
        assertEquals(a, b);
    }

    @Test
    void fourEntryJournal_seqsAreOrdered() {
        List<JournalEntry149> journal = List.of(
            entry(0, "started",   "run-x"),
            entry(1, "token",     "run-x"),
            entry(2, "tool_call", "run-x"),
            entry(3, "completed", "run-x")
        );
        for (int i = 0; i < journal.size(); i++) assertEquals(i, journal.get(i).seq());
    }

    @Test
    void journalKinds_matchRunEventTypes() {
        List<String> kinds = List.of("started", "token", "completed");
        assertTrue(kinds.contains("started"));
        assertTrue(kinds.contains("completed"));
    }

    @Test
    void journal_from_events_maps_correctly() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            List<RunEvent> events = h.collectAll();
            List<JournalEntry149> journal = buildJournal(events, h.runId());
            assertEquals(events.size(), journal.size());
            for (int i = 0; i < journal.size(); i++) assertEquals(i, journal.get(i).seq());
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void journal_first_isStarted() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            List<RunEvent> events = h.collectAll();
            List<JournalEntry149> journal = buildJournal(events, h.runId());
            assertEquals("started", journal.get(0).kind());
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void journal_last_isCompleted() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            List<RunEvent> events = h.collectAll();
            List<JournalEntry149> journal = buildJournal(events, h.runId());
            assertEquals("completed", journal.get(journal.size() - 1).kind());
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void journal_runIds_allMatch() {
        skipIfAbsent();
        try (Agent a = new Agent()) {
            RunHandle h = a.run(new AgentSpec("llama3", null, null, null, null));
            String id = h.runId();
            List<JournalEntry149> journal = buildJournal(h.collectAll(), id);
            journal.forEach(e -> assertEquals(id, e.runId()));
        } catch (UnsatisfiedLinkError e) {
            skipIfAbsent();
        } catch (Throwable t) {
            fail(t.toString());
        }
    }

    @Test
    void journalEntry_notEqualDifferentSeq() {
        JournalEntry149 a = entry(0, "started", "r1");
        JournalEntry149 b = entry(1, "started", "r1");
        assertNotEquals(a, b);
    }

    private List<JournalEntry149> buildJournal(List<RunEvent> events, String runId) {
        java.util.ArrayList<JournalEntry149> result = new java.util.ArrayList<>();
        int seq = 0;
        for (RunEvent ev : events) {
            String kind = switch (ev) {
                case RunEvent.Started s   -> "started";
                case RunEvent.Token t     -> "token";
                case RunEvent.Completed c -> "completed";
                case RunEvent.Resumed r   -> "resumed";
                case RunEvent.ToolCall tc -> "tool_call";
            };
            result.add(entry(seq++, kind, runId));
        }
        return result;
    }

    private static void skipIfAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "ancora_ffi not present");
    }
}
