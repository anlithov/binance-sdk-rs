use super::model::AggregatedTradeResponse;
use super::Market;
use crate::rest::endpoints::{Spot, API};
use crate::util::{build_request, is_start_time_valid};
use anyhow::{bail, Result};
use std::collections::BTreeMap;

impl Market {
  /// Get compressed, aggregate Recent 24h trades.
  ///
  /// LIMIT 500
  pub async fn list_agg_trades_recent<S>(&self, symbol: S) -> Result<Vec<AggregatedTradeResponse>>
  where
    S: Into<String>,
  {
    self
      .list_agg_trades_custom(symbol, None, None, None, None)
      .await
  }

  /// Get compressed, aggregate Recent 24h trades.
  ///
  /// LIMIT 500
  pub async fn list_agg_trades_from_id<S, I>(
    &self,
    symbol: S,
    from_id: I,
  ) -> Result<Vec<AggregatedTradeResponse>>
  where
    S: Into<String>,
    I: Into<u64>,
  {
    self
      .list_agg_trades_custom(symbol, from_id.into(), None, None, None)
      .await
  }

  /// Get compressed, aggregate Recent 24h trades.
  ///
  /// LIMIT 500
  pub async fn list_agg_trades_from_time<S, STR>(
    &self,
    symbol: S,
    start_time: STR,
  ) -> Result<Vec<AggregatedTradeResponse>>
  where
    S: Into<String>,
    STR: Into<u64>,
  {
    self
      .list_agg_trades_custom(symbol, None, start_time.into(), None, None)
      .await
  }

  /// Get compressed, aggregate Recent 24h trades.
  ///
  /// LIMIT 500
  pub async fn list_agg_trades_by_timeframe<S, STR, END>(
    &self,
    symbol: S,
    start_time: STR,
    end_time: END,
  ) -> Result<Vec<AggregatedTradeResponse>>
  where
    S: Into<String>,
    STR: Into<u64>,
    END: Into<u64>,
  {
    self
      .list_agg_trades_custom(symbol, None, start_time.into(), end_time.into(), None)
      .await
  }

  /// Get compressed, aggregate trades. Trades that fill at the time, from the same order,
  /// with the same price will have the quantity aggregated.
  /// - If fromId, startTime, and endTime are not sent, the most recent aggregate trades will be returned.
  /// - Note that if a trade has the following values, this was a duplicate aggregate trade and marked as invalid:
  /// - p = '0' // price
  /// - q = '0' // qty
  /// - f = -1 // ﬁrst_trade_id
  /// - l = -1 // last_trade_id
  pub async fn list_agg_trades_custom<S, I, STR, END, L>(
    &self,
    symbol: S,
    from_id: I,
    start_time: STR,
    end_time: END,
    limit: L,
  ) -> Result<Vec<AggregatedTradeResponse>>
  where
    S: Into<String>,
    I: Into<Option<u64>>,
    STR: Into<Option<u64>>,
    END: Into<Option<u64>>,
    L: Into<Option<u16>>,
  {
    let start_time = start_time.into();
    let end_time = end_time.into();
    if start_time.is_some() && end_time.is_some() && (&start_time > &end_time) {
      bail!("End time should be greater than start time");
    }
    if let Some(st) = &start_time {
      if !is_start_time_valid(st) {
        bail!("Start time should be less than the current time");
      }
    }

    let mut parameters: BTreeMap<String, String> = BTreeMap::new();

    parameters.insert("symbol".into(), symbol.into());
    if let Some(lt) = limit.into() {
      parameters.insert("limit".into(), format!("{}", lt));
    }
    if let Some(st) = start_time {
      parameters.insert("startTime".into(), format!("{}", st));
    }
    if let Some(et) = end_time {
      parameters.insert("endTime".into(), format!("{}", et));
    }
    if let Some(fi) = from_id.into() {
      parameters.insert("fromId".into(), format!("{}", fi));
    }

    let request = build_request(parameters);

    self
      .client
      .get(API::Spot(Spot::AggTrades), Some(request))
      .await
  }
}
