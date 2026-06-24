/// All generated protobuf types plus their pbjson serde implementations.
pub mod ancora {
    include!(concat!(env!("OUT_DIR"), "/ancora.rs"));
    include!(concat!(env!("OUT_DIR"), "/ancora.serde.rs"));
}

#[cfg(test)]
mod tests {
    use prost::Message;

    use super::ancora::{
        content_block::Block, image_content::Source as ImageSource, journal_event::Event,
        ActivityRecordedEvent, AgentSpec, AudioContent, ContentBlock, DocumentContent, EffectClass,
        ErrorEvent, HumanDecisionReceivedEvent, HumanDecisionRequestedEvent, ImageContent,
        JournalEvent, Message as AncMsg, NodeEnteredEvent, NodeExitedEvent, Ping, Pong,
        RetryPolicy, RetryScheduledEvent, Role, RunCancelledEvent, RunCompletedEvent,
        RunStartedEvent, TextContent, TokenUsage, ToolCallContent, ToolResultContent, ToolSpec,
    };

    #[test]
    fn ping_round_trip() {
        let original = Ping {
            id: "test-ping-1".to_string(),
        };
        let encoded = original.encode_to_vec();
        let decoded = Ping::decode(encoded.as_slice()).expect("decode Ping");
        assert_eq!(original, decoded);
    }

    #[test]
    fn pong_round_trip() {
        let original = Pong {
            id: "test-pong-1".to_string(),
        };
        let encoded = original.encode_to_vec();
        let decoded = Pong::decode(encoded.as_slice()).expect("decode Pong");
        assert_eq!(original, decoded);
    }

    #[test]
    fn message_with_text_round_trip() {
        let msg = AncMsg {
            id: "msg-1".to_string(),
            role: Role::Assistant as i32,
            content: vec![ContentBlock {
                block: Some(Block::Text(TextContent {
                    text: "Hello, world!".to_string(),
                })),
            }],
            created_at_ns: 1_000_000,
            usage: Some(TokenUsage {
                input_tokens: 10,
                output_tokens: 5,
                cache_read_tokens: 0,
                cache_write_tokens: 0,
            }),
            cost: None,
            model_id: "test-model".to_string(),
        };
        let encoded = msg.encode_to_vec();
        let decoded = AncMsg::decode(encoded.as_slice()).expect("decode Message");
        assert_eq!(msg, decoded);
    }

    #[test]
    fn tool_call_and_result_round_trip() {
        let call = ContentBlock {
            block: Some(Block::ToolCall(ToolCallContent {
                tool_call_id: "tc-1".to_string(),
                tool_name: "search".to_string(),
                arguments_json: r#"{"q":"rust"}"#.to_string(),
            })),
        };
        let result = ContentBlock {
            block: Some(Block::ToolResult(ToolResultContent {
                tool_call_id: "tc-1".to_string(),
                result_json: r#"{"hits":42}"#.to_string(),
                is_error: false,
            })),
        };
        for block in [&call, &result] {
            let encoded = block.encode_to_vec();
            let decoded = ContentBlock::decode(encoded.as_slice()).expect("decode ContentBlock");
            assert_eq!(block, &decoded);
        }
    }

    #[test]
    fn image_content_round_trip() {
        let block = ContentBlock {
            block: Some(Block::Image(ImageContent {
                source: Some(ImageSource::Url("https://example.com/img.png".to_string())),
                media_type: "image/png".to_string(),
            })),
        };
        let encoded = block.encode_to_vec();
        let decoded = ContentBlock::decode(encoded.as_slice()).expect("decode image block");
        assert_eq!(block, decoded);
    }

    #[test]
    fn audio_content_round_trip() {
        let block = ContentBlock {
            block: Some(Block::Audio(AudioContent {
                source: Some(super::ancora::audio_content::Source::InlineBase64(
                    "dGVzdA==".to_string(),
                )),
                media_type: "audio/wav".to_string(),
            })),
        };
        let encoded = block.encode_to_vec();
        let decoded = ContentBlock::decode(encoded.as_slice()).expect("decode audio block");
        assert_eq!(block, decoded);
    }

