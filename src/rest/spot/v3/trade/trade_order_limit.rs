use super::enums::{OrderSide, OrderType, TimeInForce};
use super::requests::*;
use super::responses::*;
use super::SpotTradeV3Manager;
use crate::model::EmptyResponse;
use crate::rest::endpoints::{SpotV3, API};
use crate::util::build_signed_query;
use anyhow::Result;

// Limit orders
impl SpotTradeV3Manager {
  /// Place a LIMIT order - BUY
  pub async fn place_limit_buy_order<S, Q, PR>(
    &self,
    symbol: S,
    qty: Q,
    price: PR,
  ) -> Result<OrderCreatedResponse>
  where
    S: Into<String>,
    Q: Into<f64>,
    PR: Into<f64>,
  {
    self
      .place_limit_order(symbol, OrderSide::Buy, qty, price)
      .await
  }

  /// Place a LIMIT test order - BUY
  ///
  /// This order is sandboxed: it is validated, but not sent to the matching engine.
  pub async fn test_place_limit_buy_order<S, Q, PR>(
    &self,
    symbol: S,
    qty: Q,
    price: PR,
  ) -> Result<EmptyResponse>
  where
    S: Into<String>,
    Q: Into<f64>,
    PR: Into<f64>,
  {
    self
      .test_place_limit_order(symbol, OrderSide::Buy, qty, price)
      .await
  }

  /// Place a LIMIT order - SELL
  pub async fn place_limit_sell_order<S, Q, PR>(
    &self,
    symbol: S,
    qty: Q,
    price: PR,
  ) -> Result<OrderCreatedResponse>
  where
    S: Into<String>,
    Q: Into<f64>,
    PR: Into<f64>,
  {
    self
      .place_limit_order(symbol, OrderSide::Sell, qty, price)
      .await
  }

  /// Place a LIMIT test order - SELL
  ///
  /// This order is sandboxed: it is validated, but not sent to the matching engine.
  pub async fn test_place_limit_sell_order<S, Q, PR>(
    &self,
    symbol: S,
    qty: Q,
    price: PR,
  ) -> Result<EmptyResponse>
  where
    S: Into<String>,
    Q: Into<f64>,
    PR: Into<f64>,
  {
    self
      .test_place_limit_order(symbol, OrderSide::Sell, qty, price)
      .await
  }

  pub async fn place_limit_order<S, Q, PR>(
    &self,
    symbol: S,
    order_side: OrderSide,
    qty: Q,
    price: PR,
  ) -> Result<OrderCreatedResponse>
  where
    S: Into<String>,
    Q: Into<f64>,
    PR: Into<f64>,
  {
    let mut request = PlaceOrderRequest::default();

    request.symbol = symbol.into();
    request.qty = Some(qty.into());
    request.price = Some(price.into());
    request.stop_price = None;
    request.order_side = order_side;
    request.order_type = OrderType::Limit;
    request.time_in_force = Some(TimeInForce::GTC);
    request.new_client_order_id = None;

    let params_tree = request.build_params_tree();
    let query = build_signed_query(params_tree, self.recv_window)?;

    self
      .client
      .post_signed(API::SpotV3(SpotV3::Order), query)
      .await
  }

  async fn test_place_limit_order<S, Q, PR>(
    &self,
    symbol: S,
    order_side: OrderSide,
    qty: Q,
    price: PR,
  ) -> Result<EmptyResponse>
  where
    S: Into<String>,
    Q: Into<f64>,
    PR: Into<f64>,
  {
    let mut request = PlaceOrderRequest::default();

    request.symbol = symbol.into();
    request.qty = Some(qty.into());
    request.price = Some(price.into());
    request.stop_price = None;
    request.order_side = order_side;
    request.order_type = OrderType::Limit;
    request.time_in_force = Some(TimeInForce::GTC);

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
