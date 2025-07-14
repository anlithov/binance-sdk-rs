use crate::rest::core::rate_limiter::error::RateLimitError;
use crate::rest::spot::v3::account::responses::AccountRateLimitIntervalResponse;
use crate::result::AnyhowResult;
use anyhow::anyhow;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::Instant;

pub mod manage;
pub mod read;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct OrderIntervalAndNum {
  pub(crate) interval: AccountRateLimitIntervalResponse,
  pub(crate) interval_num: u64,
}

impl OrderIntervalAndNum {
  /// Parse an interval from a header name like "x-mbx-order-count-1m"
  /// Returns Some(OrderIntervalAndNum) if parsing is successful, None otherwise
  pub fn from_header_name(header_name: &str) -> Option<Self> {
    // Extract the part after the last dash
    let parts: Vec<&str> = header_name.split('-').collect();
    if parts.len() < 2 {
      return None;
    }

    let interval_part = parts.last().unwrap();
    if interval_part.len() < 2 {
      return None;
    }

    // Last character is the interval letter
    let interval_char = interval_part.chars().last().unwrap().to_ascii_uppercase();

    // Everything before the last character is the interval number
    let interval_num_str = &interval_part[..interval_part.len() - 1];
    let interval_num = match interval_num_str.parse::<u64>() {
      Ok(num) => num,
      Err(_) => return None,
    };

    // Map the letter to an interval type
    let interval = match interval_char {
      'S' => AccountRateLimitIntervalResponse::Second,
      'M' => AccountRateLimitIntervalResponse::Minute,
      'D' => AccountRateLimitIntervalResponse::Day,
      _ => return None,
    };

    Some(OrderIntervalAndNum {
      interval,
      interval_num,
    })
  }
}

#[derive(Default, Debug)]
pub struct OrderRateLimitInterval {
  order_limit: Arc<Mutex<u64>>,
  order_count: Arc<Mutex<u64>>,
}

/// OrderRateLimiter handles Binance's multi-tier order rate limits:
#[derive(Debug)]
pub struct OrderRateLimitManager {
  intervals: HashMap<OrderIntervalAndNum, OrderRateLimitInterval>,
  last_updated_limits: Arc<Mutex<Instant>>,
  last_updated_used_count: Arc<Mutex<Instant>>,
}

impl OrderRateLimitManager {
  pub fn new() -> Self {
    Self {
      intervals: HashMap::new(),
      last_updated_limits: Arc::new(Mutex::new(Instant::now())),
      last_updated_used_count: Arc::new(Mutex::new(Instant::now())),
    }
  }

  pub async fn orders_this_period(&self, interval: &OrderIntervalAndNum) -> AnyhowResult<u64> {
    let count_rates = self.intervals.get(interval);

    if let Some(count_rates) = count_rates {
      Ok(*count_rates.order_count.lock().await)
    } else {
      Err(anyhow!("Interval not found"))
    }
  }

  /// Acquire permission to place an order
  /// Returns Ok(()) if the order can proceed, or an error if rate limited
  pub async fn acquire(&self) -> AnyhowResult<()> {
    for (key, interval_limit) in self.intervals.iter() {
      let mut count = interval_limit.order_count.lock().await;

      return if *count < *interval_limit.order_limit.lock().await {
        *count += 1;

        Ok(())
      } else {
        Err(anyhow!(RateLimitError::LimitExceeded {
          interval: key.interval.to_string(),
          interval_num: key.interval_num,
        }))
      };
    }

    Ok(())
  }
}
