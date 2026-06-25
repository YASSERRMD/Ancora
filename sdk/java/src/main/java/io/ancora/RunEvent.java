package io.ancora;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;

@JsonTypeInfo(
    use = JsonTypeInfo.Id.NAME,
    include = JsonTypeInfo.As.PROPERTY,
    property = "kind"
)
@JsonSubTypes({
    @JsonSubTypes.Type(value = RunEvent.Started.class, name = "started"),
    @JsonSubTypes.Type(value = RunEvent.Token.class, name = "token"),
    @JsonSubTypes.Type(value = RunEvent.Completed.class, name = "completed"),
    @JsonSubTypes.Type(value = RunEvent.Resumed.class, name = "resumed"),
    @JsonSubTypes.Type(value = RunEvent.ToolCall.class, name = "tool_call"),
})
public sealed interface RunEvent
    permits RunEvent.Started, RunEvent.Token, RunEvent.Completed, RunEvent.Resumed, RunEvent.ToolCall {

    @JsonIgnoreProperties(ignoreUnknown = true)
    record Started(String runId, String spec) implements RunEvent {}

    @JsonIgnoreProperties(ignoreUnknown = true)
    record Token(String runId, String token, String model) implements RunEvent {}

    @JsonIgnoreProperties(ignoreUnknown = true)
    record Completed(String runId) implements RunEvent {}

    @JsonIgnoreProperties(ignoreUnknown = true)
    record Resumed(String runId, String decision) implements RunEvent {}

    @JsonIgnoreProperties(ignoreUnknown = true)
    record ToolCall(String runId, String name, String input) implements RunEvent {}
}
