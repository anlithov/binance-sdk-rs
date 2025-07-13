use crate::rest::core::inner_client::InnerClient;

pub mod enums;
pub mod responses;

pub mod requests;
pub mod trade_history;
pub mod trade_order_cancel;
pub mod trade_order_custom;
pub mod trade_order_info;
pub mod trade_order_limit;
pub mod trade_order_market;
pub mod trade_order_market_with_quote;
pub mod trade_order_stop_limit;
pub mod trade_sor_allocations_history;
pub mod trade_stp_orders_info;

#[derive(Clone)]
pub struct SpotTradeV3Manager {
  pub(crate) client: InnerClient,
  pub(crate) recv_window: u64,
}
