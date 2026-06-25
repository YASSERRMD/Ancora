package io.ancora;

import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;
import java.util.NoSuchElementException;

public final class RunHandle {

    private final Runtime runtime;
    private final String runId;

    RunHandle(Runtime runtime, String runId) {
        this.runtime = runtime;
        this.runId = runId;
    }

    public String runId() {
        return runId;
    }

    public Iterable<RunEvent> events() {
        return () -> new Iterator<>() {
            private RunEvent pending = null;
            private boolean done = false;

            @Override
            public boolean hasNext() {
                if (done) return false;
                if (pending != null) return true;
                try {
                    String json = runtime.pollEvent(runId);
                    if (json == null) {
                        done = true;
                        return false;
                    }
                    pending = Wire.parseEvent(json);
                    return true;
                } catch (Throwable t) {
                    throw new RuntimeException("Error polling next event", t);
                }
            }

            @Override
            public RunEvent next() {
                if (!hasNext()) throw new NoSuchElementException();
                RunEvent e = pending;
                pending = null;
                return e;
            }
        };
    }

    public List<RunEvent> collectAll() {
        List<RunEvent> list = new ArrayList<>();
        for (RunEvent e : events()) list.add(e);
        return list;
    }

    public RunHandle resume(byte[] decisionBytes) throws Throwable {
        runtime.resumeRun(runId, decisionBytes);
        return this;
    }

    public RunHandle resume(String decisionJson) throws Throwable {
        return resume(decisionJson.getBytes(StandardCharsets.UTF_8));
    }

    public List<RunEvent> resumeAndCollectAll(byte[] decisionBytes) throws Throwable {
        resume(decisionBytes);
        return collectAll();
    }

    public String getCost() throws Throwable {
        return runtime.getCost(runId);
    }
}
