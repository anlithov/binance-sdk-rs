use crate::rest::core::inner_client::InnerClient;

pub mod enums;
pub mod market_depth;
pub mod market_instrument_info;
pub mod market_klines;
pub mod market_ticker_info;
pub mod market_ticker_price;
pub mod market_trades_history;
pub mod market_trades_history_agg;
pub mod requests;
pub mod responses;

#[derive(Clone)]
pub struct SpotMarketV3Manager {
  pub(crate) client: InnerClient,
  pub(crate) recv_window: u64,
}
