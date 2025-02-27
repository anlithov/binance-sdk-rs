use super::enums::{OrderSide, OrderType, TimeInForce};
use super::requests::*;
use super::responses::*;
use super::SpotTradeV3Manager;
use crate::model::EmptyResponse;
use crate::rest::endpoints::{SpotV3, API};
use crate::util::build_signed_query;
use anyhow::Result;

impl SpotTradeV3Manager {
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
        OrderSide::Sell,
        qty,
        price,
        stop_price,
        time_in_force,
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
        OrderSide::Buy,
        qty,
        price,
        stop_price,
        time_in_force,
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
        OrderSide::Sell,
        qty,
        price,
        stop_price,
        time_in_force,
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
        OrderSide::Sell,
        qty,
        price,
        stop_price,
        time_in_force,
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
    order_side: OrderSide,
    qty: Q,
    price: PR,
    stop_price: SPR,
    time_in_force: TimeInForce,
  ) -> Result<OrderCreatedResponse>
  where
    S: Into<String>,
    Q: Into<f64>,
    PR: Into<f64>,
    SPR: Into<f64>,
  {
    let mut request = PlaceOrderRequest::default();

    request.symbol = symbol.into();
    request.qty = Some(qty.into());
    request.price = Some(price.into());
    request.stop_price = Some(stop_price.into());
    request.order_side = order_side;
    request.order_type = OrderType::StopLossLimit;
    request.time_in_force = Some(time_in_force);

    let params_tree = request.build_params_tree();
    let query = build_signed_query(params_tree, self.recv_window)?;

    self
      .client
      .post_signed(API::SpotV3(SpotV3::Order), query)
      .await
  }

  /// Create a stop limit sell order for the given symbol, price and stop price.
  /// Returning a `Transaction` value with the same parameters sent on the order.
  ///
  /// This order is sandboxed: it is validated, but not sent to the matching engine.
  async fn test_place_stop_limit_order<S, Q, PR, SPR>(
    &self,
    symbol: S,
    order_side: OrderSide,
    qty: Q,
    price: PR,
    stop_price: SPR,
    time_in_force: TimeInForce,
  ) -> Result<EmptyResponse>
  where
    S: Into<String>,
    Q: Into<f64>,
    PR: Into<f64>,
    SPR: Into<f64>,
  {
    let mut request = PlaceOrderRequest::default();

    request.symbol = symbol.into();
    request.qty = Some(qty.into());
    request.price = Some(price.into());
    request.stop_price = Some(stop_price.into());
    request.order_side = order_side;
    request.order_type = OrderType::StopLossLimit;
    request.time_in_force = Some(time_in_force);

    let params_tree = request.build_params_tree();
    let query = build_signed_query(params_tree, self.recv_window)?;

    match self
      .client
      .post_signed::<EmptyResponse>(API::SpotV3(SpotV3::OrderTest), query)
      .await
    {
      Ok(_) => Ok(EmptyResponse {}),
      Err(e) => Err(e),
    }
  }
}
