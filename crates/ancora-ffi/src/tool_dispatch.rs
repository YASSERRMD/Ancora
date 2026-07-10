use std::sync::Mutex;

use ancora_core::agent::ToolDispatcher;
use ancora_core::error::AncoraError;
use ancora_proto::ancora::{ToolCallContent, ToolResultContent};

use crate::buffer::{ancora_buffer_free, AncorBuffer};
use crate::error_code::AncorErrorCode;
use crate::tool_registry::ToolRegistry;

/// Bridges `ancora_core::agent::ToolDispatcher` to the host tool callbacks
/// registered through `ancora_tool_register`. Also records a `tool_call`
/// lifecycle event for every dispatch, drained into the run's FFI event
/// queue after the loop finishes.
pub(crate) struct FfiToolDispatcher<'a> {
    tools: &'a Mutex<ToolRegistry>,
    run_id: String,
    events: Mutex<Vec<String>>,
}

impl<'a> FfiToolDispatcher<'a> {
    pub(crate) fn new(tools: &'a Mutex<ToolRegistry>, run_id: &str) -> Self {
        Self {
            tools,
            run_id: run_id.to_owned(),
            events: Mutex::new(Vec::new()),
        }
    }

    pub(crate) fn into_events(self) -> Vec<String> {
        self.events.into_inner().unwrap()
    }
}

