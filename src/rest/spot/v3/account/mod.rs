use crate::rest::inner_client::InnerClient;

pub mod responses;

pub mod account_balance;
pub mod account_trade_rate_limits;

#[derive(Clone)]
pub struct SpotAccountManagerV3 {
  pub(crate) client: InnerClient,
  pub(crate) recv_window: u64,
}
