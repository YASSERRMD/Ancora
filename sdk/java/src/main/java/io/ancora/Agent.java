package io.ancora;

public final class Agent implements AutoCloseable {

    private final Runtime runtime;
    private final boolean ownsRuntime;

    public Agent() throws Throwable {
        this.runtime = new Runtime();
        this.ownsRuntime = true;
    }

    public Agent(Runtime runtime) {
        this.runtime = runtime;
        this.ownsRuntime = false;
    }

    public RunHandle run(AgentSpec spec) throws Throwable {
        byte[] specBytes = Wire.encodeAgentSpec(spec);
        String runId = runtime.startRun(specBytes);
        return new RunHandle(runtime, runId);
    }

    public RunHandle runGraph(GraphSpec spec) throws Throwable {
        byte[] specBytes = Wire.encodeGraphSpec(spec);
        String runId = runtime.startRun(specBytes);
        return new RunHandle(runtime, runId);
    }

    public Runtime runtime() {
        return runtime;
    }

    @Override
    public void close() throws Throwable {
        if (ownsRuntime) runtime.close();
    }
}
