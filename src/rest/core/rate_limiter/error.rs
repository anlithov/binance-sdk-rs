use crate::rest::spot::v3::market::responses::RateLimitIntervalResponse;

/// A more detailed error type for order rate limiting issues
#[derive(Debug)]
pub enum RateLimitError {
  /// Rate limit exceeded, with suggested wait time and limit type
  LimitExceeded {
    interval: RateLimitIntervalResponse,
    interval_num: u64,
  },
  /// Lock acquisition failure
  LockError(String),
  /// Other error
  Other(String),
}

// Add implementations for Display and Error traits
impl std::fmt::Display for RateLimitError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      RateLimitError::LimitExceeded {
        interval,
        interval_num,
      } => {
        write!(
          f,
          "Rate limit exceeded per: {:?} interval, interval num: {}",
          interval, interval_num
        )
      }
      RateLimitError::LockError(msg) => write!(f, "Rate limiter lock error: {}", msg),
      RateLimitError::Other(msg) => write!(f, "Rate limiter error: {}", msg),
    }
  }
}

impl std::error::Error for RateLimitError {}
