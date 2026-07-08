/// Failure clustering - group similar failure reasons together.
///
/// Uses a simple token-overlap similarity to cluster failure messages.
use std::collections::HashMap;

/// A single failure observation.
#[derive(Debug, Clone)]
pub struct Failure {
    pub case_id: String,
    pub reason: String,
}

/// A cluster of similar failures.
#[derive(Debug, Clone)]
pub struct FailureCluster {
    pub label: String,
    pub count: usize,
    pub members: Vec<Failure>,
}

/// Cluster failures by token-overlap similarity.
///
/// Two messages are in the same cluster when they share at least
/// `min_shared_tokens` tokens (whitespace-split words).
pub fn cluster_failures(failures: &[Failure], min_shared_tokens: usize) -> Vec<FailureCluster> {
    let mut clusters: Vec<FailureCluster> = Vec::new();

    for failure in failures {
        let tokens = tokenize(&failure.reason);
        let best = clusters.iter_mut().find(|c| {
            let label_tokens = tokenize(&c.label);
            shared_tokens(&tokens, &label_tokens) >= min_shared_tokens
        });

        if let Some(cluster) = best {
            cluster.members.push(failure.clone());
            cluster.count += 1;
        } else {
            clusters.push(FailureCluster {
                label: failure.reason.clone(),
                count: 1,
                members: vec![failure.clone()],
            });
        }
    }

    // Sort by count descending.
    clusters.sort_by(|a, b| b.count.cmp(&a.count));
    clusters
}

/// Tokenize a string into lowercase words.
fn tokenize(s: &str) -> Vec<String> {
    s.split_whitespace()
        .map(|w| {
            w.to_lowercase()
                .trim_matches(|c: char| !c.is_alphanumeric())
                .to_string()
        })
        .filter(|w| !w.is_empty())
        .collect()
}

/// Count shared tokens between two token lists (set intersection size).
fn shared_tokens(a: &[String], b: &[String]) -> usize {
    let mut counts_a: HashMap<&str, usize> = HashMap::new();
    for t in a {
        *counts_a.entry(t.as_str()).or_insert(0) += 1;
    }
    let mut shared = 0usize;
    for t in b {
        if let Some(cnt) = counts_a.get_mut(t.as_str()) {
            if *cnt > 0 {
                shared += 1;
                *cnt -= 1;
            }
        }
    }
    shared
}

/// Collect all failures from a breakdown slice.
pub fn collect_failures(breakdowns: &[crate::breakdown::CaseBreakdown]) -> Vec<Failure> {
    breakdowns
        .iter()
        .flat_map(|b| {
            b.fail_reasons.iter().map(|r| Failure {
                case_id: b.case_id.clone(),
                reason: r.clone(),
            })
        })
        .collect()
}
