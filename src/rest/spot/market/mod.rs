use crate::rest::inner_client::InnerClientRest;
use std::fmt::Display;

pub mod market_depth;
pub mod market_klines;
pub mod market_ticker_info;
pub mod market_ticker_price;
pub mod market_trades_history;
pub mod market_trades_history_agg;
pub mod model;

#[derive(Clone)]
pub struct Market {
  pub client: InnerClientRest,
  pub recv_window: u64,
}

pub enum KlineInterval {
  Sec1,
  Min1,
  Min3,
  Min5,
  Min15,
  Min30,
  Hour1,
  Hour2,
  Hour4,
  Hour6,
  Hour8,
  Hour12,
  Day1,
  Day3,
  Week1,
  Month1,
}

impl KlineInterval {
  pub fn from_int(value: i32) -> Option<Self> {
    match value {
      1 => Some(KlineInterval::Sec1),
      2 => Some(KlineInterval::Min1),
      3 => Some(KlineInterval::Min3),
      4 => Some(KlineInterval::Min5),
      5 => Some(KlineInterval::Min15),
      6 => Some(KlineInterval::Min30),
      7 => Some(KlineInterval::Hour1),
      8 => Some(KlineInterval::Hour2),
      9 => Some(KlineInterval::Hour4),
      10 => Some(KlineInterval::Hour6),
      11 => Some(KlineInterval::Hour8),
      12 => Some(KlineInterval::Hour12),
      13 => Some(KlineInterval::Day1),
      14 => Some(KlineInterval::Day3),
      15 => Some(KlineInterval::Week1),
      16 => Some(KlineInterval::Month1),
      _ => None,
    }
  }
}

impl Display for KlineInterval {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Sec1 => write!(f, "1s"),
      Self::Min1 => write!(f, "1m"),
      Self::Min3 => write!(f, "3m"),
      Self::Min5 => write!(f, "5m"),
      Self::Min15 => write!(f, "15m"),
      Self::Min30 => write!(f, "30m"),
      Self::Hour1 => write!(f, "1h"),
      Self::Hour2 => write!(f, "2h"),
      Self::Hour4 => write!(f, "4h"),
      Self::Hour6 => write!(f, "6h"),
      Self::Hour8 => write!(f, "8h"),
      Self::Hour12 => write!(f, "12h"),
      Self::Day1 => write!(f, "1d"),
      Self::Day3 => write!(f, "3d"),
      Self::Week1 => write!(f, "1w"),
      Self::Month1 => write!(f, "1M"),
    }
  }
}
