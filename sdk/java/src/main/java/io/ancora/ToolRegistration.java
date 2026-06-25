package io.ancora;

public final class ToolRegistration implements AutoCloseable {

    private final ToolSpec spec;
    private final AutoCloseable disposable;

    ToolRegistration(ToolSpec spec, AutoCloseable disposable) {
        this.spec = spec;
        this.disposable = disposable;
    }

    public ToolSpec spec() {
        return spec;
    }

    @Override
    public void close() throws Exception {
        disposable.close();
    }
}
