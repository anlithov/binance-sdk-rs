use crate::config::REST_API_HOST;
use crate::rest::core::inner_client::InnerClient;
use crate::rest::core::rate_limiter::ip_rate_limit_manager::IpRateLimitManager;
use crate::rest::core::rate_limiter::order_rate_limit_manager::OrderRateLimitManager;
use crate::rest::endpoints::{SpotV3, API};
use crate::rest::spot::v3::market::responses::GeneralExchangeInfoResponse;
use crate::result::AnyhowResult;
use std::sync::Arc;

pub mod error;
pub mod initialization;
pub mod ip_rate_limit_manager;
pub mod order_rate_limit_manager;
pub mod reader;

/// Manages rate limit information from exchange info
#[derive(Debug)]
pub struct RateLimitManager {
  ip_rate_limiter: Arc<IpRateLimitManager>,
  order_rate_limiter: Arc<OrderRateLimitManager>,
}

impl RateLimitManager {
  /// Create a new rate limit manager
  pub async fn new() -> AnyhowResult<Arc<Self>> {
    let client = InnerClient::new(None, None, REST_API_HOST.to_string());
    let exchange_info: GeneralExchangeInfoResponse =
      client.get(API::SpotV3(SpotV3::ExchangeInfo), None).await?;

    let order_rate_limiter = Self::init_order_rate_limit_manager(&exchange_info.rate_limits);
    let ip_rate_limiter = Self::init_ip_rate_limit_manager(&exchange_info.rate_limits);

    Ok(Arc::new(Self {
      ip_rate_limiter,
      order_rate_limiter,
    }))
  }

  pub fn ip_rate_limiter(&self) -> Arc<IpRateLimitManager> {
    self.ip_rate_limiter.clone()
  }

  pub fn order_rate_limiter(&self) -> Arc<OrderRateLimitManager> {
    self.order_rate_limiter.clone()
  }
}
