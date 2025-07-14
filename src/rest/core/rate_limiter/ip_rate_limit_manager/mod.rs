use crate::rest::core::rate_limiter::error::RateLimitError;
use crate::rest::endpoints::API;
use crate::rest::spot::v3::market::responses::RateLimitIntervalResponse;
use crate::result::AnyhowResult;
use anyhow::anyhow;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

pub mod manage;
pub mod read;
pub mod weight_calculators;

/// Type for weight calculator functions
pub type WeightCalculator = fn(endpoint: &API, query: Option<String>) -> u64;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct IpIntervalAndNum {
  pub interval: RateLimitIntervalResponse,
  pub interval_num: u64,
}

impl IpIntervalAndNum {
  /// Parse an interval from a header name like "x-mbx-used-weight-1m"
  /// Returns Some(IpIntervalAndNum) if parsing is successful, None otherwise
  pub(crate) fn from_header_name(header_name: &str) -> Option<Self> {
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
      'S' => RateLimitIntervalResponse::Second,
      'M' => RateLimitIntervalResponse::Minute,
      'D' => RateLimitIntervalResponse::Day,
      _ => return None,
    };

    Some(IpIntervalAndNum {
      interval,
      interval_num,
    })
  }
}

#[derive(Default, Debug)]
pub(crate) struct IpIntervalSetup {
  pub weight_limit: Arc<Mutex<u64>>,
  pub weight_count: Arc<Mutex<u64>>,
}

/// RateLimiter based on token bucket algorithm for Binance API
#[derive(Debug)]
pub struct IpRateLimitManager {
  pub(crate) intervals: HashMap<IpIntervalAndNum, IpIntervalSetup>,
  pub(crate) last_updated_limits: Arc<Mutex<Instant>>,
  pub(crate) last_updated_used_count: Arc<Mutex<Instant>>,
  /// Maps API endpoints to their weight calculator functions
  pub(crate) endpoint_weight_calculators: HashMap<API, WeightCalculator>,
  /// Default weight calculators for endpoint types
  pub(crate) default_calculators: HashMap<String, WeightCalculator>,
}

impl IpRateLimitManager {
  /// Create a new rate limiter with default settings
  pub(crate) fn new() -> Self {
    let mut instance = Self {
      intervals: HashMap::new(),
      last_updated_limits: Arc::new(Mutex::new(Instant::now())),
      last_updated_used_count: Arc::new(Mutex::new(Instant::now())),
      endpoint_weight_calculators: HashMap::new(),
      default_calculators: HashMap::new(),
    };

    // Initialize weight calculators
    instance.init_weight_calculators();

    instance
  }

  /// Acquire permission to make a request with a certain weight based on endpoint and query
  /// Returns Ok(()) if the request can proceed, or an error if rate limited
  pub(crate) async fn acquire(&self, api: &API, query: Option<String>) -> AnyhowResult<()> {
    let future_spent_weight = self.calc_endpoint_weight(api, query);

    for (key, interval_limit) in self.intervals.iter() {
      let mut weight_count = interval_limit.weight_count.lock().await;

      return if *weight_count < *interval_limit.weight_limit.lock().await {
        *weight_count += future_spent_weight;

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
