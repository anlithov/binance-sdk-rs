use super::SpotMarketV3Manager;
use crate::rest::endpoints::{SpotV3, API};
use crate::rest::spot::v3::market::responses::OrderBookResponse;
use crate::util::build_query;
use anyhow::Result;
use std::collections::BTreeMap;

// Market Data endpoints
impl SpotMarketV3Manager {
  /// Order book at the default depth of 100
  pub async fn fetch_depth<S>(&self, symbol: S) -> Result<OrderBookResponse>
  where
    S: Into<String>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();

    parameters.insert("symbol".into(), symbol.into());
    let request = build_query(parameters);

    self
      .client
      .get(API::SpotV3(SpotV3::Depth), Some(request))
      .await
  }

  /// Order book at the default depth of 100
  pub async fn fetch_depth_with_limit<S, L>(&self, symbol: S, limit: L) -> Result<OrderBookResponse>
  where
    S: Into<String>,
    L: Into<u16>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();

    parameters.insert("symbol".into(), symbol.into());
    parameters.insert("limit".into(), limit.into().to_string());
    let request = build_query(parameters);

    self
      .client
      .get(API::SpotV3(SpotV3::Depth), Some(request))
      .await
  }
}
