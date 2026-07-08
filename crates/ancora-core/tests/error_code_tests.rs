use ancora_core::error::AncoraError;
use ancora_proto::ancora::ErrorCode;

fn error_code_round_trips(err: AncoraError) -> bool {
    let code = err.error_code();
    let reconstructed = AncoraError::from((code, "msg".to_string()));
    reconstructed.error_code() as i32 == code as i32
}

#[test]
fn nondeterminism_error_code_round_trips() {
    assert!(error_code_round_trips(AncoraError::Nondeterminism {
        seq: 1,
        expected: "a".into(),
        got: "b".into()
    }));
}

#[test]
fn journal_write_error_code_round_trips() {
    assert!(error_code_round_trips(AncoraError::JournalWrite(
        "write fail".into()
    )));
}

#[test]
fn max_steps_error_code_round_trips() {
    assert!(error_code_round_trips(AncoraError::MaxSteps {
        max_steps: 10
    }));
}

#[test]
fn policy_residency_error_code_round_trips() {
    assert!(error_code_round_trips(AncoraError::PolicyResidency(
        "eu-only".into()
    )));
}

#[test]
fn tool_not_found_error_code_round_trips() {
    assert!(error_code_round_trips(AncoraError::ToolNotFound(
        "unknown".into()
    )));
}

#[test]
fn graph_invalid_error_code_round_trips() {
    assert!(error_code_round_trips(AncoraError::GraphInvalid(
        "cycle".into()
    )));
}

#[test]
fn cancelled_error_code_round_trips() {
    assert!(error_code_round_trips(AncoraError::Cancelled(
        "by user".into()
    )));
}

#[test]
fn storage_error_code_round_trips() {
    assert!(error_code_round_trips(AncoraError::Storage(
        "disk full".into()
    )));
}

#[test]
fn internal_error_code_round_trips() {
    assert!(error_code_round_trips(AncoraError::Internal("bug".into())));
}

#[test]
fn error_unspecified_maps_to_internal() {
    let err = AncoraError::from((ErrorCode::ErrorUnspecified, "fallback".into()));
    assert!(matches!(err, AncoraError::Internal(_)));
}

#[test]
fn all_display_messages_are_non_empty() {
    let errors: Vec<AncoraError> = vec![
        AncoraError::Nondeterminism {
            seq: 0,
            expected: "x".into(),
            got: "y".into(),
        },
        AncoraError::JournalGap { seq: 0 },
        AncoraError::JournalWrite("e".into()),
        AncoraError::MaxSteps { max_steps: 1 },
        AncoraError::OutputValidation {
            attempts: 1,
            reason: "r".into(),
        },
        AncoraError::Timeout { timeout_ms: 100 },
        AncoraError::ModelRefused("r".into()),
        AncoraError::ModelHttp {
            status: 500,
            body: "b".into(),
        },
        AncoraError::ModelParse("p".into()),
        AncoraError::ModelUnreachable("u".into()),
        AncoraError::ToolFailed {
            name: "t".into(),
            message: "m".into(),
        },
        AncoraError::ToolNotFound("t".into()),
        AncoraError::ToolInputInvalid {
            name: "t".into(),
            reason: "r".into(),
        },
        AncoraError::ToolDenied("t".into()),
        AncoraError::PolicyResidency("p".into()),
        AncoraError::PolicyPermission("p".into()),
        AncoraError::GraphInvalid("g".into()),
        AncoraError::NodeNotFound("n".into()),
        AncoraError::Cancelled("c".into()),
        AncoraError::InvalidState("s".into()),
        AncoraError::Storage("s".into()),
        AncoraError::Internal("i".into()),
    ];

    for err in errors {
        let msg = err.to_string();
        assert!(
            !msg.is_empty(),
            "display message must be non-empty for {:?}",
            err
        );
    }
}
