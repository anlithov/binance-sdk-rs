use super::model::*;
pub use super::order_builder::TimeInForce;
use super::order_builder::*;
use super::Account;
use crate::model::EmptyResponse;
use crate::rest::endpoints::{Spot, API};
use crate::util::build_signed_request;
use anyhow::Result;

// Market orders with quote quantity
impl Account {
  /// Place a MARKET order with quote quantity - BUY
  pub async fn market_buy_with_quote_quantity<S, Q>(
    &self,
    symbol: S,
    quote_qty: Q,
  ) -> Result<OrderCreatedResponse>
  where
    S: Into<String>,
    Q: Into<f64>,
  {
    self
      .market_order_with_quote_quantity(symbol, quote_qty, OrderSide::Buy)
      .await
  }

  /// Place a test MARKET order with quote quantity - BUY
  ///
  /// This order is sandboxed: it is validated, but not sent to the matching engine.
  pub async fn test_market_buy_with_quote_quantity<S, Q>(
    &self,
    symbol: S,
    quote_qty: Q,
  ) -> Result<EmptyResponse>
  where
    S: Into<String>,
    Q: Into<f64>,
  {
    self
      .test_market_order_with_quote_quantity(symbol, quote_qty, OrderSide::Buy)
      .await
  }

  /// Place a market order - SELL
  pub async fn market_sell_with_quote_quantity<S, Q>(
    &self,
    symbol: S,
    quote_qty: Q,
  ) -> Result<OrderCreatedResponse>
  where
    S: Into<String>,
    Q: Into<f64>,
  {
    self
      .market_order_with_quote_quantity(symbol, quote_qty, OrderSide::Sell)
      .await
  }

  /// Place a market test order - SELL
  ///
  /// This order is sandboxed: it is validated, but not sent to the matching engine.
  pub async fn test_market_sell_with_quote_quantity<S, Q>(
    &self,
    symbol: S,
    quote_qty: Q,
  ) -> Result<EmptyResponse>
  where
    S: Into<String>,
    Q: Into<f64>,
  {
    self
      .test_market_order_with_quote_quantity(symbol, quote_qty, OrderSide::Sell)
      .await
  }

  async fn market_order_with_quote_quantity<S, Q>(
    &self,
    symbol: S,
    qty: Q,
    order_side: OrderSide,
  ) -> Result<OrderCreatedResponse>
  where
    S: Into<String>,
    Q: Into<f64>,
  {
    let buy = OrderQuoteQuantityRequest {
      symbol: symbol.into(),
      quote_order_qty: qty.into(),
      price: 0.0,
      order_side,
      order_type: OrderType::Market,
      time_in_force: TimeInForce::GTC,
      new_client_order_id: None,
    };
    let order = buy.build_quote();
    let request = build_signed_request(order, self.recv_window)?;

    self
      .client
      .post_signed(API::Spot(Spot::Order), request)
      .await
  }

  async fn test_market_order_with_quote_quantity<S, Q>(
    &self,
    symbol: S,
    qty: Q,
    order_side: OrderSide,
  ) -> Result<EmptyResponse>
  where
    S: Into<String>,
    Q: Into<f64>,
  {
    let buy = OrderQuoteQuantityRequest {
      symbol: symbol.into(),
      quote_order_qty: qty.into(),
      price: 0.0,
      order_side,
      order_type: OrderType::Market,
      time_in_force: TimeInForce::GTC,
      new_client_order_id: None,
    };
    let order = buy.build_quote();
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
