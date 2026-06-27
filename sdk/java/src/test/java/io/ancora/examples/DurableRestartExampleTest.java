package io.ancora.examples;

import io.ancora.Agent;
import io.ancora.AgentSpec;
import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;
import static io.ancora.examples.SharedHelpers.RunJournal;

class DurableRestartExampleTest {

    @Test
    void journal_records_and_replays_events() {
        RunJournal journal = new RunJournal();
        journal.recordRun("run-1");
        journal.appendEvent("run-1", "{\"kind\":\"started\"}");
        journal.appendEvent("run-1", "{\"kind\":\"completed\"}");

        assertEquals(2, journal.eventsForRun("run-1").size());
        assertEquals(1, journal.runCount());
    }

    @Test
    void journal_returns_empty_for_unknown_run() {
        RunJournal journal = new RunJournal();
        assertTrue(journal.eventsForRun("missing").isEmpty());
    }

    @Test
    void journal_tracks_multiple_runs() {
        RunJournal journal = new RunJournal();
        journal.recordRun("a");
        journal.recordRun("b");
        assertEquals(2, journal.runCount());
        assertTrue(journal.eventsForRun("a").isEmpty());
    }

    @Test
    void record_run_is_idempotent() {
        RunJournal journal = new RunJournal();
        journal.recordRun("dup");
        journal.recordRun("dup");
        assertEquals(1, journal.runCount());
    }

    @Test
    void durable_restart_persists_live_events() throws Throwable {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "native library absent");
        try (Agent agent = new Agent()) {
            RunJournal journal = new RunJournal();
            AgentSpec spec = new AgentSpec("local-model", "Persist my events.", null, null, null);
            var handle = agent.run(spec);
            String runId = handle.runId();
            journal.recordRun(runId);

            for (var ev : handle.events())
                journal.appendEvent(runId, ev.toString());

            List<String> replayed = journal.eventsForRun(runId);
            assertFalse(replayed.isEmpty());
            assertEquals(1, journal.runCount());
        } catch (UnsatisfiedLinkError ignored) {}
    }
}
