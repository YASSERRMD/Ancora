use std::sync::Arc;

use ancora_proto::ancora::{
    content_block::Block, AgentSpec, ContentBlock, Message as ProtoMessage, Role, TextContent,
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
}

/// Outcome of one agent loop step.
pub enum StepOutcome {
    /// The model produced text with no tool calls; the loop is done.
    FinalOutput { text: String },
    /// The model requested one or more tool calls.
    ToolCalls { calls: Vec<ToolCallContent> },
}

/// Single-agent runtime built from an `AgentSpec`.
pub struct Agent {
    pub spec: AgentSpec,
    pub run: Run,
    messages: Vec<ProtoMessage>,
    step: u32,
    store: Arc<dyn JournalStore>,
}

impl Agent {
    pub fn new(spec: AgentSpec, run_id: impl Into<String>, store: Arc<dyn JournalStore>) -> Self {
        Self {
            spec,
            run: Run::new(run_id),
            messages: Vec::new(),
            step: 0,
            store,
        }
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

        self.messages.push(response);
        self.step += 1;

        if tool_calls.is_empty() {
            Ok(StepOutcome::FinalOutput { text })
        } else {
            Ok(StepOutcome::ToolCalls { calls: tool_calls })
        }
    }

    /// Run the reason-act loop until the model produces a final output.
    pub fn run_loop(
        &mut self,
        input: &str,
        model: &dyn ModelClient,
        dispatcher: &dyn ToolDispatcher,
    ) -> Result<String, AncoraError> {
        if !self.spec.instructions.is_empty() {
            self.messages.push(text_message(Role::System, &self.spec.instructions));
        }
        self.messages.push(text_message(Role::User, input));

        loop {
            let max_steps = self.spec.max_steps;
            if max_steps > 0 && self.step >= max_steps {
                return Err(AncoraError::MaxSteps { max_steps });
            }

            match self.step(model)? {
                StepOutcome::FinalOutput { text } => return Ok(text),
                StepOutcome::ToolCalls { calls } => {
                    for call in &calls {
                        let result = dispatcher.dispatch(call)?;
                        self.messages.push(tool_result_message(result));
                    }
                }
            }
        }
    }
}

fn text_message(role: Role, text: &str) -> ProtoMessage {
    ProtoMessage {
        id: String::new(),
        role: role as i32,
        content: vec![ContentBlock {
            block: Some(Block::Text(TextContent { text: text.to_string() })),
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
