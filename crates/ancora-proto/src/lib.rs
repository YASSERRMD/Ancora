pub mod ancora {
    include!(concat!(env!("OUT_DIR"), "/ancora.rs"));
}

pub mod messages {
    include!(concat!(env!("OUT_DIR"), "/ancora.rs"));
}

#[cfg(test)]
mod tests {
    use prost::Message;

    use super::ancora::{Ping, Pong};
    use super::messages::{
        content_block::Block, image_content::Source as ImageSource, AudioContent, ContentBlock,
        DocumentContent, ImageContent, Message as AncMsg, Role, TextContent, TokenUsage,
        ToolCallContent, ToolResultContent,
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
                source: Some(super::messages::audio_content::Source::InlineBase64(
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
                source: Some(super::messages::document_content::Source::InlineBase64(
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
}
