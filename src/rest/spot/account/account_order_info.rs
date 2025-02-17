use super::model::*;
use super::Account;
use crate::rest::endpoints::{Spot, API};
use crate::util::build_signed_request;
use anyhow::Result;
use std::collections::BTreeMap;

impl Account {
  /// All current open orders
  pub async fn list_all_open_orders(&self) -> Result<Vec<OrderResponse>> {
    let parameters: BTreeMap<String, String> = BTreeMap::new();

    let request = build_signed_request(parameters, self.recv_window)?;
    self
      .client
      .get_signed(API::Spot(Spot::OpenOrders), Some(request))
      .await
  }

  /// Current open orders for ONE symbol
  pub async fn list_open_orders_by_symbol<S>(&self, symbol: S) -> Result<Vec<OrderResponse>>
  where
    S: Into<String>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();
    parameters.insert("symbol".into(), symbol.into());

    let request = build_signed_request(parameters, self.recv_window)?;
    self
      .client
      .get_signed(API::Spot(Spot::OpenOrders), Some(request))
      .await
  }

  /// Check an order's status
  pub async fn fetch_order_status<S, O>(&self, symbol: S, order_id: O) -> Result<OrderResponse>
  where
    S: Into<String>,
    O: Into<u64>,
  {
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();
    parameters.insert("symbol".into(), symbol.into());
    parameters.insert("orderId".into(), order_id.into().to_string());

    let request = build_signed_request(parameters, self.recv_window)?;
    self
      .client
      .get_signed(API::Spot(Spot::Order), Some(request))
      .await
  }
}
