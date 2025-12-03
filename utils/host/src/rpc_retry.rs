//! RPC retry utilities with exponential backoff.
//!
//! Provides retry logic for RPC calls that may fail due to rate limiting (HTTP 429)
//! or transient errors (502, 503, timeouts).

use std::time::Duration;

use anyhow::Result;
use tracing::warn;

/// Default maximum retry attempts for RPC requests.
pub const DEFAULT_MAX_RETRIES: u32 = 5;

/// Default initial backoff delay in milliseconds.
pub const DEFAULT_INITIAL_BACKOFF_MS: u64 = 500;

/// Default maximum backoff delay in milliseconds.
pub const DEFAULT_MAX_BACKOFF_MS: u64 = 30_000;

/// Gets the maximum retry attempts from RPC_MAX_RETRIES env var.
pub fn get_max_retries() -> u32 {
    std::env::var("RPC_MAX_RETRIES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_MAX_RETRIES)
}

/// Gets the initial backoff in ms from RPC_INITIAL_BACKOFF_MS env var.
pub fn get_initial_backoff_ms() -> u64 {
    std::env::var("RPC_INITIAL_BACKOFF_MS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_INITIAL_BACKOFF_MS)
}

/// Gets the max backoff in ms from RPC_MAX_BACKOFF_MS env var.
pub fn get_max_backoff_ms() -> u64 {
    std::env::var("RPC_MAX_BACKOFF_MS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_MAX_BACKOFF_MS)
}

/// Check if an error is retryable (rate limit or transient).
pub fn is_retryable_error(error: &anyhow::Error) -> bool {
    let err_str = error.to_string().to_lowercase();
    err_str.contains("429")
        || err_str.contains("rate limit")
        || err_str.contains("capacity exceeded")
        || err_str.contains("compute units")
        || err_str.contains("503")
        || err_str.contains("502")
        || err_str.contains("timeout")
        || err_str.contains("connection reset")
        || err_str.contains("connection refused")
}

/// Simple jitter using system time to avoid thundering herd.
fn get_jitter_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as u64 % 100)
        .unwrap_or(0)
}

/// Execute an async operation with exponential backoff retry.
///
/// # Arguments
/// * `operation` - A closure that returns a Future producing a Result
///
/// # Returns
/// The result of the operation if successful within max retries, or the last error.
///
/// # Example
/// ```ignore
/// let result = with_retry(|| async {
///     fetch_data_from_rpc().await
/// }).await?;
/// ```
pub async fn with_retry<T, F, Fut>(operation: F) -> Result<T>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let max_retries = get_max_retries();
    let initial_backoff = get_initial_backoff_ms();
    let max_backoff = get_max_backoff_ms();

    let mut attempt = 0;
    let mut backoff_ms = initial_backoff;

    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) if is_retryable_error(&e) && attempt < max_retries => {
                attempt += 1;
                let jitter = get_jitter_ms();
                let delay = Duration::from_millis(backoff_ms + jitter);
                warn!(
                    attempt,
                    max_retries,
                    delay_ms = delay.as_millis() as u64,
                    error = %e,
                    "RPC request failed, retrying..."
                );
                tokio::time::sleep(delay).await;
                backoff_ms = (backoff_ms * 2).min(max_backoff);
            }
            Err(e) => {
                if attempt > 0 {
                    warn!(
                        attempt,
                        max_retries,
                        error = %e,
                        "RPC request failed after all retries"
                    );
                }
                return Err(e);
            }
        }
    }
}

/// Execute an async operation with exponential backoff retry and custom settings.
///
/// # Arguments
/// * `max_retries` - Maximum number of retry attempts
/// * `initial_backoff_ms` - Initial backoff delay in milliseconds
/// * `max_backoff_ms` - Maximum backoff delay in milliseconds
/// * `operation` - A closure that returns a Future producing a Result
pub async fn with_retry_config<T, F, Fut>(
    max_retries: u32,
    initial_backoff_ms: u64,
    max_backoff_ms: u64,
    operation: F,
) -> Result<T>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let mut attempt = 0;
    let mut backoff_ms = initial_backoff_ms;

    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) if is_retryable_error(&e) && attempt < max_retries => {
                attempt += 1;
                let jitter = get_jitter_ms();
                let delay = Duration::from_millis(backoff_ms + jitter);
                warn!(
                    attempt,
                    max_retries,
                    delay_ms = delay.as_millis() as u64,
                    error = %e,
                    "RPC request failed, retrying..."
                );
                tokio::time::sleep(delay).await;
                backoff_ms = (backoff_ms * 2).min(max_backoff_ms);
            }
            Err(e) => {
                if attempt > 0 {
                    warn!(
                        attempt,
                        max_retries,
                        error = %e,
                        "RPC request failed after all retries"
                    );
                }
                return Err(e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[tokio::test]
    async fn test_with_retry_success_first_attempt() {
        let result = with_retry(|| async { Ok::<_, anyhow::Error>(42) }).await;
        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_with_retry_success_after_retries() {
        let attempts = AtomicU32::new(0);

        let result = with_retry_config(3, 10, 100, || async {
            let count = attempts.fetch_add(1, Ordering::SeqCst);
            if count < 2 {
                Err(anyhow::anyhow!("429 rate limit exceeded"))
            } else {
                Ok(42)
            }
        })
        .await;

        assert_eq!(result.unwrap(), 42);
        assert_eq!(attempts.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_with_retry_exhausts_retries() {
        let attempts = AtomicU32::new(0);

        let result = with_retry_config(2, 10, 100, || async {
            attempts.fetch_add(1, Ordering::SeqCst);
            Err::<i32, _>(anyhow::anyhow!("429 rate limit exceeded"))
        })
        .await;

        assert!(result.is_err());
        // Initial attempt + 2 retries = 3 total attempts
        assert_eq!(attempts.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_non_retryable_error_fails_immediately() {
        let attempts = AtomicU32::new(0);

        let result = with_retry_config(3, 10, 100, || async {
            attempts.fetch_add(1, Ordering::SeqCst);
            Err::<i32, _>(anyhow::anyhow!("invalid argument"))
        })
        .await;

        assert!(result.is_err());
        // Should fail immediately without retrying
        assert_eq!(attempts.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_is_retryable_error() {
        assert!(is_retryable_error(&anyhow::anyhow!("HTTP 429 Too Many Requests")));
        assert!(is_retryable_error(&anyhow::anyhow!("rate limit exceeded")));
        assert!(is_retryable_error(&anyhow::anyhow!(
            "exceeded compute units capacity"
        )));
        assert!(is_retryable_error(&anyhow::anyhow!("503 Service Unavailable")));
        assert!(is_retryable_error(&anyhow::anyhow!("502 Bad Gateway")));
        assert!(is_retryable_error(&anyhow::anyhow!("connection timeout")));

        assert!(!is_retryable_error(&anyhow::anyhow!("invalid argument")));
        assert!(!is_retryable_error(&anyhow::anyhow!("not found")));
    }
}
