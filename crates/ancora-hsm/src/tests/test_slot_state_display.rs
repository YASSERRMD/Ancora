use crate::slot::SlotState;

#[test]
fn display_empty() {
    assert_eq!(format!("{}", SlotState::Empty), "EMPTY");
}

#[test]
fn display_token_present() {
    assert_eq!(format!("{}", SlotState::TokenPresent), "TOKEN_PRESENT");
}

#[test]
fn display_token_absent() {
    assert_eq!(format!("{}", SlotState::TokenAbsent), "TOKEN_ABSENT");
}
