use crate::config::Config;
use crate::rest::account_general::v1::AccountGeneralManagerV1;
use crate::rest::core::inner_client::InnerClient;
use crate::rest::spot::v3::account::SpotAccountManagerV3;
use crate::rest::spot::v3::general::GeneralManagerV3;
use crate::rest::spot::v3::market::SpotMarketV3Manager;
use crate::rest::spot::v3::trade::SpotTradeV3Manager;
use crate::rest::spot::v3::user_stream::SpotUserStreamManagerV3;

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
        let mut inner_client = InnerClient::new(api_key, secret_key, config.rest_api_host.clone());

        if let Some(rate_limit_manager) = &config.ip_rate_limit_manager {
          inner_client = inner_client.with_ip_rate_limit_manager(rate_limit_manager.clone());
        }
        if let Some(rate_limit_manager) = &config.unfilled_order_rate_limit_manager {
          inner_client =
            inner_client.with_unfilled_order_rate_limit_manager(rate_limit_manager.clone());
        }

        $typename {
          client: inner_client,
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
impl_binance_for!(AccountGeneralManagerV1);
