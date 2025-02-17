use super::model::*;
pub use super::order_builder::TimeInForce;
use super::order_builder::*;
use super::Account;
use crate::model::EmptyResponse;
use crate::rest::endpoints::{Spot, API};
use crate::util::build_signed_request;
use anyhow::Result;

impl Account {
  /// Create a stop limit buy order for the given symbol, price and stop price.
  /// Returning a `Transaction` value with the same parameters sent on the order.
  pub async fn place_stop_limit_buy_order<S, F, PR, SPR>(
    &self,
    symbol: S,
    qty: F,
    price: PR,
    stop_price: SPR,
    time_in_force: TimeInForce,
  ) -> Result<OrderCreatedResponse>
  where
    S: Into<String>,
    F: Into<f64>,
    PR: Into<f64>,
    SPR: Into<f64>,
  {
    self
      .place_stop_limit_order(
        symbol,
        qty,
        price,
        stop_price,
        time_in_force,
        OrderSide::Sell,
      )
      .await
  }

  /// Create a stop limit buy test order for the given symbol, price and stop price.
  /// Returning a `Transaction` value with the same parameters sent on the order.
  ///
  /// This order is sandboxed: it is validated, but not sent to the matching engine.
  pub async fn test_здфсу_stop_limit_buy_order<S, F, PR, SPR>(
    &self,
    symbol: S,
    qty: F,
    price: PR,
    stop_price: SPR,
    time_in_force: TimeInForce,
  ) -> Result<EmptyResponse>
  where
    S: Into<String>,
    F: Into<f64>,
    PR: Into<f64>,
    SPR: Into<f64>,
  {
    self
      .test_place_stop_limit_order(
        symbol,
        qty,
        price,
        stop_price,
        time_in_force,
        OrderSide::Buy,
      )
      .await
  }

  /// Create a stop limit sell order for the given symbol, price and stop price.
  /// Returning a `Transaction` value with the same parameters sent on the order.
  pub async fn place_stop_limit_sell_order<S, F, PR, SPR>(
    &self,
    symbol: S,
    qty: F,
    price: PR,
    stop_price: SPR,
    time_in_force: TimeInForce,
  ) -> Result<OrderCreatedResponse>
  where
    S: Into<String>,
    F: Into<f64>,
    PR: Into<f64>,
    SPR: Into<f64>,
  {
    self
      .place_stop_limit_order(
        symbol,
        qty,
        price,
        stop_price,
        time_in_force,
        OrderSide::Sell,
      )
      .await
  }

  /// Create a stop limit sell order for the given symbol, price and stop price.
  /// Returning a `Transaction` value with the same parameters sent on the order.
  ///
  /// This order is sandboxed: it is validated, but not sent to the matching engine.
  pub async fn test_place_stop_limit_sell_order<S, F, PR, SPR>(
    &self,
    symbol: S,
    qty: F,
    price: PR,
    stop_price: SPR,
    time_in_force: TimeInForce,
  ) -> Result<EmptyResponse>
  where
    S: Into<String>,
    F: Into<f64>,
    PR: Into<f64>,
    SPR: Into<f64>,
  {
    self
      .test_place_stop_limit_order(
        symbol,
        qty,
        price,
        stop_price,
        time_in_force,
        OrderSide::Sell,
      )
      .await
  }

  /// Create a stop limit sell order for the given symbol, price and stop price.
  /// Returning a `Transaction` value with the same parameters sent on the order.
  ///
  /// This order is sandboxed: it is validated, but not sent to the matching engine.
  async fn place_stop_limit_order<S, Q, PR, SPR>(
    &self,
    symbol: S,
    qty: Q,
    price: PR,
    stop_price: SPR,
    time_in_force: TimeInForce,
    order_side: OrderSide,
  ) -> Result<OrderCreatedResponse>
  where
    S: Into<String>,
    Q: Into<f64>,
    PR: Into<f64>,
    SPR: Into<f64>,
  {
    let req = OrderRequest {
      symbol: symbol.into(),
      qty: qty.into(),
      price: price.into(),
      stop_price: Some(stop_price.into()),
      order_side,
      order_type: OrderType::StopLossLimit,
      time_in_force,
      new_client_order_id: None,
    };

    let order = req.build_order();
    let request = build_signed_request(order, self.recv_window)?;

    self
      .client
      .post_signed(API::Spot(Spot::Order), request)
      .await
  }

  /// Create a stop limit sell order for the given symbol, price and stop price.
  /// Returning a `Transaction` value with the same parameters sent on the order.
  ///
  /// This order is sandboxed: it is validated, but not sent to the matching engine.
  async fn test_place_stop_limit_order<S, Q, PR, SPR>(
    &self,
    symbol: S,
    qty: Q,
    price: PR,
    stop_price: SPR,
    time_in_force: TimeInForce,
    order_side: OrderSide,
  ) -> Result<EmptyResponse>
  where
    S: Into<String>,
    Q: Into<f64>,
    PR: Into<f64>,
    SPR: Into<f64>,
  {
    let req = OrderRequest {
      symbol: symbol.into(),
      qty: qty.into(),
      price: price.into(),
      stop_price: Some(stop_price.into()),
      order_side,
      order_type: OrderType::StopLossLimit,
      time_in_force,
      new_client_order_id: None,
    };

    let order = req.build_order();
    let request = build_signed_request(order, self.recv_window)?;

    match self
      .client
      .post_signed::<EmptyResponse>(API::Spot(Spot::OrderTest), request)
      .await
    {
      Ok(_) => Ok(EmptyResponse {}),
      Err(e) => Err(e),
    }
  }
}
