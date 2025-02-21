use super::requests::PlaceOrderRequest;
use super::responses::*;
use super::SpotTradeV3Manager;
use crate::model::EmptyResponse;
use crate::rest::endpoints::{SpotV3, API};
use crate::util::build_signed_query;
use anyhow::Result;

// Custom order
impl SpotTradeV3Manager {
  /// Configures and submit your custom order
  async fn place_custom_order(
    &self,
    order_request: PlaceOrderRequest,
  ) -> Result<OrderCreatedResponse> {
    let order = order_request.build_params_tree();
    let request = build_signed_query(order, self.recv_window)?;

    self
      .client
      .post_signed(API::SpotV3(SpotV3::Order), request)
      .await
  }

  /// Configures and submit your custom order
  ///
  /// This order is sandboxed: it is validated, but not sent to the matching engine.
  async fn test_place_custom_order(
    &self,
    order_request: PlaceOrderRequest,
  ) -> Result<EmptyResponse> {
    let order = order_request.build_params_tree();
    let request = build_signed_query(order, self.recv_window)?;

    match self
      .client
      .post_signed::<EmptyResponse>(API::SpotV3(SpotV3::OrderTest), request)
      .await
    {
      Ok(_) => Ok(EmptyResponse {}),
      Err(e) => Err(e),
    }
  }
}
