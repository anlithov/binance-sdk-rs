use super::enums::{OrderSide, OrderType};
use super::requests::*;
use super::responses::*;
use super::SpotTradeV3Manager;
use crate::model::EmptyResponse;
use crate::rest::endpoints::{SpotV3, API};
use crate::util::build_signed_query;
use anyhow::Result;

// Market orders with quote quantity
impl SpotTradeV3Manager {
  /// Place a MARKET order with quote quantity - BUY
  pub async fn place_market_buy_order_with_quote_quantity<S, Q>(
    &self,
    symbol: S,
    quote_qty: Q,
  ) -> Result<OrderCreatedResponse>
  where
    S: Into<String>,
    Q: Into<f64>,
  {
    self
      .place_market_order_with_quote_quantity(symbol, OrderSide::Buy, quote_qty)
      .await
  }

  /// Place a test MARKET order with quote quantity - BUY
  ///
  /// This order is sandboxed: it is validated, but not sent to the matching engine.
  pub async fn test_place_market_buy_order_with_quote_quantity<S, Q>(
    &self,
    symbol: S,
    quote_qty: Q,
  ) -> Result<EmptyResponse>
  where
    S: Into<String>,
    Q: Into<f64>,
  {
    self
      .test_place_market_order_with_quote_quantity(symbol, OrderSide::Buy, quote_qty)
      .await
  }

  /// Place a MARKET order with quote quantity - SELL
  pub async fn place_market_sell_order_with_quote_quantity<S, Q>(
    &self,
    symbol: S,
    quote_qty: Q,
  ) -> Result<OrderCreatedResponse>
  where
    S: Into<String>,
    Q: Into<f64>,
  {
    self
      .place_market_order_with_quote_quantity(symbol, OrderSide::Sell, quote_qty)
      .await
  }

  /// Place a MARKET order with quote quantity - SELL
  ///
  /// This order is sandboxed: it is validated, but not sent to the matching engine.
  pub async fn test_place_market_sell_order_with_quote_quantity<S, Q>(
    &self,
    symbol: S,
    quote_qty: Q,
  ) -> Result<EmptyResponse>
  where
    S: Into<String>,
    Q: Into<f64>,
  {
    self
      .test_place_market_order_with_quote_quantity(symbol, OrderSide::Sell, quote_qty)
      .await
  }

  async fn place_market_order_with_quote_quantity<S, Q>(
    &self,
    symbol: S,
    order_side: OrderSide,
    quote_qty: Q,
  ) -> Result<OrderCreatedResponse>
  where
    S: Into<String>,
    Q: Into<f64>,
  {
    let mut request = PlaceOrderRequest::default();

    request.symbol = symbol.into();
    request.quote_order_qty = Some(quote_qty.into());
    request.order_side = order_side;
    request.order_type = OrderType::Market;

    let params_tree = request.build_params_tree();
    let query = build_signed_query(params_tree, self.recv_window)?;

    self
      .client
      .post_signed(API::SpotV3(SpotV3::Order), query)
      .await
  }

  async fn test_place_market_order_with_quote_quantity<S, Q>(
    &self,
    symbol: S,
    order_side: OrderSide,
    quote_qty: Q,
  ) -> Result<EmptyResponse>
  where
    S: Into<String>,
    Q: Into<f64>,
  {
    let mut request = PlaceOrderRequest::default();

    request.symbol = symbol.into();
    request.quote_order_qty = Some(quote_qty.into());
    request.order_side = order_side;
    request.order_type = OrderType::Market;

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
