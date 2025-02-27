use super::responses::*;
use super::SpotTradeV3Manager;
use crate::rest::endpoints::{SpotV3, API};
use crate::util::build_signed_query;
use anyhow::Result;
use std::collections::BTreeMap;

impl SpotTradeV3Manager {
  /// Get an order's Info
  pub async fn fetch_order_by_id<S, O>(&self, symbol: S, order_id: O) -> Result<OrderInfoResponse>
  where
    S: Into<String>,
    O: Into<u64>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();
    parameters.insert("symbol".into(), symbol.into());
    parameters.insert("orderId".into(), order_id.into().to_string());

    let request = build_signed_query(parameters, self.recv_window)?;
    self
      .client
      .get_signed(API::SpotV3(SpotV3::Order), Some(request))
      .await
  }

  /// All current open orders
  pub async fn list_all_open_orders(&self) -> Result<Vec<OrderInfoResponse>> {
    let parameters: BTreeMap<String, String> = BTreeMap::new();

    let request = build_signed_query(parameters, self.recv_window)?;
    self
      .client
      .get_signed(API::SpotV3(SpotV3::OpenOrders), Some(request))
      .await
  }

  /// Current open orders for ONE symbol
  pub async fn list_open_orders_by_symbol<S>(&self, symbol: S) -> Result<Vec<OrderInfoResponse>>
  where
    S: Into<String>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();
    parameters.insert("symbol".into(), symbol.into());

    let request = build_signed_query(parameters, self.recv_window)?;
    self
      .client
      .get_signed(API::SpotV3(SpotV3::OpenOrders), Some(request))
      .await
  }
}
