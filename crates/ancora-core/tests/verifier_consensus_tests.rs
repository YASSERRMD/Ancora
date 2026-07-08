use ancora_core::error::AncoraError;
use ancora_core::executor::{VerifierNode, VerifierResult};
use ancora_core::graph::{Node, NodeKind, NodeSpec};
use ancora_proto::ancora::AgentSpec as ProtoSpec;

fn agent_node(id: &str) -> Node {
    Node {
        id: id.to_string(),
        kind: NodeKind::Agent,
        model_id: None,
        spec: NodeSpec::Agent(ProtoSpec {
            name: id.to_string(),
            model_id: "mock".to_string(),
            instructions: String::new(),
            output_schema_json: String::new(),
            tools: vec![],
            max_steps: 1,
            model_retry: None,
            model_params_json: String::new(),
        }),
    }
}

struct AlwaysApprove;

impl VerifierNode for AlwaysApprove {
    fn verify(&self, _node: &Node, candidate: &str) -> Result<VerifierResult, AncoraError> {
        Ok(VerifierResult::Approved {
            output: candidate.to_string(),
        })
    }
}

struct AlwaysReject;

impl VerifierNode for AlwaysReject {
    fn verify(&self, _node: &Node, candidate: &str) -> Result<VerifierResult, AncoraError> {
        Ok(VerifierResult::Rejected {
            reason: format!("rejected: {}", candidate),
        })
    }
}

/// Runs N verifiers and returns the number of approvals.
fn run_n_verifiers<V: VerifierNode>(verifier: &V, node: &Node, candidate: &str, n: usize) -> usize {
    (0..n)
        .filter(|_| {
            matches!(
                verifier.verify(node, candidate).unwrap(),
                VerifierResult::Approved { .. }
            )
        })
        .count()
}

#[test]
fn always_approve_verifier_approves_any_candidate() {
    let node = agent_node("v-node");
    let result = AlwaysApprove.verify(&node, "my-output").unwrap();
    assert!(matches!(result, VerifierResult::Approved { .. }));
}

#[test]
fn always_reject_verifier_rejects_with_reason() {
    let node = agent_node("v-node");
    let result = AlwaysReject.verify(&node, "my-output").unwrap();
    assert!(matches!(result, VerifierResult::Rejected { .. }));
    if let VerifierResult::Rejected { reason } = result {
        assert!(
            reason.contains("my-output"),
            "rejection reason must cite the candidate"
        );
    }
}

#[test]
fn majority_consensus_approves_when_most_approve() {
    let node = agent_node("v-node");
    let approvals = run_n_verifiers(&AlwaysApprove, &node, "candidate", 5);
    assert_eq!(approvals, 5);
    assert!(approvals > 5 / 2, "majority must approve");
}

#[test]
fn majority_consensus_rejects_when_most_reject() {
    let node = agent_node("v-node");
    let approvals = run_n_verifiers(&AlwaysReject, &node, "candidate", 5);
    assert_eq!(approvals, 0);
    assert!(approvals <= 5 / 2, "majority must reject");
}

#[test]
fn tie_break_at_three_verifiers_two_approve_one_reject() {
    let node = agent_node("v-node");

    let mut approve_count = 0usize;
    for verifier in [
        &AlwaysApprove as &dyn VerifierNode,
        &AlwaysApprove,
        &AlwaysReject,
    ]
    .iter()
    {
        if let VerifierResult::Approved { .. } = verifier.verify(&node, "candidate").unwrap() {
            approve_count += 1;
        }
    }

    assert_eq!(approve_count, 2, "2 of 3 approvals");
    let passes_majority = approve_count > 3 / 2;
    assert!(passes_majority, "2 of 3 is majority: should pass");
}

#[test]
fn approved_result_preserves_candidate_output() {
    let node = agent_node("v-node");
    let candidate = r#"{"result":"42"}"#;
    if let VerifierResult::Approved { output } = AlwaysApprove.verify(&node, candidate).unwrap() {
        assert_eq!(output, candidate);
    } else {
        panic!("expected Approved");
    }
}

#[test]
fn verifier_reject_path_carries_reason() {
    let node = agent_node("v-node");
    if let VerifierResult::Rejected { reason } = AlwaysReject.verify(&node, "bad").unwrap() {
        assert!(!reason.is_empty(), "rejection reason must be non-empty");
    } else {
        panic!("expected Rejected");
    }
}
