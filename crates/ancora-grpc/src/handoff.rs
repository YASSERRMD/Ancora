use crate::agent_card::AgentCard;
use crate::client::A2aClient;
use crate::task::Task;

/// A handoff request from one agent to another over A2A.
#[derive(Debug, Clone)]
pub struct HandoffRequest {
    /// The HTTP base URL of the remote agent (used to fetch the agent card).
    pub agent_url: String,
    /// Unique task identifier for this handoff.
    pub task_id: String,
    /// The input payload to send to the remote agent.
    pub input: String,
    /// When true, the caller requires the remote agent card to carry a valid
    /// Ed25519 signature before the task is submitted.
    pub require_signed_identity: bool,
}

/// Result of a completed polyglot handoff.
#[derive(Debug)]
pub struct HandoffResult {
    pub task: Task,
    pub remote_card: AgentCard,
}

/// Perform a polyglot handoff: fetch the remote agent card (optionally verify
/// its identity), then submit the task.
///
/// Returns `Ok(HandoffResult)` when the handoff was accepted.
/// Returns `Err(description)` if the card fetch, identity check, or task
/// submission fails.
pub async fn perform_handoff(req: HandoffRequest) -> Result<HandoffResult, String> {
    let client = A2aClient::from_url(&req.agent_url)
        .map_err(|e| format!("invalid agent URL: {}", e))?;

    let remote_card = if req.require_signed_identity {
        client
            .fetch_and_verify_card()
            .await
            .map_err(|e| format!("identity verification failed: {}", e))?
    } else {
        client
            .fetch_card()
            .await
            .map_err(|e| format!("card fetch failed: {}", e))?
    };

    let task = client.submit_task(&req.task_id, &req.input).await;

    Ok(HandoffResult { task, remote_card })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handoff_request_defaults_to_signed_required() {
        let req = HandoffRequest {
            agent_url: "http://localhost:8080".into(),
            task_id: "t1".into(),
            input: "hello".into(),
            require_signed_identity: true,
        };
        assert!(req.require_signed_identity);
    }

    #[test]
    fn handoff_request_can_skip_identity_check() {
        let req = HandoffRequest {
            agent_url: "http://localhost:8080".into(),
            task_id: "t2".into(),
            input: "hello".into(),
            require_signed_identity: false,
        };
        assert!(!req.require_signed_identity);
    }

    #[tokio::test]
    async fn perform_handoff_rejects_invalid_url() {
        let req = HandoffRequest {
            agent_url: "grpc://localhost:50051".into(),
            task_id: "t3".into(),
            input: "ping".into(),
            require_signed_identity: false,
        };
        let err = perform_handoff(req).await.unwrap_err();
        assert!(err.contains("invalid agent URL"), "got: {}", err);
    }
}
