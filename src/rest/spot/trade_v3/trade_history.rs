use super::responses::TradeRecordResponse;
use super::SpotTradeV3Manager;
use crate::rest::endpoints::{SpotV3, API};
use crate::rest::spot::trade_v3::requests::TradeHistoryRequest;
use crate::util::build_signed_query;
use anyhow::Result;

impl SpotTradeV3Manager {
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
    let mut req = TradeHistoryRequest::default();
    req.symbol = symbol.into();
    req.order_id = Some(order_id.into());

    match self.list_trades_history_custom(req).await {
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

  /// Read trade_v3 history starting from UNIX/timestamp (sec/ms from EPOCH)
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
    let mut req = TradeHistoryRequest::default();
    req.symbol = symbol.into();
    req.from_id = Some(from_id.into());

    self.list_trades_history_custom(req).await
  }

  /// Read Recent Last 24h Trade history
  ///
  /// LIMIT - 500 (default)
  pub async fn list_trades_history_recent<S>(&self, symbol: S) -> Result<Vec<TradeRecordResponse>>
  where
    S: Into<String>,
  {
    let mut req = TradeHistoryRequest::default();
    req.symbol = symbol.into();

    self.list_trades_history_custom(req).await
  }

  /// Read trade_v3 history starting from UNIX/timestamp (sec/ms from EPOCH)
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
    let mut req = TradeHistoryRequest::default();
    req.symbol = symbol.into();
    req.start_time = Some(start_time.into());

    self.list_trades_history_custom(req).await
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
    let mut req = TradeHistoryRequest::default();
    req.symbol = symbol.into();
    req.start_time = Some(start_time.into());
    req.end_time = Some(end_time.into());

    self.list_trades_history_custom(req).await
  }

  /// Read Trade history
  pub async fn list_trades_history_custom(
    &self,
    request: TradeHistoryRequest,
  ) -> Result<Vec<TradeRecordResponse>> {
    let request = build_signed_query(request.build_params_tree()?, self.recv_window)?;
    self
      .client
      .get_signed(API::SpotV3(SpotV3::MyTrades), Some(request))
      .await
  }
}
