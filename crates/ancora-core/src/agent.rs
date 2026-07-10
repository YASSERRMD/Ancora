use std::sync::Arc;

use ancora_proto::ancora::{
    content_block::Block, journal_event::Event as JournalEventVariant, ActivityRecordedEvent,
    AgentSpec, ContentBlock, JournalEvent, Message as ProtoMessage, Role, TextContent,
    ToolCallContent, ToolResultContent,
};

use crate::error::AncoraError;
use crate::journal::JournalStore;
use crate::run::Run;

/// Drives model completions for the agent loop.
pub trait ModelClient: Send + Sync {
    fn complete(
        &self,
        messages: &[ProtoMessage],
        spec: &AgentSpec,
    ) -> Result<ProtoMessage, AncoraError>;
}

/// Dispatches tool calls to registered tool implementations.
pub trait ToolDispatcher: Send + Sync {
    fn dispatch(&self, call: &ToolCallContent) -> Result<ToolResultContent, AncoraError>;

    /// Returns true if `call` must be approved by a human before `dispatch`
    /// is invoked. When true, `Agent::run_loop`/`resume` suspend and return
    /// `AgentOutcome::Suspended` instead of calling `dispatch`; a caller
    /// resumes the run later with a decision via `Agent::resume`.
    ///
    /// Default: never requires approval, so every dispatcher that predates
    /// this method keeps its existing fully-synchronous behavior with no
    /// changes required.
    fn requires_approval(&self, _call: &ToolCallContent) -> bool {
        false
    }
}

/// Outcome of one agent loop step.
pub enum StepOutcome {
    /// The model produced text with no tool calls; the loop is done.
    FinalOutput { text: String },
    /// The model requested one or more tool calls.
    ToolCalls { calls: Vec<ToolCallContent> },
}

/// Outcome of driving the agent loop (`run_loop` or `resume`) to either
/// completion or the next point where it needs a human decision.
#[derive(Debug)]
pub enum AgentOutcome {
    /// The loop ran to completion and produced final text.
    Completed(String),
    /// The loop is paused waiting on a human decision for `tool_call`,
    /// which `dispatcher.requires_approval` flagged. Call `Agent::resume`
    /// with the decision to continue.
    Suspended {
        tool_call: ToolCallContent,
        prompt: String,
    },
}

/// Single-agent runtime built from an `AgentSpec`.
pub struct Agent {
    pub spec: AgentSpec,
    pub run: Run,
    messages: Vec<ProtoMessage>,
    step: u32,
    journal_seq: u64,
    store: Arc<dyn JournalStore>,
    /// Tool calls from the current step not yet resolved into a message.
    /// Non-empty only while suspended waiting on a human decision for the
    /// call at the front; the remaining calls (if any) still need
    /// dispatching once that decision arrives.
    pending_calls: Vec<ToolCallContent>,
    /// The step number `pending_calls` was produced at, so a journal key
    /// written after resuming still reflects the step the model actually
    /// requested the call in.
    pending_step: u32,
}

impl Agent {
    pub fn new(spec: AgentSpec, run_id: impl Into<String>, store: Arc<dyn JournalStore>) -> Self {
        Self {
            spec,
            run: Run::new(run_id),
            messages: Vec::new(),
            step: 0,
            journal_seq: 0,
            store,
            pending_calls: Vec::new(),
            pending_step: 0,
        }
    }

    /// The tool_call_id of the pending approval-gated call, if the agent is
    /// currently suspended.
    pub fn pending_tool_call_id(&self) -> Option<&str> {
        self.pending_calls.first().map(|c| c.tool_call_id.as_str())
    }

