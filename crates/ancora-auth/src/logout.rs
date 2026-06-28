use crate::revocation::RevocationStore;
use crate::session::SessionStore;

pub struct LogoutResult {
    pub session_logged_out: bool,
    pub token_revoked: bool,
}

pub fn logout_session(
    sessions: &mut SessionStore,
    revocations: &mut RevocationStore,
    session_id: &str,
) -> LogoutResult {
    let token_raw = sessions
        .get(session_id)
        .map(|s| s.token_raw.clone());

    let session_logged_out = sessions.logout(session_id);

    let token_revoked = if let Some(raw) = token_raw {
        revocations.revoke(raw);
        true
    } else {
        false
    };

    LogoutResult {
        session_logged_out,
        token_revoked,
    }
}

pub fn logout_all_for_subject(
    sessions: &mut SessionStore,
    revocations: &mut RevocationStore,
    subject: &str,
) -> usize {
    let session_ids: Vec<String> = sessions
        .sessions_for_subject(subject)
        .into_iter()
        .map(|s| s.session_id.clone())
        .collect();

    let count = session_ids.len();
    for sid in session_ids {
        logout_session(sessions, revocations, &sid);
    }
    count
}
