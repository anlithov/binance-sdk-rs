use super::model::*;
pub use super::order_builder::TimeInForce;
use super::Account;
use crate::model::EmptyResponse;
use crate::rest::endpoints::{Spot, API};
use crate::rest::spot::account::order_builder::OrderCustomRequest;
use crate::util::build_signed_request;
use anyhow::Result;

// Custom order
impl Account {
  /// Configures and submit your custom order
  async fn place_custom_order(
    &self,
    order_request: OrderCustomRequest,
  ) -> Result<OrderCreatedResponse> {
    let order = order_request.build_order();
    let request = build_signed_request(order, self.recv_window)?;

    self
      .client
      .post_signed(API::Spot(Spot::Order), request)
      .await
  }

  /// Configures and submit your custom order
  ///
  /// This order is sandboxed: it is validated, but not sent to the matching engine.
  async fn test_place_custom_order(
    &self,
    order_request: OrderCustomRequest,
  ) -> Result<EmptyResponse> {
    let order = order_request.build_order();
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
