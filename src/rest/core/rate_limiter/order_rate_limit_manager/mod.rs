use crate::rest::core::rate_limiter::error::RateLimitError;
use crate::rest::spot::v3::market::responses::RateLimitIntervalResponse;
use crate::result::AnyhowResult;
use anyhow::anyhow;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod manage;

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct OrderIntervalAndNum {
  pub interval: RateLimitIntervalResponse,
  pub interval_num: u64,
}

#[derive(Default, Debug)]
pub struct OrderRateLimitInterval {
  limit: Arc<Mutex<u64>>,
  count_used: Arc<Mutex<u64>>,
}

/// OrderRateLimiter handles Binance's multi-tier order rate limits:
#[derive(Debug)]
pub struct OrderRateLimitManager {
  intervals: HashMap<OrderIntervalAndNum, OrderRateLimitInterval>,
}

impl OrderRateLimitManager {
  pub fn new() -> Self {
    Self {
      intervals: HashMap::new(),
    }
  }

  pub async fn orders_this_period(&self, interval: &OrderIntervalAndNum) -> AnyhowResult<u64> {
    let count_rates = self.intervals.get(interval);

    if let Some(count_rates) = count_rates {
      Ok(*count_rates.count_used.lock().await)
    } else {
      Err(anyhow!("Interval not found"))
    }
  }

  /// Acquire permission to place an order
  /// Returns Ok(()) if the order can proceed, or an error if rate limited
  pub async fn acquire(&self) -> AnyhowResult<()> {
    for (key, interval_limit) in self.intervals.iter() {
      let mut count = interval_limit.count_used.lock().await;

      return if *count < *interval_limit.limit.lock().await {
        *count += 1;

        Ok(())
      } else {
        Err(anyhow!(RateLimitError::LimitExceeded {
          interval: key.interval.clone(),
          interval_num: key.interval_num,
        }))
      };
    }

    Ok(())
  }
}