    #[test]
    fn document_content_round_trip() {
        let block = ContentBlock {
            block: Some(Block::Document(DocumentContent {
                source: Some(super::ancora::document_content::Source::InlineBase64(
                    "cGRm".to_string(),
                )),
                media_type: "application/pdf".to_string(),
                filename: "report.pdf".to_string(),
            })),
        };
        let encoded = block.encode_to_vec();
        let decoded = ContentBlock::decode(encoded.as_slice()).expect("decode document block");
        assert_eq!(block, decoded);
    }

    #[test]
    fn token_usage_round_trip() {
        let usage = TokenUsage {
            input_tokens: 100,
            output_tokens: 200,
            cache_read_tokens: 50,
            cache_write_tokens: 25,
        };
        let encoded = usage.encode_to_vec();
        let decoded = TokenUsage::decode(encoded.as_slice()).expect("decode TokenUsage");
        assert_eq!(usage, decoded);
    }

    #[test]
    fn agent_spec_round_trip() {
        let spec = AgentSpec {
            name: "researcher".to_string(),
            model_id: "llama3".to_string(),
            instructions: "You are a research assistant.".to_string(),
            output_schema_json: r#"{"type":"object"}"#.to_string(),
            tools: vec![ToolSpec {
                name: "web_search".to_string(),
                description: "Search the web".to_string(),
                input_schema_json: r#"{"type":"object","properties":{"q":{"type":"string"}}}"#
                    .to_string(),
                output_schema_json: r#"{"type":"string"}"#.to_string(),
                effect_class: EffectClass::EffectRead as i32,
                idempotency_key_template: "".to_string(),
            }],
            max_steps: 10,
            model_retry: Some(RetryPolicy {
                max_attempts: 3,
                initial_backoff_ms: 100,
                max_backoff_ms: 5000,
                jitter: 0.1,
            }),
            model_params_json: r#"{"temperature":0.7}"#.to_string(),
        };
        let encoded = spec.encode_to_vec();
        let decoded = AgentSpec::decode(encoded.as_slice()).expect("decode AgentSpec");
        assert_eq!(spec, decoded);
    }

    #[test]
    fn tool_spec_write_effect_round_trip() {
        let tool = ToolSpec {
            name: "send_email".to_string(),
            description: "Sends an email".to_string(),
            input_schema_json: r#"{"type":"object"}"#.to_string(),
            output_schema_json: r#"{"type":"null"}"#.to_string(),
            effect_class: EffectClass::EffectWrite as i32,
            idempotency_key_template: "{run_id}-{node_id}-{seq}".to_string(),
        };
        let encoded = tool.encode_to_vec();
        let decoded = ToolSpec::decode(encoded.as_slice()).expect("decode ToolSpec");
        assert_eq!(tool, decoded);
        assert_eq!(decoded.effect_class, EffectClass::EffectWrite as i32);
        assert!(!decoded.idempotency_key_template.is_empty());
    }

