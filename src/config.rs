use crate::rest::core::rate_limiter::ip_rate_limit_manager::IpRateLimitManager;
use crate::rest::core::rate_limiter::unfilled_order_rate_limit_manager::UnfilledOrderRateLimitManager;
use std::sync::Arc;

pub const REST_API_HOST: &str = "https://api.binance.com";
const WS_HOST: &str = "wss://stream.binance.com/ws";
const FUTURES_REST_API_HOST: &str = "https://fapi.binance.com";
const FUTURES_WS_HOST: &str = "wss://fstream.binance.com/ws";

#[derive(Clone, Debug)]
pub struct Config {
  pub rest_api_host: String,
  pub ws_host: String,
  pub futures_rest_api_host: String,
  pub futures_ws_host: String,
  pub recv_window: u64,
  pub ip_rate_limit_manager: Option<Arc<IpRateLimitManager>>,
  pub unfilled_order_rate_limit_manager: Option<Arc<UnfilledOrderRateLimitManager>>,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      rest_api_host: REST_API_HOST.into(),
      ws_host: WS_HOST.into(),
      futures_rest_api_host: FUTURES_REST_API_HOST.into(),
      futures_ws_host: FUTURES_WS_HOST.into(),
      recv_window: 5000,
      ip_rate_limit_manager: None,
      unfilled_order_rate_limit_manager: None,
    }
  }
}

impl Config {
  pub fn testnet() -> Self {
    Self::default()
      .set_rest_api_endpoint("https://testnet.binance.vision")
      .set_ws_endpoint("wss://testnet.binance.vision/ws")
      .set_futures_rest_api_endpoint("https://testnet.binancefuture.com")
      .set_futures_ws_endpoint("https://testnet.binancefuture.com/ws")
  }

  pub fn set_rest_api_endpoint<T: Into<String>>(mut self, rest_api_host: T) -> Self {
    self.rest_api_host = rest_api_host.into();
    self
  }

  pub fn set_ws_endpoint<T: Into<String>>(mut self, ws_host: T) -> Self {
    self.ws_host = ws_host.into();
    self
  }

  pub fn set_futures_rest_api_endpoint<T: Into<String>>(
    mut self,
    futures_rest_api_host: T,
  ) -> Self {
    self.futures_rest_api_host = futures_rest_api_host.into();
    self
  }

  pub fn set_futures_ws_endpoint<T: Into<String>>(mut self, futures_ws_host: T) -> Self {
    self.futures_ws_host = futures_ws_host.into();
    self
  }

  pub fn set_recv_window(mut self, recv_window: u64) -> Self {
    self.recv_window = recv_window;
    self
  }

  pub fn set_ip_rate_limit_manager(mut self, rate_limit_manager: Arc<IpRateLimitManager>) -> Self {
    self.ip_rate_limit_manager = Some(rate_limit_manager);
    self
  }

  pub fn set_unfilled_order_rate_limit_manager(
    mut self,
    rate_limit_manager: Arc<UnfilledOrderRateLimitManager>,
  ) -> Self {
    self.unfilled_order_rate_limit_manager = Some(rate_limit_manager);
    self
  }
}
