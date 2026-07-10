use std::time::Duration;

use crate::client::ModelClient;
use crate::error::InferenceError;
use crate::types::{CompletionRequest, CompletionResponse, TokenEvent};

/// Bounds retry/backoff behavior for a `RetryingModelClient`.
#[derive(Debug, Clone, Copy)]
pub struct RetryPolicy {
    /// Total attempts including the first call. 1 disables retrying.
    pub max_attempts: u32,
    pub initial_backoff_ms: u64,
    pub max_backoff_ms: u64,
    /// Add up to 50% random jitter on top of the computed backoff, to avoid
    /// a thundering herd of clients retrying in lockstep.
    pub jitter: bool,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_backoff_ms: 200,
            max_backoff_ms: 10_000,
            jitter: true,
        }
    }
}

impl RetryPolicy {
    /// Exponential backoff for the given zero-based retry attempt (0 = the
    /// delay before the second overall attempt), capped at `max_backoff_ms`.
    pub fn backoff_ms(&self, attempt: u32) -> u64 {
        let exp = self
            .initial_backoff_ms
            .saturating_mul(1u64 << attempt.min(20));
        exp.min(self.max_backoff_ms)
    }

    fn jittered(&self, base_ms: u64, attempt: u32) -> u64 {
        if !self.jitter || base_ms == 0 {
            return base_ms;
        }
        // A tiny deterministic PRNG (xorshift) seeded from the attempt
        // number, not real randomness: the goal is spreading concurrent
        // clients apart, not cryptographic unpredictability, and this keeps
        // the crate free of an external rand dependency.
        let mut x = (attempt as u64).wrapping_add(0x9E3779B97F4A7C15) | 1;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        let fraction = (x % 1000) as f64 / 1000.0; // 0.0..1.0
        let jittered = base_ms as f64 * (1.0 + 0.5 * fraction);
        jittered.round() as u64
    }
}

/// Returns the delay to wait before retrying, or `None` if `err` should not
/// be retried at all.
fn retry_delay(err: &InferenceError, policy: &RetryPolicy, attempt: u32) -> Option<Duration> {
    match err {
        InferenceError::RateLimit { retry_after } => {
            let base_ms = retry_after
                .map(|d| d.as_millis() as u64)
                .unwrap_or_else(|| policy.backoff_ms(attempt));
            Some(Duration::from_millis(base_ms.min(policy.max_backoff_ms)))
        }
        InferenceError::Unreachable(_) => Some(Duration::from_millis(
            policy.jittered(policy.backoff_ms(attempt), attempt),
        )),
        InferenceError::Http { status, .. } if *status >= 500 => Some(Duration::from_millis(
            policy.jittered(policy.backoff_ms(attempt), attempt),
        )),
        // Refused, Parse, AuthRejected, MissingCredential, and 4xx (other
        // than 429) are not transient: retrying won't change the outcome.
        _ => None,
    }
}

/// Wraps any `ModelClient` with retry/backoff on transient failures
/// (429 with `Retry-After`, 5xx, and unreachable-endpoint errors).
/// Auth failures, parse errors, and other 4xx responses are never retried.
pub struct RetryingModelClient<C> {
    inner: C,
    policy: RetryPolicy,
}

impl<C: ModelClient> RetryingModelClient<C> {
    pub fn new(inner: C, policy: RetryPolicy) -> Self {
        Self { inner, policy }
    }

    fn run_with_retry<T>(
        &self,
        mut op: impl FnMut() -> Result<T, InferenceError>,
    ) -> Result<T, InferenceError> {
        let mut attempt = 0;
        loop {
            match op() {
                Ok(v) => return Ok(v),
                Err(e) => {
                    let is_last = attempt + 1 >= self.policy.max_attempts;
                    match retry_delay(&e, &self.policy, attempt) {
                        Some(delay) if !is_last => {
                            std::thread::sleep(delay);
                            attempt += 1;
                        }
                        _ => return Err(e),
                    }
                }
            }
        }
    }
}

impl<C: ModelClient> ModelClient for RetryingModelClient<C> {
    fn complete(&self, request: &CompletionRequest) -> Result<CompletionResponse, InferenceError> {
        self.run_with_retry(|| self.inner.complete(request))
    }