impl ToolDispatcher for FfiToolDispatcher<'_> {
    fn dispatch(&self, call: &ToolCallContent) -> Result<ToolResultContent, AncoraError> {
        self.events.lock().unwrap().push(
            serde_json::json!({
                "kind": "tool_call",
                "run_id": self.run_id,
                "tool_call_id": call.tool_call_id,
                "name": call.tool_name,
                "input": call.arguments_json,
            })
            .to_string(),
        );

        let callback = { self.tools.lock().unwrap().get(&call.tool_name) };
        let Some(callback) = callback else {
            return Err(AncoraError::ToolNotFound(call.tool_name.clone()));
        };

        let input = call.arguments_json.as_bytes();
        let mut out = AncorBuffer {
            ptr: std::ptr::null_mut(),
            len: 0,
        };
        // Safety: `callback` was registered via `ancora_tool_register`, which
        // requires the host to uphold the `AncorToolCallback` contract
        // (valid function pointer, writes a well-formed buffer to `out`).
        let code = unsafe { callback(input.as_ptr(), input.len(), &mut out) };

        if code != AncorErrorCode::Ok {
            return Ok(ToolResultContent {
                tool_call_id: call.tool_call_id.clone(),
                result_json: serde_json::json!({
                    "error": format!("tool callback returned error code {code:?}")
                })
                .to_string(),
                is_error: true,
            });
        }

        let result_json = if out.ptr.is_null() || out.len == 0 {
            String::new()
        } else {
            let slice = unsafe { std::slice::from_raw_parts(out.ptr, out.len) };
            String::from_utf8_lossy(slice).into_owned()
        };
        unsafe { ancora_buffer_free(out) };

        Ok(ToolResultContent {
            tool_call_id: call.tool_call_id.clone(),
            result_json,
            is_error: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tool_callback::AncorToolCallback;

    unsafe extern "C" fn echo_callback(
        input: *const u8,
        input_len: usize,
        out: *mut AncorBuffer,
    ) -> AncorErrorCode {
        let slice = unsafe { std::slice::from_raw_parts(input, input_len) };
        unsafe { *out = crate::buffer::ancora_buffer_from_str(&String::from_utf8_lossy(slice)) };
        AncorErrorCode::Ok
    }

    unsafe extern "C" fn failing_callback(
        _input: *const u8,
        _input_len: usize,
        _out: *mut AncorBuffer,
    ) -> AncorErrorCode {
        AncorErrorCode::Internal
    }

    fn call(name: &str) -> ToolCallContent {
        ToolCallContent {
            tool_call_id: "tc-1".into(),
            tool_name: name.into(),
            arguments_json: r#"{"q":"hi"}"#.into(),
        }
    }

    #[test]
    fn dispatch_invokes_registered_callback_and_returns_its_output() {
        let mut registry = ToolRegistry::new();
        registry.register("echo", echo_callback as AncorToolCallback);
        let tools = Mutex::new(registry);
        let dispatcher = FfiToolDispatcher::new(&tools, "run-1");

        let result = dispatcher.dispatch(&call("echo")).unwrap();
        assert_eq!(result.result_json, r#"{"q":"hi"}"#);
        assert!(!result.is_error);
    }

    #[test]
    fn dispatch_records_a_tool_call_event() {
        let mut registry = ToolRegistry::new();
        registry.register("echo", echo_callback as AncorToolCallback);
        let tools = Mutex::new(registry);
        let dispatcher = FfiToolDispatcher::new(&tools, "run-1");
        dispatcher.dispatch(&call("echo")).unwrap();

        let events = dispatcher.into_events();
        assert_eq!(events.len(), 1);
        assert!(events[0].contains("tool_call"));
        assert!(events[0].contains("echo"));
    }

    #[test]
    fn dispatch_unregistered_tool_returns_tool_not_found() {
        let tools = Mutex::new(ToolRegistry::new());
        let dispatcher = FfiToolDispatcher::new(&tools, "run-1");
        let err = dispatcher.dispatch(&call("missing")).unwrap_err();
        assert!(matches!(err, AncoraError::ToolNotFound(name) if name == "missing"));
    }

    #[test]
    fn dispatch_callback_error_code_becomes_error_result_not_err() {
        let mut registry = ToolRegistry::new();
        registry.register("boom", failing_callback as AncorToolCallback);
        let tools = Mutex::new(registry);
        let dispatcher = FfiToolDispatcher::new(&tools, "run-1");

        let result = dispatcher.dispatch(&call("boom")).unwrap();
        assert!(result.is_error);
    }

    /// End-to-end: an `Agent::run_loop` where the model requests a tool
    /// call, `FfiToolDispatcher` invokes the real registered callback, the
    /// result feeds back into the next model call, and the loop terminates
    /// with the model's follow-up text. Exercises the exact composition
    /// `InnerRun::execute` relies on.
    #[test]
    fn agent_run_loop_with_ffi_dispatcher_completes_after_real_tool_call() {
        use std::sync::atomic::{AtomicU32, Ordering};
        use std::sync::Arc;

        use ancora_core::agent::{Agent, ModelClient};
        use ancora_core::error::AncoraError;
        use ancora_core::journal::MemoryStore;
        use ancora_proto::ancora::content_block::Block;
        use ancora_proto::ancora::{
            AgentSpec, ContentBlock, Message as ProtoMessage, Role, TextContent,
        };

        struct RequestsToolThenAnswers {
            calls: AtomicU32,
        }
        impl ModelClient for RequestsToolThenAnswers {
            fn complete(
                &self,
                messages: &[ProtoMessage],
                _spec: &AgentSpec,
            ) -> Result<ProtoMessage, AncoraError> {
                let n = self.calls.fetch_add(1, Ordering::SeqCst);
                if n == 0 {
                    Ok(ProtoMessage {
                        id: String::new(),
                        role: Role::Assistant as i32,
                        content: vec![ContentBlock {
                            block: Some(Block::ToolCall(ToolCallContent {
                                tool_call_id: "tc-1".into(),
                                tool_name: "echo".into(),
                                arguments_json: r#"{"q":"ping"}"#.into(),
                            })),
                        }],
                        created_at_ns: 0,
                        usage: None,
                        cost: None,
                        model_id: String::new(),
                    })
                } else {
                    let saw_tool_result = messages.iter().any(|m| {
                        m.content
                            .iter()
                            .any(|b| matches!(&b.block, Some(Block::ToolResult(_))))
                    });
                    assert!(
                        saw_tool_result,
                        "second model call must see the tool result in history"
                    );
                    Ok(ProtoMessage {
                        id: String::new(),
                        role: Role::Assistant as i32,
                        content: vec![ContentBlock {
                            block: Some(Block::Text(TextContent {
                                text: "done".into(),
                            })),
                        }],
                        created_at_ns: 0,
                        usage: None,
                        cost: None,
                        model_id: String::new(),
                    })
                }
            }
        }

        let mut registry = ToolRegistry::new();
        registry.register("echo", echo_callback as AncorToolCallback);
        let tools = Mutex::new(registry);
        let dispatcher = FfiToolDispatcher::new(&tools, "run-tool-loop");
        let model = RequestsToolThenAnswers {
            calls: AtomicU32::new(0),
        };
        let spec = AgentSpec {
            max_steps: 10,
            ..Default::default()
        };
        let mut agent = Agent::new(spec, "run-tool-loop", Arc::new(MemoryStore::new()));

        let output = agent.run_loop("go", &model, &dispatcher).unwrap();
        assert_eq!(output, "done");

        let events = dispatcher.into_events();
        assert_eq!(events.len(), 1);
        assert!(events[0].contains("tool_call"));
        assert!(events[0].contains("echo"));
    }
}
