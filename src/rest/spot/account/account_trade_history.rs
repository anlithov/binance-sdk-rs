use super::model::TradeRecordResponse;
use super::Account;
use crate::rest::endpoints::{Spot, API};
use crate::util::{build_signed_request, is_start_time_valid};
use anyhow::{bail, Result};
use std::collections::BTreeMap;

impl Account {
  /// Read One order from history by "symbol" and "id"
  pub async fn fetch_trade_record_by_order_id<S, O>(
    &self,
    symbol: S,
    order_id: O,
  ) -> Result<Option<TradeRecordResponse>>
  where
    S: Into<String>,
    O: Into<u64>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();
    parameters.insert("symbol".into(), symbol.into());
    parameters.insert("orderId".into(), order_id.into().to_string());

    let request = build_signed_request(parameters, self.recv_window)?;
    match self
      .client
      .get_signed::<Vec<TradeRecordResponse>>(API::Spot(Spot::MyTrades), Some(request))
      .await
    {
      Ok(res) => {
        if res.is_empty() {
          Ok(None)
        } else {
          Ok(Some(res[0].clone()))
        }
      }
      Err(e) => Err(e),
    }
  }

  /// Read trade history starting from UNIX/timestamp (sec/ms from EPOCH)
  /// Get trades >= that from_id
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
      .list_trades_history_custom(symbol.into(), from_id.into(), None, None, None, None)
      .await
  }

  /// Read Recent Last 24h Trade history
  ///
  /// LIMIT - 500 (default)
  pub async fn list_trades_history_recent<S>(&self, symbol: S) -> Result<Vec<TradeRecordResponse>>
  where
    S: Into<String>,
  {
    self
      .list_trades_history_custom(symbol.into(), None, None, None, None, None)
      .await
  }

  /// Read trade history starting from UNIX/timestamp (sec/ms from EPOCH)
  ///
  /// e.g. 1739401957098/1739401957
  /// LIMIT - 500 (default)
  pub async fn list_trades_history_from_time<S, STR>(
    &self,
    symbol: S,
    start_time: STR,
  ) -> Result<Vec<TradeRecordResponse>>
  where
    S: Into<String>,
    STR: Into<u64>,
  {
    self
      .list_trades_history_custom(symbol.into(), None, None, start_time.into(), None, None)
      .await
  }

  /// Read Trade history starting by timeframe
  /// between "start_time" and "end_time" UNIX/timestamp (sec/ms from EPOCH)
  ///
  /// e.g. 1739401957098/1739401957
  /// LIMIT - 500 (default)
  pub async fn list_trades_history_by_timeframe<S, STR, END>(
    &self,
    symbol: S,
    start_time: STR,
    end_time: END,
  ) -> Result<Vec<TradeRecordResponse>>
  where
    S: Into<String>,
    STR: Into<u64>,
    END: Into<u64>,
  {
    self
      .list_trades_history_custom(
        symbol.into(),
        None,
        None,
        start_time.into(),
        end_time.into(),
        None,
      )
      .await
  }

  /// Read Trade history
  pub async fn list_trades_history_custom<S, O, I, STR, END, L>(
    &self,
    symbol: S,
    order_id: O,
    from_id: I,
    start_time: STR,
    end_time: END,
    limit: L,
  ) -> Result<Vec<TradeRecordResponse>>
  where
    S: Into<String>,
    O: Into<Option<u64>>,
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
    if let Some(ord) = order_id.into() {
      parameters.insert("orderId".into(), format!("{}", ord));
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

    let request = build_signed_request(parameters, self.recv_window)?;
    self
      .client
      .get_signed(API::Spot(Spot::MyTrades), Some(request))
      .await
  }
}
