use super::model::{AccountCommissionRatesResponse, RateLimitIntervalResponse};
use super::Account;
use crate::rest::endpoints::{Spot, API};
use crate::util::build_signed_request;
use anyhow::Result;
use std::collections::BTreeMap;

impl Account {
  /// Displays the user's unfilled order count for all intervals.
  pub async fn rate_limit_for_orders(&self) -> Result<Vec<RateLimitIntervalResponse>> {
    let request = build_signed_request(BTreeMap::new(), self.recv_window)?;

    self
      .client
      .get_signed(API::Spot(Spot::RateLimitOrder), Some(request))
      .await
  }

  pub async fn commission_rates<S>(&self, symbol: S) -> Result<Vec<AccountCommissionRatesResponse>>
  where
    S: Into<String>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();
    parameters.insert("symbol".into(), symbol.into());

    let request = build_signed_request(parameters, self.recv_window)?;

    self
      .client
      .get_signed(API::Spot(Spot::AccountCommissions), Some(request))
      .await
  }
}
