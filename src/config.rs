#[derive(Clone, Debug)]
pub struct Config {
  pub rest_api_host: String,
  pub ws_host: String,

  pub futures_rest_api_host: String,
  pub futures_ws_host: String,

  pub recv_window: u64,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      rest_api_host: "https://api.binance.com".into(),
      ws_host: "wss://stream.binance.com/ws".into(),

      futures_rest_api_host: "https://fapi.binance.com".into(),
      futures_ws_host: "wss://fstream.binance.com/ws".into(),

      recv_window: 5000,
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
}
