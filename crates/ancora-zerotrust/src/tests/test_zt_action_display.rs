use crate::audit::ZtAction;

#[test]
fn action_display() {
    assert_eq!(format!("{}", ZtAction::AccessGranted), "ACCESS_GRANTED");
    assert_eq!(format!("{}", ZtAction::AccessDenied), "ACCESS_DENIED");
    assert_eq!(format!("{}", ZtAction::MfaRequired), "MFA_REQUIRED");
    assert_eq!(format!("{}", ZtAction::SessionCreated), "SESSION_CREATED");
    assert_eq!(format!("{}", ZtAction::SessionRevoked), "SESSION_REVOKED");
    assert_eq!(format!("{}", ZtAction::DevicePostureChecked), "DEVICE_POSTURE_CHECKED");
    assert_eq!(format!("{}", ZtAction::PolicyEvaluated), "POLICY_EVALUATED");
}
