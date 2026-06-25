package io.ancora.examples;

import io.ancora.*;

public class SingleAgent {
    public static void main(String[] args) throws Throwable {
        try (Agent agent = new Agent()) {
            AgentSpec spec = new AgentSpec(
                "claude-3-5-haiku-20241022",
                "You are a concise assistant. Answer in one sentence.",
                null, null, null);

            RunHandle handle = agent.run(spec);
            System.out.println("Run ID: " + handle.runId());

            for (RunEvent ev : handle.events()) {
                switch (ev) {
                    case RunEvent.Started s ->
                        System.out.println("[started]");
                    case RunEvent.Token t ->
                        System.out.print(t.text());
                    case RunEvent.ToolCall tc ->
                        System.out.println("[tool_call] " + tc.name() + " <- " + tc.input());
                    case RunEvent.Completed c ->
                        System.out.println("\n[completed]");
                    default -> {}
                }
            }
        }
    }
}
