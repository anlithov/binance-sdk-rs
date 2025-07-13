use crate::rest::endpoints::API;
use crate::rest::spot::v3::market::responses::RateLimitIntervalResponse;
use crate::result::AnyhowResult;
use anyhow::anyhow;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

pub mod manage;
pub mod weight_calculators;

/// Type for weight calculator functions
pub type WeightCalculator = fn(endpoint: &API, query: Option<&str>) -> u32;

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct IpIntervalAndNum {
  pub interval: RateLimitIntervalResponse,
  pub interval_num: u64,
}

impl IpIntervalAndNum {
  /// Parse an interval from a header name like "x-mbx-used-weight-1m"
  /// Returns Some(IpIntervalAndNum) if parsing is successful, None otherwise
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
pub struct IpRateLimitInterval {
  weight_limit: Arc<Mutex<u64>>,
  weight_used: Arc<Mutex<u64>>,
}

/// RateLimiter based on token bucket algorithm for Binance API
#[derive(Debug)]
pub struct IpRateLimitManager {
  intervals: HashMap<IpIntervalAndNum, IpRateLimitInterval>,
  /// Maps API endpoints to their weight calculator functions
  endpoint_weight_calculators: HashMap<API, WeightCalculator>,
  /// Default weight calculators for endpoint types
  default_calculators: HashMap<String, WeightCalculator>,
}

impl IpRateLimitManager {
  /// Create a new rate limiter with default settings
  pub fn new() -> Self {
    let mut instance = Self {
      intervals: HashMap::new(),
      endpoint_weight_calculators: HashMap::new(),
      default_calculators: HashMap::new(),
    };

    // Initialize weight calculators
    instance.init_weight_calculators();

    instance
  }

  pub fn add_interval_and_limit(&mut self, interval: IpIntervalAndNum, weight_limit: u64) {
    self.intervals.insert(
      interval,
      IpRateLimitInterval {
        weight_limit: Arc::new(Mutex::new(weight_limit)),
        weight_used: Arc::new(Mutex::new(0)),
      },
    );
  }

  pub async fn weight_this_period(&self, interval: &IpIntervalAndNum) -> AnyhowResult<u64> {
    let count_rates = self.intervals.get(interval);

    if let Some(count_rates) = count_rates {
      Ok(*count_rates.weight_used.lock().await)
    } else {
      Err(anyhow!("Interval not found"))
    }
  }

  /// Get the weight for a specific API endpoint and query
  pub fn get_weight(&self, api: &API, query: Option<&str>) -> u32 {
    // First check if we have a specific calculator for this endpoint
    if let Some(calculator) = self.endpoint_weight_calculators.get(api) {
      return calculator(api, query);
    }

    // If no specific calculator, use the default calculator for the API type
    match api {
      API::SpotV3(_) => {
        if let Some(calculator) = self.default_calculators.get("SpotV3") {
          return calculator(api, query);
        }
      }
      // Handle other API types
      _ => {}
    }

    // Fallback to default weight of 1
    1
  }

  /// Acquire permission to make a request with a certain weight based on endpoint and query
  /// Returns Ok(()) if the request can proceed, or an error if rate limited
  pub async fn acquire(&self, api: &API, query: Option<String>) -> AnyhowResult<()> {
    let weight = self.get_weight(api, query);
    self.acquire_weight(weight).await
  }

  /// Acquire a specific weight amount
  async fn acquire_weight(&self, weight: u32) -> AnyhowResult<()> {
    // Lock mutex with proper error handling
    let mut available = self.available_weights.lock().await;
    let mut last_refill = self.last_refill.lock().await;
    let limit = self.weight_limit.lock().await;

    // Calculate how much time passed since last refill
    let now = Instant::now();
    let duration_since_refill = now.duration_since(*last_refill);

    // Calculate how many tokens to add based on time passed
    // Rate is tokens per minute, so we calculate the proportion based on elapsed time
    let tokens_to_add = (duration_since_refill.as_millis() as f64 / 60000.0) * *limit as f64;
    let tokens_to_add = tokens_to_add.floor() as u32;

    // Add tokens, but don't exceed the bucket capacity (weight_limit)
    *available = (*available + tokens_to_add).min(*limit);

    // Update last refill time - only account for the time we actually credited tokens for
    if tokens_to_add > 0 {
      let refill_duration =
        Duration::from_millis((tokens_to_add as f64 / *limit as f64 * 60000.0) as u64);
      *last_refill = *last_refill + refill_duration;
    }

    // Check if we have enough tokens for this request
    if *available >= weight {
      // We have enough capacity, deduct the weight
      *available -= weight;
      Ok(())
    } else {
      // Not enough capacity, calculate wait time
      let weight_needed = weight - *available;
      let refill_rate_per_ms = *limit as f64 / 60000.0;
      let wait_time_ms = (weight_needed as f64 / refill_rate_per_ms).ceil() as u64;

      Err(anyhow!(
        "Rate limited. Try again in {:.2} seconds",
        wait_time_ms as f64 / 1000.0
      ))
    }
  }
}
