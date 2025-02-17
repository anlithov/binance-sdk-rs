use super::Market;
use crate::rest::endpoints::{Spot, API};
use crate::rest::spot::market::model::{
  BookTickerResponse, BookTickersMultiResponse, TickerDaySummaryResponse,
};
use crate::util::{build_request, vec_to_string};
use anyhow::Result;
use std::collections::BTreeMap;

impl Market {
  // Symbols order book ticker
  // -> Best price/qty on the order book for ALL symbols.
  pub async fn list_book_tickers_all(&self) -> Result<BookTickersMultiResponse> {
    self.client.get(API::Spot(Spot::BookTicker), None).await
  }

  /// Latest prices for MULTI symbol.
  pub async fn list_book_tickers_multi<S>(&self, symbols: S) -> Result<BookTickersMultiResponse>
  where
    S: Into<Vec<String>>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();

    let formatted = vec_to_string(symbols.into());

    parameters.insert("symbols".into(), formatted);
    let request = build_request(parameters);
    self
      .client
      .get(API::Spot(Spot::BookTicker), Some(request))
      .await
  }

  // -> Best price/qty on the order book for ONE symbol
  pub async fn fetch_book_ticker<S>(&self, symbol: S) -> Result<BookTickerResponse>
  where
    S: Into<String>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();
    parameters.insert("symbol".into(), symbol.into());
    let request = build_request(parameters);
    self
      .client
      .get(API::Spot(Spot::BookTicker), Some(request))
      .await
  }

  // 24hr ticker price change statistics
  pub async fn fetch_ticker_day_summary<S>(&self, symbol: S) -> Result<TickerDaySummaryResponse>
  where
    S: Into<String>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();
    parameters.insert("symbol".into(), symbol.into());
    let request = build_request(parameters);
    self
      .client
      .get(API::Spot(Spot::Ticker24hr), Some(request))
      .await
  }

  // 24hr ticker price change statistics for all symbols
  pub async fn list_ticker_day_summary_all(&self) -> Result<Vec<TickerDaySummaryResponse>> {
    self.client.get(API::Spot(Spot::Ticker24hr), None).await
  }
}