    /// Retries the whole streamed call on a transient failure. Because
    /// streaming already delivers tokens to `on_token` as they arrive, a
    /// retry after a partial stream re-invokes `on_token` for the retried
    /// attempt's tokens too -- callers driving UI state from token events
    /// should expect possible duplicates across a retry, not just at the
    /// final result.
    fn stream_complete(
        &self,
        request: &CompletionRequest,
        on_token: &mut dyn FnMut(TokenEvent),
    ) -> Result<CompletionResponse, InferenceError> {
        self.run_with_retry(|| self.inner.stream_complete(request, on_token))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Mutex;

    use super::*;
    use crate::types::Message;

    fn req() -> CompletionRequest {
        CompletionRequest::simple("m", vec![Message::text("user", "hi")])
    }

    fn fast_policy(max_attempts: u32) -> RetryPolicy {
        RetryPolicy {
            max_attempts,
            initial_backoff_ms: 1,
            max_backoff_ms: 5,
            jitter: false,
        }
    }

    struct ScriptedClient {
        // Each call pops the next result from the front of the queue.
        results: Mutex<Vec<Result<CompletionResponse, InferenceError>>>,
        calls: AtomicU32,
    }

    impl ScriptedClient {
        fn new(results: Vec<Result<CompletionResponse, InferenceError>>) -> Self {
            Self {
                results: Mutex::new(results),
                calls: AtomicU32::new(0),
            }
        }
    }

    impl ModelClient for ScriptedClient {
        fn complete(
            &self,
            _request: &CompletionRequest,
        ) -> Result<CompletionResponse, InferenceError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            let mut results = self.results.lock().unwrap();
            if results.is_empty() {
                panic!("ScriptedClient ran out of scripted results");
            }
            results.remove(0)
        }
    }

    fn ok_response(text: &str) -> CompletionResponse {
        CompletionResponse {
            content: text.to_owned(),
            tokens_in: 1,
            tokens_out: 1,
            cost_usd: None,
            tool_calls: vec![],
        }
    }

