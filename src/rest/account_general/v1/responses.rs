use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountApiRestrictions {
  pub ip_restrict: bool,
  pub create_time: u64,
  pub enable_reading: bool,
  pub enable_withdrawals: bool, // This option allows you to withdraw via API. You must apply the IP Access Restriction filter in order to enable withdrawals
  pub enable_internal_transfer: bool, // This option authorizes this key to transfer funds between your master account and your sub_account instantly
  pub enable_margin: bool, //  This option can be adjusted after the Cross Margin account transfer is completed
  pub enable_futures: bool, //  The Futures API cannot be used if the API key was created before the Futures account was opened, or if you have enabled portfolio margin.
  pub permits_universal_transfer: bool, // Authorizes this key to be used for a dedicated universal transfer API to transfer multiple supported currencies. Each business's own transfer API rights are not affected by this authorization
  pub enable_vanilla_options: bool,     //  Authorizes this key to Vanilla options trading
  pub enable_fix_api_trade: bool,       //
  pub enable_fix_read_only: bool,
  pub enable_spot_and_margin_trading: bool, // Spot and margin trading
  pub enable_portfolio_margin_trading: bool, //  API Key created before your activate portfolio margin does not support portfolio margin API service
}
