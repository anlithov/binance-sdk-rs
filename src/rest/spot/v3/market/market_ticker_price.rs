use super::SpotMarketV3Manager;
use crate::rest::endpoints::{SpotV3, API};
use crate::rest::spot::v3::market::responses::TickerPriceResponse;
use crate::util::{build_query, vec_to_string_array};
use anyhow::Result;
use std::collections::BTreeMap;

impl SpotMarketV3Manager {
  /// Latest price for ONE symbol.
  pub async fn fetch_ticker_price_latest<S>(&self, symbol: S) -> Result<f64>
  where
    S: Into<String>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();
    parameters.insert("symbol".into(), symbol.into());
    let request = build_query(parameters);
    self
      .client
      .get::<TickerPriceResponse>(API::SpotV3(SpotV3::Price), Some(request))
      .await
      .map(|r| r.price)
  }

  /// Latest prices for MULTI symbol.
  pub async fn list_ticker_prices_multi<S>(&self, symbols: S) -> Result<Vec<TickerPriceResponse>>
  where
    S: Into<Vec<String>>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();

    let formatted = vec_to_string_array(symbols.into());

    parameters.insert("symbols".into(), formatted);
    let request = build_query(parameters);
    self
      .client
      .get(API::SpotV3(SpotV3::Price), Some(request))
      .await
  }

  /// Latest price for ONE symbol.
  pub async fn list_ticker_prices_all(&self) -> Result<Vec<TickerPriceResponse>> {
    self.client.get(API::SpotV3(SpotV3::Price), None).await
  }

  /// Current average price for ONE symbol.
  pub async fn fetch_ticker_price_avg<S>(&self, symbol: S) -> Result<TickerPriceResponse>
  where
    S: Into<String>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();
    parameters.insert("symbol".into(), symbol.into());
    let request = build_query(parameters);
    self
      .client
      .get(API::SpotV3(SpotV3::AvgPrice), Some(request))
      .await
  }
}
