use crate::config::REST_API_HOST;
use crate::rest::core::inner_client::InnerClient;
use crate::rest::core::rate_limiter::error::RateLimitError;
use crate::rest::core::rate_limiter::ip_rate_limit_manager::IpRateLimitManager;
use crate::rest::endpoints::{SpotV3, API};
use crate::rest::spot::v3::account::responses::{
  AccountRateLimitIntervalResponse, AccountRateLimitResponse,
};
use crate::result::AnyhowResult;
use crate::util::build_signed_query;
use anyhow::anyhow;
use std::collections::{BTreeMap, HashMap};
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

/// UnfilledOrderRateLimitManager handles Binance's multi-tier order rate limits:
#[derive(Debug)]
pub struct UnfilledOrderRateLimitManager {
  intervals: HashMap<OrderIntervalAndNum, OrderRateLimitInterval>,
  last_updated_limits: Arc<Mutex<Instant>>,
  last_updated_used_count: Arc<Mutex<Instant>>,
}

impl UnfilledOrderRateLimitManager {
  /// Creates a new unfilled order rate limit manager
  /// Consumes points to fetch order rate limits and unfilled order counts
  ///
  /// *If ip_rate_limit_manager is defined, it will be used to count spent weight.
  /// For this purpose, the ip rate limiter must be created first*
  pub async fn new(
    api_key: String,
    secret_key: String,
    ip_rate_limit_manager: Option<Arc<IpRateLimitManager>>,
  ) -> AnyhowResult<Arc<Self>> {
    let instance = Self {
      intervals: HashMap::new(),
      last_updated_limits: Arc::new(Mutex::new(Instant::now())),
      last_updated_used_count: Arc::new(Mutex::new(Instant::now())),
    };

    let arc = Arc::new(instance);

    // We want ip rate limiter, if defined, ALSO setup spent WEIGHT in the past.
    // We put arc into InnerClient
    // InnerClient handles response from any endpoint and updates the rate limiter
    // then we dispose of Arc and return rate limiter with correct spent count
    let mut client = InnerClient::new(Some(api_key), Some(secret_key), REST_API_HOST.to_string());
    if let Some(ip_rate_limit_manager) = ip_rate_limit_manager {
      client = client.with_ip_rate_limit_manager(ip_rate_limit_manager);
    }
    client = client.with_unfilled_order_rate_limit_manager(arc.clone());

    let request = build_signed_query(BTreeMap::new(), 5000)?;
    // To get used already counts
    let account_order_rate_limits: Vec<AccountRateLimitResponse> = client
      .get_signed(API::SpotV3(SpotV3::RateLimitOrder), Some(request))
      .await?;

    drop(client);

    let mut unwrapped = Arc::try_unwrap(arc).unwrap();

    for rate_limit in account_order_rate_limits {
      if rate_limit.rate_limit_type.eq("ORDERS") {
        unwrapped.add_interval_and_limit(
          OrderIntervalAndNum {
            interval: rate_limit.interval.clone(),
            interval_num: rate_limit.interval_num,
          },
          rate_limit.limit,
        );
      }
    }

    Ok(Arc::new(unwrapped))
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
