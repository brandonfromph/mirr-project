#![forbid(unsafe_code)]
#![deny(warnings)]

use std::future::Future;
use std::time::Duration;
use tokio::time::{sleep, timeout};

const DEFAULT_TIMEOUT_MS: u64 = 30_000;
const MAX_TIMEOUT_MS: u64 = 60_000;
const MAX_RETRIES: u8 = 5;
const BASE_BACKOFF_MS: u64 = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryDecision {
    Retry,
    Stop,
}

#[derive(Debug, Clone, Copy)]
pub struct ResiliencePolicy {
    pub max_retries: u8,
    pub timeout_ms: u64,
    pub fallback_to_lexical: bool,
}

impl Default for ResiliencePolicy {
    fn default() -> Self {
        Self { max_retries: 1, timeout_ms: DEFAULT_TIMEOUT_MS, fallback_to_lexical: true }
    }
}

impl ResiliencePolicy {
    pub fn normalized(self) -> Self {
        Self {
            max_retries: self.max_retries.min(MAX_RETRIES),
            timeout_ms: self.timeout_ms.clamp(1_000, MAX_TIMEOUT_MS),
            fallback_to_lexical: self.fallback_to_lexical,
        }
    }
}

pub async fn run_with_resilience<T, F, Fut>(
    policy: ResiliencePolicy,
    mut operation: F,
) -> anyhow::Result<T>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = anyhow::Result<T>>,
{
    let normalized = policy.normalized();

    let mut attempt = 0_u8;
    loop {
        let result = timeout(Duration::from_millis(normalized.timeout_ms), operation()).await;
        match result {
            Ok(Ok(value)) => return Ok(value),
            Ok(Err(err)) => {
                if should_retry(attempt, normalized.max_retries) == RetryDecision::Stop {
                    return Err(err);
                }
            }
            Err(_) => {
                let timeout_error =
                    anyhow::anyhow!("query timed out after {} ms", normalized.timeout_ms);
                if should_retry(attempt, normalized.max_retries) == RetryDecision::Stop {
                    return Err(timeout_error);
                }
            }
        }

        attempt = attempt.saturating_add(1);
        let backoff = BASE_BACKOFF_MS.saturating_mul((attempt as u64).saturating_add(1));
        sleep(Duration::from_millis(backoff)).await;
    }
}

fn should_retry(attempt: u8, max_retries: u8) -> RetryDecision {
    if attempt >= max_retries {
        RetryDecision::Stop
    } else {
        RetryDecision::Retry
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[tokio::test]
    async fn retries_then_succeeds() {
        let attempts = AtomicUsize::new(0);
        let policy =
            ResiliencePolicy { max_retries: 3, timeout_ms: 1_000, fallback_to_lexical: true };

        let result = run_with_resilience(policy, || {
            let count = attempts.fetch_add(1, Ordering::SeqCst);
            async move {
                if count < 2 {
                    Err(anyhow::anyhow!("transient"))
                } else {
                    Ok(42_u32)
                }
            }
        })
        .await
        .expect("eventually succeeds");

        assert_eq!(result, 42);
        assert_eq!(attempts.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn timeout_produces_error() {
        let policy =
            ResiliencePolicy { max_retries: 0, timeout_ms: 1_000, fallback_to_lexical: true };
        let result = run_with_resilience(policy, || async {
            tokio::time::sleep(Duration::from_millis(1_500)).await;
            Ok(())
        })
        .await;
        assert!(result.is_err());
    }
}
