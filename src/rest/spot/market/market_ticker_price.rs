use super::Market;
use crate::rest::endpoints::{Spot, API};
use crate::rest::spot::market::model::TickerPriceResponse;
use crate::util::{build_request, vec_to_string};
use anyhow::Result;
use std::collections::BTreeMap;

impl Market {
  /// Latest price for ONE symbol.
  pub async fn fetch_ticker_price_latest<S>(&self, symbol: S) -> Result<TickerPriceResponse>
  where
    S: Into<String>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();
    parameters.insert("symbol".into(), symbol.into());
    let request = build_request(parameters);
    self.client.get(API::Spot(Spot::Price), Some(request)).await
  }

  /// Latest prices for MULTI symbol.
  pub async fn list_ticker_prices_multi<S>(&self, symbols: S) -> Result<Vec<TickerPriceResponse>>
  where
    S: Into<Vec<String>>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();

    let formatted = vec_to_string(symbols.into());

    parameters.insert("symbols".into(), formatted);
    let request = build_request(parameters);
    self.client.get(API::Spot(Spot::Price), Some(request)).await
  }

  /// Latest price for ONE symbol.
  pub async fn list_ticker_prices_all(&self) -> Result<Vec<TickerPriceResponse>> {
    self.client.get(API::Spot(Spot::Price), None).await
  }

  /// Current average price for ONE symbol.
  pub async fn fetch_ticker_price_avg<S>(&self, symbol: S) -> Result<TickerPriceResponse>
  where
    S: Into<String>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();
    parameters.insert("symbol".into(), symbol.into());
    let request = build_request(parameters);
    self
      .client
      .get(API::Spot(Spot::AvgPrice), Some(request))
      .await
  }
}
