use crate::config::Config;
use crate::rest::inner_client::InnerClientRest;
use crate::rest::spot::account::Account;

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

impl Binance for Account {
  fn new(api_key: Option<String>, secret_key: Option<String>) -> Account {
    Self::new_with_config(api_key, secret_key, &Config::default())
  }

  fn new_with_config(
    api_key: Option<String>,
    secret_key: Option<String>,
    config: &Config,
  ) -> Account {
    Account {
      client: InnerClientRest::new(api_key, secret_key, config.rest_api_host.clone()),
      recv_window: config.recv_window,
    }
  }
}
