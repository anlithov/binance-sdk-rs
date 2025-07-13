use crate::rest::core::rate_limiter::RateLimitManager;
use reqwest::Client;
use std::sync::Arc;

pub mod methods;
pub mod rate_limit_manage;

#[derive(Clone)]
pub struct InnerClient {
  api_key: Option<String>,
  secret_key: Option<String>,
  server_host: String,
  http_client: Client,
  rate_limit_manager: Option<Arc<RateLimitManager>>,
}

impl InnerClient {
  pub fn new(api_key: Option<String>, secret_key: Option<String>, server_host: String) -> Self {
    Self {
      api_key,
      secret_key,
      server_host,
      http_client: Client::builder().pool_idle_timeout(None).build().unwrap(),
      rate_limit_manager: None,
    }
  }

  /// Create a new client with a custom rate limiter
  pub fn with_rate_limit_manager(mut self, rate_limiter: Arc<RateLimitManager>) -> Self {
    self.rate_limit_manager = Some(rate_limiter);
    self
  }
}
