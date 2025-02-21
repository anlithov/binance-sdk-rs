use super::responses::{AccountCommissionRatesResponse, RateLimitIntervalResponse};
use super::SpotAccountManagerV3;
use crate::rest::endpoints::{SpotV3, API};
use crate::util::build_signed_query;
use anyhow::Result;
use std::collections::BTreeMap;

impl SpotAccountManagerV3 {
  /// Displays the user's unfilled order count for all intervals.
  pub async fn fetch_rate_limits_for_orders(&self) -> Result<Vec<RateLimitIntervalResponse>> {
    let request = build_signed_query(BTreeMap::new(), self.recv_window)?;

    self
      .client
      .get_signed(API::SpotV3(SpotV3::RateLimitOrder), Some(request))
      .await
  }

  pub async fn fetch_symbol_fee_rates<S>(
    &self,
    symbol: S,
  ) -> Result<Vec<AccountCommissionRatesResponse>>
  where
    S: Into<String>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();
    parameters.insert("symbol".into(), symbol.into());

    let request = build_signed_query(parameters, self.recv_window)?;

    self
      .client
      .get_signed(API::SpotV3(SpotV3::AccountCommissions), Some(request))
      .await
  }
}
