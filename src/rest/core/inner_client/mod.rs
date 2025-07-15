use crate::rest::core::rate_limiter::ip_rate_limit_manager::IpRateLimitManager;
use crate::rest::core::rate_limiter::unfilled_order_rate_limit_manager::UnfilledOrderRateLimitManager;
use reqwest::Client;
use std::sync::Arc;

pub mod ip_rate_limit_manage;
pub mod methods;
pub mod rate_limit_manage;
pub mod unfilled_order_rate_limit_manage;

#[derive(Clone)]
pub struct InnerClient {
  api_key: Option<String>,
  secret_key: Option<String>,
  server_host: String,
  http_client: Client,
  ip_rate_limit_manager: Option<Arc<IpRateLimitManager>>,
  unfilled_order_rate_limit_manager: Option<Arc<UnfilledOrderRateLimitManager>>,
}

impl InnerClient {
  pub fn new(api_key: Option<String>, secret_key: Option<String>, server_host: String) -> Self {
    Self {
      api_key,
      secret_key,
      server_host,
      http_client: Client::builder().pool_idle_timeout(None).build().unwrap(),
      ip_rate_limit_manager: None,
      unfilled_order_rate_limit_manager: None,
    }
  }

  /// Create a new client with a custom rate limiter
  pub fn with_ip_rate_limit_manager(mut self, rate_limiter: Arc<IpRateLimitManager>) -> Self {
    self.ip_rate_limit_manager = Some(rate_limiter);
    self
  }

  pub fn with_unfilled_order_rate_limit_manager(
    mut self,
    rate_limiter: Arc<UnfilledOrderRateLimitManager>,
  ) -> Self {
    self.unfilled_order_rate_limit_manager = Some(rate_limiter);
    self
  }
}