    /// Call the model with the current message history and return the step outcome.
    pub fn step(&mut self, model: &dyn ModelClient) -> Result<StepOutcome, AncoraError> {
        let response = model.complete(&self.messages, &self.spec)?;

        let tool_calls: Vec<ToolCallContent> = response
            .content
            .iter()
            .filter_map(|b| {
                if let Some(Block::ToolCall(tc)) = &b.block {
                    Some(tc.clone())
                } else {
                    None
                }
            })
            .collect();

        let text: String = response
            .content
            .iter()
            .filter_map(|b| {
                if let Some(Block::Text(t)) = &b.block {
                    Some(t.text.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .join("");

        let key = format!("step:{}:model", self.step);
        self.journal_append(key, "model_call", &text)?;

        self.messages.push(response);
        self.step += 1;

        if tool_calls.is_empty() {
            Ok(StepOutcome::FinalOutput { text })
        } else {
            Ok(StepOutcome::ToolCalls { calls: tool_calls })
        }
    }

    /// Run the reason-act loop until the model produces a final output or
    /// hits an approval-gated tool call.
    pub fn run_loop(
        &mut self,
        input: &str,
        model: &dyn ModelClient,
        dispatcher: &dyn ToolDispatcher,
    ) -> Result<AgentOutcome, AncoraError> {
        if !self.spec.instructions.is_empty() {
            self.messages
                .push(text_message(Role::System, &self.spec.instructions));
        }
        self.messages.push(text_message(Role::User, input));
        self.drive_loop(model, dispatcher)
    }

    /// Continue a run suspended by `AgentOutcome::Suspended`, supplying the
    /// human's decision for the pending tool call, then drive the loop
    /// forward exactly as `run_loop` does.
    ///
    /// Returns `AncoraError::InvalidState` if nothing is pending (the run
    /// already completed/failed, or was never suspended) -- callers that
    /// resume defensively/idempotently should check for a `Suspended`
    /// outcome first rather than relying on this call succeeding.
    pub fn resume(
        &mut self,
        decision: ToolResultContent,
        model: &dyn ModelClient,
        dispatcher: &dyn ToolDispatcher,
    ) -> Result<AgentOutcome, AncoraError> {
        if self.pending_calls.is_empty() {
            return Err(AncoraError::InvalidState(
                "no pending tool call to resume".to_string(),
            ));
        }
        let call = self.pending_calls.remove(0);
        let key = format!("step:{}:tool:{}", self.pending_step, call.tool_call_id);
        self.journal_append(key, "tool_result", &decision.result_json)?;
        self.messages.push(tool_result_message(decision));
        self.drive_loop(model, dispatcher)
    }

    fn drive_loop(
        &mut self,
        model: &dyn ModelClient,
        dispatcher: &dyn ToolDispatcher,
    ) -> Result<AgentOutcome, AncoraError> {
        loop {
            if self.pending_calls.is_empty() {
                let max_steps = self.spec.max_steps;
                if max_steps > 0 && self.step >= max_steps {
                    return Err(AncoraError::MaxSteps { max_steps });
                }

                let step_num = self.step;
                match self.step(model)? {
                    StepOutcome::FinalOutput { text } => return Ok(AgentOutcome::Completed(text)),
                    StepOutcome::ToolCalls { calls } => {
                        self.pending_calls = calls;
                        self.pending_step = step_num;
                    }
                }
            }

            while let Some(call) = self.pending_calls.first().cloned() {
                if dispatcher.requires_approval(&call) {
                    let prompt = format!("tool '{}' requires human approval", call.tool_name);
                    return Ok(AgentOutcome::Suspended {
                        tool_call: call,
                        prompt,
                    });
                }
                let result = dispatcher.dispatch(&call)?;
                let key = format!("step:{}:tool:{}", self.pending_step, call.tool_call_id);
                self.journal_append(key, "tool_result", &result.result_json)?;
                self.messages.push(tool_result_message(result));
                self.pending_calls.remove(0);
            }
        }
    }

    fn journal_append(
        &mut self,
        key: String,
        kind: &str,
        result_json: &str,
    ) -> Result<(), AncoraError> {
        let seq = self.journal_seq;
        self.journal_seq += 1;
        self.store
            .append(
                &self.run.id,
                JournalEvent {
                    event_id: key.clone(),
                    run_id: self.run.id.clone(),
                    seq,
                    recorded_at_ns: 0,
                    event: Some(JournalEventVariant::ActivityRecorded(
                        ActivityRecordedEvent {
                            activity_key: key,
                            activity_kind: kind.to_string(),
                            input_json: String::new(),
                            result_json: result_json.to_string(),
                            replayed: false,
                        },
                    )),
                },
            )
            .map(|_| ())
    }
}

fn text_message(role: Role, text: &str) -> ProtoMessage {
    ProtoMessage {
        id: String::new(),
        role: role as i32,
        content: vec![ContentBlock {
            block: Some(Block::Text(TextContent {
                text: text.to_string(),
            })),
        }],
        created_at_ns: 0,
        usage: None,
        cost: None,
        model_id: String::new(),
    }
}

fn tool_result_message(result: ToolResultContent) -> ProtoMessage {
    ProtoMessage {
        id: String::new(),
        role: Role::Tool as i32,
        content: vec![ContentBlock {
            block: Some(Block::ToolResult(result)),
        }],
        created_at_ns: 0,
        usage: None,
        cost: None,
        model_id: String::new(),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ancora_proto::ancora::{
        content_block::Block, ContentBlock, Role, TextContent, ToolCallContent, ToolResultContent,
    };

    use super::*;
    use crate::journal::MemoryStore;

    fn make_spec(max_steps: u32) -> AgentSpec {
        AgentSpec {
            name: "test".to_string(),
            model_id: "mock".to_string(),
            instructions: String::new(),
            output_schema_json: String::new(),
            tools: vec![],
            max_steps,
            model_retry: None,
            model_params_json: String::new(),
        }
    }

    fn text_response(text: &str) -> ProtoMessage {
        ProtoMessage {
            id: String::new(),
            role: Role::Assistant as i32,
            content: vec![ContentBlock {
                block: Some(Block::Text(TextContent {
                    text: text.to_string(),
                })),
            }],
            created_at_ns: 0,
            usage: None,
            cost: None,
            model_id: String::new(),
        }
    }

    #[test]
    fn loop_terminates_on_final_output() {
        struct AlwaysText;
        impl ModelClient for AlwaysText {
            fn complete(
                &self,
                _: &[ProtoMessage],
                _: &AgentSpec,
            ) -> Result<ProtoMessage, AncoraError> {
                Ok(text_response("the answer"))
            }
        }
        struct Noop;
        impl ToolDispatcher for Noop {
            fn dispatch(&self, _: &ToolCallContent) -> Result<ToolResultContent, AncoraError> {
                unreachable!("no tools should be called")
            }
        }

        let mut agent = Agent::new(make_spec(10), "run-loop-1", Arc::new(MemoryStore::new()));
        let outcome = agent.run_loop("hello", &AlwaysText, &Noop).unwrap();
        assert!(matches!(outcome, AgentOutcome::Completed(text) if text == "the answer"));
        assert_eq!(agent.step, 1);
    }

    #[test]
    fn loop_halts_at_max_steps() {
        struct AlwaysTool;
        impl ModelClient for AlwaysTool {
            fn complete(
                &self,
                _: &[ProtoMessage],
                _: &AgentSpec,
            ) -> Result<ProtoMessage, AncoraError> {
                Ok(ProtoMessage {
                    id: String::new(),
                    role: Role::Assistant as i32,
                    content: vec![ContentBlock {
                        block: Some(Block::ToolCall(ToolCallContent {
                            tool_call_id: "tc-1".to_string(),
                            tool_name: "noop".to_string(),
                            arguments_json: "{}".to_string(),
                        })),
                    }],
                    created_at_ns: 0,
                    usage: None,
                    cost: None,
                    model_id: String::new(),
                })
            }
        }
        struct EchoTool;
        impl ToolDispatcher for EchoTool {
            fn dispatch(&self, call: &ToolCallContent) -> Result<ToolResultContent, AncoraError> {
                Ok(ToolResultContent {
                    tool_call_id: call.tool_call_id.clone(),
                    result_json: r#""ok""#.to_string(),
                    is_error: false,
                })
            }
        }

        let mut agent = Agent::new(make_spec(2), "run-loop-2", Arc::new(MemoryStore::new()));
        let err = agent.run_loop("go", &AlwaysTool, &EchoTool).unwrap_err();
        assert!(matches!(err, AncoraError::MaxSteps { max_steps: 2 }));
    }

    // ---- suspend / resume for approval-gated tool calls -------------------

    fn tool_call_response(id: &str, name: &str) -> ProtoMessage {
        ProtoMessage {
            id: String::new(),
            role: Role::Assistant as i32,
            content: vec![ContentBlock {
                block: Some(Block::ToolCall(ToolCallContent {
                    tool_call_id: id.to_string(),
                    tool_name: name.to_string(),
                    arguments_json: "{}".to_string(),
                })),
            }],
            created_at_ns: 0,
            usage: None,
            cost: None,
            model_id: String::new(),
        }
    }

    fn ok_result(call: &ToolCallContent) -> ToolResultContent {
        ToolResultContent {
            tool_call_id: call.tool_call_id.clone(),
            result_json: r#""ok""#.to_string(),
            is_error: false,
        }
    }

    /// Requests one tool call, then (once it sees a tool result in the
    /// message history) returns final text.
    struct ToolThenDone {
        tool_name: &'static str,
    }
    impl ModelClient for ToolThenDone {
        fn complete(
            &self,
            messages: &[ProtoMessage],
            _spec: &AgentSpec,
        ) -> Result<ProtoMessage, AncoraError> {
            let saw_result = messages.iter().any(|m| {
                m.content
                    .iter()
                    .any(|b| matches!(&b.block, Some(Block::ToolResult(_))))
            });
            if saw_result {
                Ok(text_response("done"))
            } else {
                Ok(tool_call_response("tc-1", self.tool_name))
            }
        }
    }

    /// Gates every tool call whose name is in `gated`.
    struct GatedDispatcher {
        gated: &'static [&'static str],
    }
    impl ToolDispatcher for GatedDispatcher {
        fn dispatch(&self, call: &ToolCallContent) -> Result<ToolResultContent, AncoraError> {
            Ok(ok_result(call))
        }
        fn requires_approval(&self, call: &ToolCallContent) -> bool {
            self.gated.contains(&call.tool_name.as_str())
        }
    }

    #[test]
    fn run_loop_suspends_on_approval_gated_tool_call() {
        let model = ToolThenDone { tool_name: "risky" };
        let dispatcher = GatedDispatcher { gated: &["risky"] };
        let mut agent = Agent::new(make_spec(10), "run-suspend-1", Arc::new(MemoryStore::new()));

        let outcome = agent.run_loop("go", &model, &dispatcher).unwrap();
        match outcome {
            AgentOutcome::Suspended { tool_call, prompt } => {
                assert_eq!(tool_call.tool_name, "risky");
                assert_eq!(tool_call.tool_call_id, "tc-1");
                assert!(prompt.contains("risky"));
            }
            AgentOutcome::Completed(_) => panic!("expected Suspended, got Completed"),
        }
        assert_eq!(agent.pending_tool_call_id(), Some("tc-1"));
    }

    #[test]
    fn resume_with_approval_decision_completes_run() {
        let model = ToolThenDone { tool_name: "risky" };
        let dispatcher = GatedDispatcher { gated: &["risky"] };
        let mut agent = Agent::new(make_spec(10), "run-suspend-2", Arc::new(MemoryStore::new()));
        agent.run_loop("go", &model, &dispatcher).unwrap();

        let decision = ToolResultContent {
            tool_call_id: "tc-1".to_string(),
            result_json: r#""approved""#.to_string(),
            is_error: false,
        };
        let outcome = agent.resume(decision, &model, &dispatcher).unwrap();
        assert!(matches!(outcome, AgentOutcome::Completed(text) if text == "done"));
        assert_eq!(agent.pending_tool_call_id(), None);
    }

    #[test]
    fn resume_without_pending_call_returns_invalid_state() {
        let model = ToolThenDone { tool_name: "risky" };
        let dispatcher = GatedDispatcher { gated: &[] };
        let mut agent = Agent::new(make_spec(10), "run-suspend-3", Arc::new(MemoryStore::new()));
        // Nothing is gated, so this run completes immediately -- never suspends.
        agent.run_loop("go", &model, &dispatcher).unwrap();

        let decision = ToolResultContent {
            tool_call_id: "tc-1".to_string(),
            result_json: "null".to_string(),
            is_error: false,
        };
        let err = agent.resume(decision, &model, &dispatcher).unwrap_err();
        assert!(matches!(err, AncoraError::InvalidState(_)));
    }

    /// A model batch that requests two gated tool calls in the same step,
    /// each requiring its own resume before the loop can proceed.
    struct TwoToolsThenDone {
        step: std::sync::atomic::AtomicU32,
    }
    impl ModelClient for TwoToolsThenDone {
        fn complete(
            &self,
            _messages: &[ProtoMessage],
            _spec: &AgentSpec,
        ) -> Result<ProtoMessage, AncoraError> {
            let n = self.step.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            if n == 0 {
                Ok(ProtoMessage {
                    id: String::new(),
                    role: Role::Assistant as i32,
                    content: vec![
                        ContentBlock {
                            block: Some(Block::ToolCall(ToolCallContent {
                                tool_call_id: "tc-a".to_string(),
                                tool_name: "risky".to_string(),
                                arguments_json: "{}".to_string(),
                            })),
                        },
                        ContentBlock {
                            block: Some(Block::ToolCall(ToolCallContent {
                                tool_call_id: "tc-b".to_string(),
                                tool_name: "risky".to_string(),
                                arguments_json: "{}".to_string(),
                            })),
                        },
                    ],
                    created_at_ns: 0,
                    usage: None,
                    cost: None,
                    model_id: String::new(),
                })
            } else {
                Ok(text_response("done"))
            }
        }
    }

    #[test]
    fn resume_drains_remaining_gated_calls_in_same_step() {
        let model = TwoToolsThenDone {
            step: std::sync::atomic::AtomicU32::new(0),
        };
        let dispatcher = GatedDispatcher { gated: &["risky"] };
        let mut agent = Agent::new(make_spec(10), "run-suspend-4", Arc::new(MemoryStore::new()));

        let outcome = agent.run_loop("go", &model, &dispatcher).unwrap();
        let tool_call = match outcome {
            AgentOutcome::Suspended { tool_call, .. } => tool_call,
            AgentOutcome::Completed(_) => panic!("expected Suspended"),
        };
        assert_eq!(tool_call.tool_call_id, "tc-a");

        let decision_a = ToolResultContent {
            tool_call_id: "tc-a".to_string(),
            result_json: r#""ok-a""#.to_string(),
            is_error: false,
        };
        let outcome = agent.resume(decision_a, &model, &dispatcher).unwrap();
        let tool_call = match outcome {
            AgentOutcome::Suspended { tool_call, .. } => tool_call,
            AgentOutcome::Completed(_) => panic!("expected second Suspended for tc-b"),
        };
        assert_eq!(tool_call.tool_call_id, "tc-b");

        let decision_b = ToolResultContent {
            tool_call_id: "tc-b".to_string(),
            result_json: r#""ok-b""#.to_string(),
            is_error: false,
        };
        let outcome = agent.resume(decision_b, &model, &dispatcher).unwrap();
        assert!(matches!(outcome, AgentOutcome::Completed(text) if text == "done"));
    }
}
