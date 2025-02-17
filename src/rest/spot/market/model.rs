use crate::rest::serde_helpers::string_or_float;
use anyhow::{bail, Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookResponse {
  pub last_update_id: u64,
  pub bids: Vec<BidResponse>,
  pub asks: Vec<AskResponse>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct BidResponse {
  #[serde(with = "string_or_float")]
  pub price: f64,
  #[serde(with = "string_or_float")]
  pub qty: f64,
}

impl BidResponse {
  pub fn new(price: f64, qty: f64) -> BidResponse {
    BidResponse { price, qty }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AskResponse {
  #[serde(with = "string_or_float")]
  pub price: f64,
  #[serde(with = "string_or_float")]
  pub qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AggregatedTradeResponse {
  #[serde(rename = "T")]
  pub time: u64,
  #[serde(rename = "a")]
  pub agg_id: u64,
  #[serde(rename = "f")]
  pub first_id: u64,
  #[serde(rename = "l")]
  pub last_id: u64,
  #[serde(rename = "m")]
  pub maker: bool,
  #[serde(rename = "M")]
  pub best_match: bool,
  #[serde(rename = "p", with = "string_or_float")]
  pub price: f64,
  #[serde(rename = "q", with = "string_or_float")]
  pub qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum KlineSummariesResponse {
  AllKlineSummaries(Vec<KlineSummaryResponse>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KlineSummaryResponse {
  pub open_time: u64,
  pub open: String,
  pub high: String,
  pub low: String,
  pub close: String,
  pub volume: String,
  pub close_time: u64,
  pub quote_asset_volume: String,
  pub number_of_trades: i64,
  pub taker_buy_base_asset_volume: String,
  pub taker_buy_quote_asset_volume: String,
}

fn get_value(row: &[Value], index: usize, name: &'static str) -> Result<Value> {
  match row.get(index) {
    Some(re) => Ok(re.clone()),
    None => bail!("{} {}", index, name),
  }
}

impl TryFrom<&Vec<Value>> for KlineSummaryResponse {
  type Error = Error;

  fn try_from(row: &Vec<Value>) -> Result<Self> {
    Ok(Self {
      open_time: from_value(get_value(row, 0, "open_time")?)?,
      open: from_value(get_value(row, 1, "open")?)?,
      high: from_value(get_value(row, 2, "high")?)?,
      low: from_value(get_value(row, 3, "low")?)?,
      close: from_value(get_value(row, 4, "close")?)?,
      volume: from_value(get_value(row, 5, "volume")?)?,
      close_time: from_value(get_value(row, 6, "close_time")?)?,
      quote_asset_volume: from_value(get_value(row, 7, "quote_asset_volume")?)?,
      number_of_trades: from_value(get_value(row, 8, "number_of_trades")?)?,
      taker_buy_base_asset_volume: from_value(get_value(row, 9, "taker_buy_base_asset_volume")?)?,
      taker_buy_quote_asset_volume: from_value(get_value(
        row,
        10,
        "taker_buy_quote_asset_volume",
      )?)?,
    })
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TickerPriceResponse {
  pub symbol: String,
  #[serde(with = "string_or_float")]
  pub price: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AveragePrice {
  pub mins: u64,
  #[serde(with = "string_or_float")]
  pub price: f64,
  pub close_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum BookTickersMultiResponse {
  AllBookTickers(Vec<BookTickerResponse>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BookTickerResponse {
  pub symbol: String,
  #[serde(with = "string_or_float")]
  pub bid_price: f64,
  #[serde(with = "string_or_float")]
  pub bid_qty: f64,
  #[serde(with = "string_or_float")]
  pub ask_price: f64,
  #[serde(with = "string_or_float")]
  pub ask_qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TickerDaySummaryResponse {
  pub symbol: String,
  pub price_change: String,
  pub price_change_percent: String,
  pub weighted_avg_price: String,
  #[serde(with = "string_or_float")]
  pub prev_close_price: f64,
  #[serde(with = "string_or_float")]
  pub last_price: f64,
  #[serde(with = "string_or_float")]
  pub bid_price: f64,
  #[serde(with = "string_or_float")]
  pub ask_price: f64,
  #[serde(with = "string_or_float")]
  pub open_price: f64,
  #[serde(with = "string_or_float")]
  pub high_price: f64,
  #[serde(with = "string_or_float")]
  pub low_price: f64,
  #[serde(with = "string_or_float")]
  pub volume: f64,
  pub open_time: u64,
  pub close_time: u64,
  pub first_id: i64,
  pub last_id: i64,
  pub count: u64,
}