    #[test]
    fn journal_event_ordering_and_round_trip() {
        let events = [
            JournalEvent {
                event_id: "evt-1".to_string(),
                run_id: "run-abc".to_string(),
                seq: 0,
                recorded_at_ns: 1_000,
                event: Some(Event::RunStarted(RunStartedEvent {
                    run_id: "run-abc".to_string(),
                    spec_bytes: b"spec".to_vec(),
                    spec_type: "AgentSpec".to_string(),
                })),
            },
            JournalEvent {
                event_id: "evt-2".to_string(),
                run_id: "run-abc".to_string(),
                seq: 1,
                recorded_at_ns: 2_000,
                event: Some(Event::NodeEntered(NodeEnteredEvent {
                    node_id: "node-1".to_string(),
                    node_kind: "agent".to_string(),
                })),
            },
            JournalEvent {
                event_id: "evt-3".to_string(),
                run_id: "run-abc".to_string(),
                seq: 2,
                recorded_at_ns: 3_000,
                event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                    activity_key: "run-abc-node-1-0".to_string(),
                    activity_kind: "model_call".to_string(),
                    input_json: r#"{"messages":[]}"#.to_string(),
                    result_json: r#"{"text":"hello"}"#.to_string(),
                    replayed: false,
                })),
            },
        ];

        let mut prev_seq = 0u64;
        for (i, event) in events.iter().enumerate() {
            let encoded = event.encode_to_vec();
            let decoded = JournalEvent::decode(encoded.as_slice())
                .unwrap_or_else(|_| panic!("decode event {i}"));
            assert_eq!(event, &decoded);
            if i > 0 {
                assert!(
                    decoded.seq > prev_seq,
                    "seq must be monotonically increasing"
                );
            }
            prev_seq = decoded.seq;
        }
    }

    #[test]
    fn journal_all_event_variants_round_trip() {
        let variants: Vec<Event> = vec![
            Event::NodeExited(NodeExitedEvent {
                node_id: "n1".to_string(),
                success: true,
            }),
            Event::HumanDecisionRequested(HumanDecisionRequestedEvent {
                prompt: "Approve?".to_string(),
                options: vec!["yes".to_string(), "no".to_string()],
                timeout_at_ns: 0,
            }),
            Event::HumanDecisionReceived(HumanDecisionReceivedEvent {
                decision: "yes".to_string(),
            }),
            Event::RunCompleted(RunCompletedEvent {
                output_json: r#"{"answer":42}"#.to_string(),
            }),
            Event::Error(ErrorEvent {
                code: "ERR_MODEL".to_string(),
                message: "model timeout".to_string(),
                detail: "".to_string(),
            }),
            Event::RetryScheduled(RetryScheduledEvent {
                target_id: "node-1".to_string(),
                attempt: 2,
                delay_ms: 500,
            }),
            Event::RunCancelled(RunCancelledEvent {
                reason: "user request".to_string(),
            }),
        ];

        for (i, variant) in variants.into_iter().enumerate() {
            let env = JournalEvent {
                event_id: format!("evt-{i}"),
                run_id: "run-x".to_string(),
                seq: i as u64,
                recorded_at_ns: i as i64 * 1000,
                event: Some(variant),
            };
            let encoded = env.encode_to_vec();
            let decoded = JournalEvent::decode(encoded.as_slice())
                .unwrap_or_else(|_| panic!("decode variant {i}"));
            assert_eq!(env, decoded);
        }
    }

    // ---- JSON serde equivalence tests (Phase 07) ----

    #[test]
    fn proto_json_message_equivalence() {
        let msg = AncMsg {
            id: "msg-json-1".to_string(),
            role: Role::User as i32,
            content: vec![ContentBlock {
                block: Some(Block::Text(TextContent {
                    text: "Hello JSON".to_string(),
                })),
            }],
            created_at_ns: 0,
            usage: None,
            cost: None,
            model_id: "".to_string(),
        };
        // Encode to proto binary and back.
        let proto_bytes = msg.encode_to_vec();
        let from_proto = AncMsg::decode(proto_bytes.as_slice()).expect("proto decode");
        // Encode to JSON and back via serde_json.
        let json = serde_json::to_string(&msg).expect("json serialize");
        let from_json: AncMsg = serde_json::from_str(&json).expect("json deserialize");
        assert_eq!(from_proto, from_json);
    }

    #[test]
    fn proto_json_journal_event_equivalence() {
        let event = JournalEvent {
            event_id: "e1".to_string(),
            run_id: "r1".to_string(),
            seq: 0,
            recorded_at_ns: 0,
            event: Some(Event::RunStarted(RunStartedEvent {
                run_id: "r1".to_string(),
                spec_bytes: vec![],
                spec_type: "AgentSpec".to_string(),
            })),
        };
        let proto_bytes = event.encode_to_vec();
        let from_proto = JournalEvent::decode(proto_bytes.as_slice()).expect("proto decode");
        let json = serde_json::to_string(&event).expect("json serialize");
        let from_json: JournalEvent = serde_json::from_str(&json).expect("json deserialize");
        assert_eq!(from_proto, from_json);
    }

    #[test]
    fn proto_json_agent_spec_equivalence() {
        let spec = AgentSpec {
            name: "test-agent".to_string(),
            model_id: "local-model".to_string(),
            instructions: "Be helpful.".to_string(),
            output_schema_json: "".to_string(),
            tools: vec![],
            max_steps: 5,
            model_retry: None,
            model_params_json: "".to_string(),
        };
        let proto_bytes = spec.encode_to_vec();
        let from_proto = AgentSpec::decode(proto_bytes.as_slice()).expect("proto decode");
        let json = serde_json::to_string(&spec).expect("json serialize");
        let from_json: AgentSpec = serde_json::from_str(&json).expect("json deserialize");
        assert_eq!(from_proto, from_json);
    }
}