    #[test]
    fn succeeds_immediately_without_retrying() {
        let inner = ScriptedClient::new(vec![Ok(ok_response("ok"))]);
        let client = RetryingModelClient::new(inner, fast_policy(3));
        let resp = client.complete(&req()).unwrap();
        assert_eq!(resp.content, "ok");
        assert_eq!(client.inner.calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn retries_on_5xx_then_succeeds() {
        let inner = ScriptedClient::new(vec![
            Err(InferenceError::Http {
                status: 503,
                body: "unavailable".into(),
            }),
            Ok(ok_response("recovered")),
        ]);
        let client = RetryingModelClient::new(inner, fast_policy(3));
        let resp = client.complete(&req()).unwrap();
        assert_eq!(resp.content, "recovered");
        assert_eq!(client.inner.calls.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn retries_on_rate_limit_then_succeeds() {
        let inner = ScriptedClient::new(vec![
            Err(InferenceError::RateLimit { retry_after: None }),
            Ok(ok_response("recovered")),
        ]);
        let client = RetryingModelClient::new(inner, fast_policy(3));
        let resp = client.complete(&req()).unwrap();
        assert_eq!(resp.content, "recovered");
    }

    #[test]
    fn retries_on_unreachable_then_succeeds() {
        let inner = ScriptedClient::new(vec![
            Err(InferenceError::Unreachable("connection refused".into())),
            Ok(ok_response("recovered")),
        ]);
        let client = RetryingModelClient::new(inner, fast_policy(3));
        let resp = client.complete(&req()).unwrap();
        assert_eq!(resp.content, "recovered");
    }

    #[test]
    fn does_not_retry_auth_rejected() {
        let inner = ScriptedClient::new(vec![Err(InferenceError::AuthRejected("bad key".into()))]);
        let client = RetryingModelClient::new(inner, fast_policy(5));
        let err = client.complete(&req()).unwrap_err();
        assert!(matches!(err, InferenceError::AuthRejected(_)));
        assert_eq!(client.inner.calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn does_not_retry_client_4xx_other_than_429() {
        let inner = ScriptedClient::new(vec![Err(InferenceError::Http {
            status: 400,
            body: "bad request".into(),
        })]);
        let client = RetryingModelClient::new(inner, fast_policy(5));
        let err = client.complete(&req()).unwrap_err();
        assert!(matches!(err, InferenceError::Http { status: 400, .. }));
        assert_eq!(client.inner.calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn does_not_retry_parse_errors() {
        let inner = ScriptedClient::new(vec![Err(InferenceError::Parse("bad json".into()))]);
        let client = RetryingModelClient::new(inner, fast_policy(5));
        let err = client.complete(&req()).unwrap_err();
        assert!(matches!(err, InferenceError::Parse(_)));
        assert_eq!(client.inner.calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn stops_after_max_attempts_and_returns_last_error() {
        let inner = ScriptedClient::new(vec![
            Err(InferenceError::Http {
                status: 500,
                body: "e1".into(),
            }),
            Err(InferenceError::Http {
                status: 500,
                body: "e2".into(),
            }),
            Err(InferenceError::Http {
                status: 500,
                body: "e3".into(),
            }),
        ]);
        let client = RetryingModelClient::new(inner, fast_policy(3));
        let err = client.complete(&req()).unwrap_err();
        assert!(matches!(err, InferenceError::Http { body, .. } if body == "e3"));
        assert_eq!(client.inner.calls.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn max_attempts_one_disables_retrying() {
        let inner = ScriptedClient::new(vec![Err(InferenceError::Http {
            status: 500,
            body: "e1".into(),
        })]);
        let client = RetryingModelClient::new(inner, fast_policy(1));
        client.complete(&req()).unwrap_err();
        assert_eq!(client.inner.calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn honors_retry_after_hint_from_rate_limit() {
        let policy = RetryPolicy {
            max_attempts: 2,
            initial_backoff_ms: 1,
            max_backoff_ms: 10_000,
            jitter: false,
        };
        let delay = retry_delay(
            &InferenceError::RateLimit {
                retry_after: Some(Duration::from_millis(7)),
            },
            &policy,
            0,
        )
        .unwrap();
        assert_eq!(delay, Duration::from_millis(7));
    }

    #[test]
    fn backoff_ms_grows_exponentially_and_caps() {
        let policy = RetryPolicy {
            max_attempts: 10,
            initial_backoff_ms: 100,
            max_backoff_ms: 1_000,
            jitter: false,
        };
        assert_eq!(policy.backoff_ms(0), 100);
        assert_eq!(policy.backoff_ms(1), 200);
        assert_eq!(policy.backoff_ms(2), 400);
        assert_eq!(policy.backoff_ms(10), 1_000); // capped
    }

    #[test]
    fn jitter_stays_within_expected_bounds() {
        let policy = RetryPolicy {
            max_attempts: 3,
            initial_backoff_ms: 100,
            max_backoff_ms: 10_000,
            jitter: true,
        };
        for attempt in 0..20 {
            let j = policy.jittered(100, attempt);
            assert!(
                (100..=150).contains(&j),
                "jitter {j} out of [100,150] at attempt {attempt}"
            );
        }
    }

    #[test]
    fn stream_complete_retries_on_transient_error() {
        struct FailOnceStream {
            calls: AtomicU32,
        }
        impl ModelClient for FailOnceStream {
            fn complete(
                &self,
                _r: &CompletionRequest,
            ) -> Result<CompletionResponse, InferenceError> {
                unreachable!("test only exercises stream_complete")
            }
            fn stream_complete(
                &self,
                _request: &CompletionRequest,
                on_token: &mut dyn FnMut(TokenEvent),
            ) -> Result<CompletionResponse, InferenceError> {
                let n = self.calls.fetch_add(1, Ordering::SeqCst);
                if n == 0 {
                    Err(InferenceError::Unreachable("drop".into()))
                } else {
                    on_token(TokenEvent {
                        text: "hi".into(),
                        finished: true,
                    });
                    Ok(ok_response("hi"))
                }
            }
        }

        let inner = FailOnceStream {
            calls: AtomicU32::new(0),
        };
        let client = RetryingModelClient::new(inner, fast_policy(3));
        let mut tokens = Vec::new();
        let resp = client
            .stream_complete(&req(), &mut |ev| tokens.push(ev.text))
            .unwrap();
        assert_eq!(resp.content, "hi");
        assert_eq!(tokens, vec!["hi".to_string()]);
    }
}
