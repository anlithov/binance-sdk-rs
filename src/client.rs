use crate::config::Config;
use crate::rest::inner_client::InnerClient;
use crate::rest::spot::account_v3::SpotAccountManagerV3;
use crate::rest::spot::general_v3::GeneralManagerV3;
use crate::rest::spot::market_v3::SpotMarketV3Manager;
use crate::rest::spot::trade_v3::SpotTradeV3Manager;
use crate::rest::spot::user_stream_v3::SpotUserStreamManagerV3;

/// A trait that all modules must implement to be a Binance API client module.
///
/// This trait provides two methods to create a new instance of the module,
/// one with default configuration and one with custom configuration.
pub trait Binance {
  /// Creates a new instance of the module with default configuration.
  ///
  /// # Parameters
  ///
  /// * `api_key`: The API key to be used for the module.
  /// * `secret_key`: The secret key to be used for the module.
  ///
  /// # Returns
  ///
  /// A new instance of the module.
  fn new(api_key: Option<String>, secret_key: Option<String>) -> Self;

  /// Creates a new instance of the module with custom configuration.
  ///
  /// # Parameters
  ///
  /// * `config`: The custom configuration to be used for the module.
  /// * `api_key`: The API key to be used for the module.
  /// * `secret_key`: The secret key to be used for the module.
  ///
  /// # Returns
  ///
  /// A new instance of the module.
  fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self;
}

macro_rules! impl_binance_for {
  ($typename:ident) => {
    impl Binance for $typename {
      fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Self::new_with_config(api_key, secret_key, &Config::default())
      }

      fn new_with_config(
        api_key: Option<String>,
        secret_key: Option<String>,
        config: &Config,
      ) -> Self {
        $typename {
          client: InnerClient::new(api_key, secret_key, config.rest_api_host.clone()),
          recv_window: config.recv_window,
        }
      }
    }
  };
}

impl_binance_for!(SpotAccountManagerV3);
impl_binance_for!(SpotTradeV3Manager);
impl_binance_for!(SpotMarketV3Manager);
impl_binance_for!(GeneralManagerV3);
impl_binance_for!(SpotUserStreamManagerV3);
