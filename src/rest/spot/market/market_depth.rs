use super::Market;
use crate::rest::endpoints::{Spot, API};
use crate::rest::spot::market::model::OrderBookResponse;
use crate::util::build_request;
use anyhow::Result;
use std::collections::BTreeMap;

// Market Data endpoints
impl Market {
  /// Order book at the default depth of 100
  pub async fn depth<S>(&self, symbol: S) -> Result<OrderBookResponse>
  where
    S: Into<String>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();

    parameters.insert("symbol".into(), symbol.into());
    let request = build_request(parameters);

    self.client.get(API::Spot(Spot::Depth), Some(request)).await
  }

  /// Order book at the default depth of 100
  pub async fn depth_with_limit<S, L>(&self, symbol: S, limit: L) -> Result<OrderBookResponse>
  where
    S: Into<String>,
    L: Into<u16>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();

    parameters.insert("symbol".into(), symbol.into());
    parameters.insert("limit".into(), limit.into().to_string());
    let request = build_request(parameters);

    self.client.get(API::Spot(Spot::Depth), Some(request)).await
  }
}
