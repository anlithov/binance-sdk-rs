use super::SpotMarketV3Manager;
use crate::rest::endpoints::{SpotV3, API};
use crate::rest::spot::trade_v3::responses::TradeRecordResponse;
use crate::util::build_query;
use anyhow::Result;
use std::collections::BTreeMap;

impl SpotMarketV3Manager {
  /// Read Recent 24h Trade history
  /// This potentially can be faster than "trades_history_recent"
  /// This method reads from MEMORY, "trades_history_recent" - from DATABASE
  ///
  /// LIMIT - 500
  pub async fn list_trades_history_recent_from_memory<S>(
    &self,
    symbol: S,
  ) -> Result<Vec<TradeRecordResponse>>
  where
    S: Into<String>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();

    parameters.insert("symbol".into(), symbol.into());

    let request = build_query(parameters);
    self
      .client
      .get_signed(API::SpotV3(SpotV3::Trades), Some(request))
      .await
  }

  /// Read Recent 24h Trade history
  ///
  /// LIMIT - 500 (default)
  pub async fn list_trades_history_recent<S>(&self, symbol: S) -> Result<Vec<TradeRecordResponse>>
  where
    S: Into<String>,
  {
    self
      .list_trades_history_custom(symbol.into(), None, None)
      .await
  }

  /// Read Trade history from ID
  ///
  /// LIMIT - 500 (default)
  pub async fn list_trades_history_from_id<S, I>(
    &self,
    symbol: S,
    from_id: I,
  ) -> Result<Vec<TradeRecordResponse>>
  where
    S: Into<String>,
    I: Into<u64>,
  {
    self
      .list_trades_history_custom(symbol.into(), from_id.into(), None)
      .await
  }

  /// Read Trade history
  ///
  /// e.g. 1739401957098/1739401957
  /// LIMIT - up to 1000
  pub async fn list_trades_history_custom<S, I, L>(
    &self,
    symbol: S,
    from_id: I,
    limit: L,
  ) -> Result<Vec<TradeRecordResponse>>
  where
    S: Into<String>,
    I: Into<Option<u64>>,
    L: Into<Option<u16>>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();

    parameters.insert("symbol".into(), symbol.into());
    if let Some(lt) = limit.into() {
      parameters.insert("limit".into(), format!("{}", lt));
    }
    if let Some(fi) = from_id.into() {
      parameters.insert("fromId".into(), format!("{}", fi));
    }

    let request = build_query(parameters);
    self
      .client
      .get_signed(API::SpotV3(SpotV3::HistoricalTrades), Some(request))
      .await
  }
}
