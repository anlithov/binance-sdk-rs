use crate::config::REST_API_HOST;
use crate::rest::core::inner_client::InnerClient;
use crate::rest::core::rate_limiter::ip_rate_limit_manager::IpRateLimitManager;
use crate::rest::core::rate_limiter::order_rate_limit_manager::OrderRateLimitManager;
use crate::rest::endpoints::{SpotV3, API};
use crate::rest::spot::v3::account::responses::AccountRateLimitResponse;
use crate::rest::spot::v3::market::responses::GeneralExchangeInfoResponse;
use crate::result::AnyhowResult;
use crate::util::build_signed_query;
use std::collections::BTreeMap;
use std::sync::Arc;

pub mod error;
pub mod initialization;
pub mod ip_rate_limit_manager;
pub mod order_rate_limit_manager;

/// Manages rate limit information from exchange info
#[derive(Debug)]
pub struct RateLimitManager {
  pub(crate) ip_rate_limiter: Option<Arc<IpRateLimitManager>>,
  pub(crate) order_rate_limiter: Option<Arc<OrderRateLimitManager>>,
}

impl RateLimitManager {
  pub fn new() -> Self {
    Self {
      ip_rate_limiter: None,
      order_rate_limiter: None,
    }
  }
  /// Create a new rate limit manager
  pub async fn with_ip_rate_limiter(self) -> AnyhowResult<Self> {
    let arc = Arc::new(self);

    let ip_rate_limiter = {
      // We want ip rate limiter ALSO setup spent WEIGHT in the past.
      // We put arc into InnerClient
      // InnerClient handles response from any endpoint and updates the rate limiter
      // then we dispose of Arc and return rate limiter with correct spent count
      let client = InnerClient::new(None, None, REST_API_HOST.to_string())
        .with_rate_limit_manager(arc.clone());
      let exchange_info: GeneralExchangeInfoResponse =
        client.get(API::SpotV3(SpotV3::ExchangeInfo), None).await?;

      drop(client);

      Self::init_ip_rate_limit_manager(&exchange_info.rate_limits)
    };

    let mut unwrapped = Arc::try_unwrap(arc).unwrap();

    unwrapped.ip_rate_limiter = Some(ip_rate_limiter);

    Ok(unwrapped)
  }

  pub async fn with_order_rate_limiter(
    self,
    api_key: String,
    secret_key: String,
  ) -> AnyhowResult<Self> {
    let arc = Arc::new(self);

    let order_rate_limiter = {
      // We want ip rate limiter, if defined, ALSO setup spent WEIGHT in the past.
      // We put arc into InnerClient
      // InnerClient handles response from any endpoint and updates the rate limiter
      // then we dispose of Arc and return rate limiter with correct spent count
      let client = InnerClient::new(Some(api_key), Some(secret_key), REST_API_HOST.to_string())
        .with_rate_limit_manager(arc.clone());

      let request = build_signed_query(BTreeMap::new(), 5000)?;
      // To get used already counts
      let account_order_rate_limits: Vec<AccountRateLimitResponse> = client
        .get_signed(API::SpotV3(SpotV3::RateLimitOrder), Some(request))
        .await?;

      Self::init_order_rate_limit_manager(&account_order_rate_limits)
    };

    let mut unwrapped = Arc::try_unwrap(arc).unwrap();

    unwrapped.order_rate_limiter = Some(order_rate_limiter);

    Ok(unwrapped)
  }

  pub fn ip_rate_limiter(&self) -> Option<Arc<IpRateLimitManager>> {
    self.ip_rate_limiter.clone()
  }

  pub fn order_rate_limiter(&self) -> Option<Arc<OrderRateLimitManager>> {
    self.order_rate_limiter.clone()
  }
}
