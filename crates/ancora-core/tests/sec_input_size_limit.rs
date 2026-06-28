// Security: input size limit -- reject oversized payloads before processing.

const MAX_INPUT_BYTES: usize = 1_048_576; // 1 MiB
const MAX_TOOL_RESULT_BYTES: usize = 65_536; // 64 KiB

fn check_input_size(payload: &[u8]) -> Result<(), String> {
    if payload.len() > MAX_INPUT_BYTES {
        Err(format!("input too large: {} bytes (max {})", payload.len(), MAX_INPUT_BYTES))
    } else {
        Ok(())
    }
}

fn check_tool_result_size(payload: &[u8]) -> Result<(), String> {
    if payload.len() > MAX_TOOL_RESULT_BYTES {
        Err(format!("tool result too large: {} bytes (max {})", payload.len(), MAX_TOOL_RESULT_BYTES))
    } else {
        Ok(())
    }
}

#[test]
fn test_small_input_accepted() {
    assert!(check_input_size(b"hello world").is_ok());
}

#[test]
fn test_input_at_limit_accepted() {
    let payload = vec![b'x'; MAX_INPUT_BYTES];
    assert!(check_input_size(&payload).is_ok());
}

#[test]
fn test_input_over_limit_rejected() {
    let payload = vec![b'x'; MAX_INPUT_BYTES + 1];
    let r = check_input_size(&payload);
    assert!(r.is_err());
    assert!(r.unwrap_err().contains("too large"));
}

#[test]
fn test_tool_result_at_limit_accepted() {
    let payload = vec![b'y'; MAX_TOOL_RESULT_BYTES];
    assert!(check_tool_result_size(&payload).is_ok());
}

#[test]
fn test_tool_result_over_limit_rejected() {
    let payload = vec![b'y'; MAX_TOOL_RESULT_BYTES + 1];
    assert!(check_tool_result_size(&payload).is_err());
}

#[test]
fn test_error_message_includes_sizes() {
    let payload = vec![b'z'; MAX_INPUT_BYTES + 100];
    let err = check_input_size(&payload).unwrap_err();
    assert!(err.contains(&MAX_INPUT_BYTES.to_string()));
}
