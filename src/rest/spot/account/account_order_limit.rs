use super::model::*;
pub use super::order_builder::TimeInForce;
use super::order_builder::*;
use super::Account;
use crate::model::EmptyResponse;
use crate::rest::endpoints::{Spot, API};
use crate::util::build_signed_request;
use anyhow::Result;

// Limit orders
impl Account {
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
      .place_limit_order(symbol, qty, OrderSide::Buy, price)
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
      .test_place_limit_order(symbol, qty, OrderSide::Buy, price)
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
      .place_limit_order(symbol, qty, OrderSide::Sell, price)
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
      .test_place_limit_order(symbol, qty, OrderSide::Sell, price)
      .await
  }

  async fn place_limit_order<S, Q, PR>(
    &self,
    symbol: S,
    qty: Q,
    order_side: OrderSide,
    price: PR,
  ) -> Result<OrderCreatedResponse>
  where
    S: Into<String>,
    Q: Into<f64>,
    PR: Into<f64>,
  {
    let request = OrderRequest {
      symbol: symbol.into(),
      qty: qty.into(),
      price: price.into(),
      stop_price: None,
      order_side,
      order_type: OrderType::Limit,
      time_in_force: TimeInForce::GTC,
      new_client_order_id: None,
    };
    let order = request.build_order();
    let request = build_signed_request(order, self.recv_window)?;

    self
      .client
      .post_signed(API::Spot(Spot::Order), request)
      .await
  }

  async fn test_place_limit_order<S, Q, PR>(
    &self,
    symbol: S,
    qty: Q,
    order_side: OrderSide,
    price: PR,
  ) -> Result<EmptyResponse>
  where
    S: Into<String>,
    Q: Into<f64>,
    PR: Into<f64>,
  {
    let request = OrderRequest {
      symbol: symbol.into(),
      qty: qty.into(),
      price: price.into(),
      stop_price: None,
      order_side,
      order_type: OrderType::Limit,
      time_in_force: TimeInForce::GTC,
      new_client_order_id: None,
    };
    let order = request.build_order();
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
