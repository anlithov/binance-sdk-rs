use super::SpotMarketV3Manager;
use crate::rest::endpoints::{SpotV3, API};
use crate::rest::spot::v3::market::responses::{
  BookTickerResponse, BookTickersMultiResponse, TickerDaySummaryResponse,
};
use crate::util::{build_query, vec_to_string_array};
use anyhow::Result;
use std::collections::BTreeMap;

impl SpotMarketV3Manager {
  /// Symbols order book ticker
  /// -> Best price/qty on the order book for ALL symbols.
  pub async fn list_book_tickers_all(&self) -> Result<BookTickersMultiResponse> {
    self.client.get(API::SpotV3(SpotV3::BookTicker), None).await
  }

  /// Latest prices for MULTI symbol.
  pub async fn list_book_tickers_multi<S>(&self, symbols: S) -> Result<BookTickersMultiResponse>
  where
    S: Into<Vec<String>>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();

    let formatted = vec_to_string_array(symbols.into());

    parameters.insert("symbols".into(), formatted);
    let request = build_query(parameters);
    self
      .client
      .get(API::SpotV3(SpotV3::BookTicker), Some(request))
      .await
  }

  // -> Best price/qty on the order book for ONE symbol
  pub async fn fetch_book_ticker<S>(&self, symbol: S) -> Result<BookTickerResponse>
  where
    S: Into<String>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();
    parameters.insert("symbol".into(), symbol.into());
    let request = build_query(parameters);
    self
      .client
      .get(API::SpotV3(SpotV3::BookTicker), Some(request))
      .await
  }

  // 24hr ticker price change statistics
  pub async fn fetch_ticker_day_stats<S>(&self, symbol: S) -> Result<TickerDaySummaryResponse>
  where
    S: Into<String>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();
    parameters.insert("symbol".into(), symbol.into());
    let request = build_query(parameters);
    self
      .client
      .get(API::SpotV3(SpotV3::Ticker24hr), Some(request))
      .await
  }

  // 24hr ticker price change statistics for all symbols
  pub async fn list_all_tickers_day_stats(&self) -> Result<Vec<TickerDaySummaryResponse>> {
    self.client.get(API::SpotV3(SpotV3::Ticker24hr), None).await
  }
}
