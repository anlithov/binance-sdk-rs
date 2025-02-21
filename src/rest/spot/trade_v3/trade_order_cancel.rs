use super::responses::*;
use super::SpotTradeV3Manager;
use crate::rest::endpoints::{SpotV3, API};
use crate::util::build_signed_query;
use anyhow::Result;
use std::collections::BTreeMap;

// Limit orders
impl SpotTradeV3Manager {
  /// Cancel order with order id
  pub async fn cancel_order_by_id<S, O>(
    &self,
    symbol: S,
    order_id: O,
  ) -> Result<OrderCanceledResponse>
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
      .delete_signed(API::SpotV3(SpotV3::Order), Some(request))
      .await
  }

  /// Cancel order with client id
  pub async fn cancel_order_by_client_id<S, OI>(
    &self,
    symbol: S,
    client_order_id: S,
  ) -> Result<OrderCanceledResponse>
  where
    S: Into<String>,
    OI: Into<String>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();
    parameters.insert("symbol".into(), symbol.into());
    parameters.insert("origClientOrderId".into(), client_order_id.into());

    let request = build_signed_query(parameters, self.recv_window)?;
    self
      .client
      .delete_signed(API::SpotV3(SpotV3::Order), Some(request))
      .await
  }

  /// Cancel all open orders for a single symbol
  pub async fn cancel_all_open_orders_by_symbol<S>(
    &self,
    symbol: S,
  ) -> Result<Vec<OrderOrTriggerCanceledResponse>>
  where
    S: Into<String>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();
    parameters.insert("symbol".into(), symbol.into());
    let request = build_signed_query(parameters, self.recv_window)?;
    self
      .client
      .delete_signed(API::SpotV3(SpotV3::OpenOrders), Some(request))
      .await
  }
}
