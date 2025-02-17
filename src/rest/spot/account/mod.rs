use crate::rest::inner_client::InnerClient;

pub mod model;
pub mod order_builder;

pub mod account_balance;
pub mod account_order_cancel;
pub mod account_order_custom;
pub mod account_order_info;
pub mod account_order_limit;
pub mod account_order_market;
pub mod account_order_market_with_qty;
pub mod account_order_stop_limit;
pub mod account_sor_allocations_history;
pub mod account_stp_orders_info;
pub mod account_trade_history;
pub mod account_trade_rate_limits;

#[derive(Clone)]
pub struct Account {
  pub client: InnerClient,
  pub recv_window: u64,
}
